use crate::resource::process;
use crate::resource::procmem;
use crate::resource::pstat::pid_t;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct ProcList {
  plist: HashMap<pid_t, process::Process>,
  kernel_threads: u32,
  userland_threads: u32,
  total_tasks: u32,
}

impl ProcList {
  pub fn new() -> Self {
    let plist = HashMap::new();
    return Self {
      plist,
      kernel_threads: 0,
      userland_threads: 0,
      total_tasks: 0,
    };
  }

  pub fn recurse_proc_tree(&mut self, ppid: Option<pid_t>, _dname: &str) {
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
      let meta = ent.metadata().unwrap();
      let name = ent.file_name().into_string().unwrap();
      if !meta.is_dir() || !name.chars().nth(0).unwrap().is_numeric() {
        continue;
      }

      // newly add the process into tree if not exists.
      let pid: pid_t = name.parse().unwrap();
      if ppid.is_some() && ppid.unwrap() == pid {
        continue;
      }
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
      self.recurse_proc_tree(Some(pid), taskdir);

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

      procmem::read_statm(proc, &format!("{}/{}", dname, pid))
    }
  }
}
