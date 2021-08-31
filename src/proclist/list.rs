use crate::resource::pstat::pid_t;
use crate::resource::tty::init_tty_drivers;
use crate::resource::{cmdline, cpu, loadavg, process, procmem, pstat, stat, tty, uptime as up};
use crate::util::clamp;
use std::collections::HashMap;
use std::fs;
use sysconf;

#[derive(Debug)]
pub struct ProcList {
  pub plist: HashMap<pid_t, process::Process>, // XXX should be private
  pub tty_drivers: Vec<tty::TtyDriver>,
  pub cpus: Vec<cpu::CPU>,
  pub loadaverage: loadavg::LoadAvg,
  pub kernel_threads: u32,
  pub userland_threads: u32,
  pub total_tasks: u32,
  pub btime: i64,
  pub jiffy: i64,          // 1Hz = `@jiffy` jiffies // [sec]. now 100
  pub average_period: f64, // average period since last update of process
  pub uptime: up::Uptime,
}

impl ProcList {
  pub fn new() -> Self {
    let plist = HashMap::new();
    let mut tty_drivers = vec![];
    let cpus = cpu::init_cpus();
    init_tty_drivers(&mut tty_drivers);
    let btime = stat::get_btime();
    let jiffy = sysconf::sysconf(sysconf::SysconfVariable::ScClkTck).unwrap() as i64;
    let loadaverage = loadavg::LoadAvg::new();
    let uptime = up::Uptime::new();

    Self {
      plist,
      cpus,
      loadaverage,
      tty_drivers,
      kernel_threads: 0,
      userland_threads: 0,
      total_tasks: 0,
      btime,
      jiffy,
      average_period: 0.0,
      uptime,
    }
  }

  // update CPUs and return average period.
  pub fn update_cpus(&mut self) -> f64 {
    let mut sum_period = 0;
    for i in 0..self.cpus.len() {
      self.cpus[i].update_time_and_period();
      sum_period += self.cpus[i].totaltime_period;
    }

    sum_period as f64 / self.cpus.len() as f64
  }

  pub fn recurse_proc_tree(&mut self, ppid: Option<pid_t>, _dname: &str, average_period: f64) {
    let dname = if _dname.chars().nth(_dname.len() - 1).unwrap() == '/' {
      &_dname[0..(_dname.len() - 1)]
    } else {
      _dname
    };
    let proc_dir = match fs::read_dir(dname) {
      Ok(_dir) => _dir,
      Err(_) => return,
    };

    for p in proc_dir.into_iter() {
      // check if the entry is proc dir.
      let ent = p.unwrap();
      let meta = match ent.metadata() {
        Ok(_meta) => _meta,
        Err(_) => continue,
      };
      let name = ent.file_name().into_string().unwrap();
      if !meta.is_dir() || !name.chars().nth(0).unwrap().is_numeric() {
        continue;
      }

      // newly add the process into tree if not exists.
      let pid: pid_t = name.parse().unwrap();
      let parent_m_pss = if ppid.is_some() && ppid.unwrap() == pid {
        continue;
      } else if ppid.is_some() {
        self.plist.get(&ppid.unwrap()).unwrap().m_pss
      } else {
        0
      };
      let pre_existing = if !self.plist.contains_key(&pid) {
        self.plist.insert(pid, process::Process::new(pid));
        false
      } else {
        true
      };

      {
        // for shorter lifetime of mut ref plist
        let proc = self.plist.get_mut(&pid).unwrap();

        // update process info
        proc.tgid = if ppid.is_some() { ppid.unwrap() } else { pid };
        proc.is_userland_thread = proc.pid != proc.tgid;
      }

      // recurse more into its tasks
      let taskdir = &format!("{}/{}/task", dname, pid);
      self.recurse_proc_tree(Some(pid), taskdir, average_period);

      let proc = self.plist.get_mut(&pid).unwrap();

      if pre_existing && proc.is_kernel_thread {
        proc.is_updated = true;
        proc.show = false;
        self.kernel_threads += 1;
        self.total_tasks += 1;
        continue;
      }
      if pre_existing && proc.is_userland_thread {
        proc.is_updated = true;
        proc.show = false;
        self.userland_threads += 1;
        self.total_tasks += 1;
        continue;
      }

      // update process's page usage
      procmem::read_statm(proc, &format!("{}/{}", dname, pid));

      // XXX reading smaps file should be rate-limited for performance.
      if !proc.is_kernel_thread {
        if ppid.is_none() {
          // root process
          procmem::read_smaps_rollup(proc, &format!("{}/{}", dname, pid));
        } else {
          // child thread
          proc.m_pss = parent_m_pss;
        }
      }

      let lasttimes = proc.utime + proc.stime;
      let old_tty_nr = proc.tty_nr;
      pstat::update_with_stat(proc, dname, self.btime, self.jiffy);

      // XXX update of TTY device should be cond-limited for performance.
      if old_tty_nr != proc.tty_nr && self.tty_drivers.len() != 0 {
        proc.tty_name = tty::get_updated_tty_driver(&self.tty_drivers, proc.tty_nr as u64);
      }


      // update CPU usage
      proc.percent_cpu = if average_period < 0.1_f64.powi(6) {
        0.0
      } else {
        clamp(
          (proc.utime as f64 + proc.stime as f64 - lasttimes as f64) / average_period * 100.0,
          0.0,
          self.cpus.len() as f64 * 100.0,
        )
      };

      // update cmdline, comm, exe
      cmdline::read_cmd_files(proc, &format!("{}/{}", dname, pid));

      if proc.is_kernel_thread {
        self.kernel_threads += 1;
      } else if proc.is_userland_thread {
        self.userland_threads += 1;
      }

      proc.show = true; // XXX
      self.total_tasks += 1;
      proc.is_updated = true;
    }

    // XXX must delete dead processes
  }

  pub fn get_sorted_by_cpu(&self, num: usize) -> Vec<process::Process> {
    let mut procs: Vec<process::Process> = self.plist.values().cloned().collect();
    procs.sort_by(|a,b| b.percent_cpu.partial_cmp(&a.percent_cpu).unwrap());
    procs.into_iter().take(num).collect()
  }
}
