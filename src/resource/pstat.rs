/*****

/proc/<pid>/stat/ related funcs.

*******/

use crate::resource::process::{Char2ProcState, ProcState, Process};
use std::fs;

#[allow(non_camel_case_types)]
pub type pid_t = i32;

// cf. /fs/proc/array.c/do_task_stat()
#[derive(Debug, PartialEq)]
pub struct PStat {
  pub pid: pid_t,
  pub comm: String,
  pub state: ProcState,
  pub ppid: pid_t,
  pub pgid: pid_t,
  pub sid: pid_t,  // session id
  pub tty_nr: i32, // controlling terminal of the proc(minor/major num)
  pub tty_pgrp: i32,
  pub flags: u32,    // see /include/linux/sched.h
  pub min_flt: u64,  // # of minor fault
  pub cmin_flt: u64, // # of minor fault of children
  pub maj_flt: u64,  // # of major fault counter
  pub cmaj_flt: u64, // # of major fault of children
  pub utime: u64,    // amount of time scheduled in usermode
  pub stime: u64,    // amount of time scheduled in kernelmode
  pub cutime: i64,   // amount of time waited-for children've been scheduled in usermode
  pub cstime: i64,   // amount of time waited-for children've been scheduled in usermode
  pub priority: i64,
  pub nice: i64,
  pub nlwp: i64,      // # of threads
  pub starttime: i64, // time after boot [clock tick]
  pub processor: i32, // id of cpu last executed on
}

impl PStat {
  pub fn from(s: String) -> PStat {
    use self::util::*;
    use crate::util::*;

    let mut ss = s.split(' ').collect::<Vec<&str>>();

    let pid = popi64(&mut ss) as pid_t;
    let comm = pop_comm(&mut ss);
    let state = Char2ProcState(popc(&mut ss));
    let ppid = popi64(&mut ss) as pid_t;
    let pgid = popi64(&mut ss) as pid_t;
    let sid = popi64(&mut ss) as pid_t;
    let tty_nr = popi64(&mut ss) as i32;
    let tty_pgrp = popi64(&mut ss) as i32;
    let flags = popi64(&mut ss) as u32;
    let min_flt = popi64(&mut ss) as u64;
    let cmin_flt = popi64(&mut ss) as u64;
    let maj_flt = popi64(&mut ss) as u64;
    let cmaj_flt = popi64(&mut ss) as u64;
    let utime = popi64(&mut ss) as u64;
    let stime = popi64(&mut ss) as u64;
    let cutime = popi64(&mut ss);
    let cstime = popi64(&mut ss);
    let priority = popi64(&mut ss);
    let nice = popi64(&mut ss);
    let nlwp = popi64(&mut ss);
    popi64(&mut ss);
    let starttime = popi64(&mut ss);

    popu64(&mut ss);
    popi64(&mut ss);
    popu64(&mut ss); // rsslim
    for _ in 0..12 {
      popu64(&mut ss);
    }
    popi64(&mut ss);

    let processor = popi64(&mut ss) as i32;

    Self {
      pid,
      comm,
      state,
      ppid,
      pgid,
      sid,
      tty_nr,
      tty_pgrp,
      flags,
      min_flt,
      cmin_flt,
      maj_flt,
      cmaj_flt,
      utime,
      stime,
      cutime,
      cstime,
      priority,
      nice,
      nlwp,
      starttime,
      processor,
    }
  }
}

pub fn read_stat(pid: pid_t) -> Result<PStat, String> {
  let stat_str = match fs::read_to_string(format!("/proc/{}/stat", pid)) {
    Ok(s) => s,
    Err(err) => return Err(err.to_string()),
  };
  Ok(PStat::from(stat_str))
}

pub fn update_with_stat(proc: &mut Process, dirname: &str, btime: i64, jiffy: i64) {
  let stat = read_stat(proc.pid).unwrap();

  if stat.pid != proc.pid {
    panic!("PID not match.");
  }

  let adjust_time = |t| t * 100 / jiffy as u64;

  proc.state = stat.state;
  proc.ppid = stat.ppid;
  proc.pgrp = stat.pgid;
  proc.session = stat.sid;
  proc.tty_nr = stat.tty_nr;
  proc.tpgid = stat.tty_pgrp;
  proc.minflt = stat.min_flt;
  proc.cminflt = stat.cmin_flt;
  proc.majflt = stat.maj_flt;
  proc.cmajflt = stat.cmaj_flt;
  proc.utime = adjust_time(stat.utime);
  proc.stime = adjust_time(stat.stime);
  proc.cutime = adjust_time(stat.cutime as u64);
  proc.cstime = adjust_time(stat.cstime as u64);
  proc.priority = stat.priority;
  proc.nice = stat.nice;
  proc.nlwp = stat.nlwp;
  if proc.starttime == 0 {
    proc.starttime = btime + adjust_time(stat.starttime as u64) as i64;
  }
  proc.processor = stat.processor;
  proc.time = proc.utime + proc.stime;
}

mod util {
  pub fn pop_comm(ss: &mut Vec<&str>) -> String {
    let mut comm = String::from("");
    if ss[0].ends_with(')') {
      comm.push_str(&ss[0][1..(ss[0].len() - 1)]);
      ss.remove(0);
      return comm;
    }

    comm.push_str(ss[0]);
    ss.remove(0);
    loop {
      if ss[0].ends_with(')') {
        comm.push_str(&ss[0][..(ss[0].len() - 1)]);
        ss.remove(0);
        break;
      }
      comm.push_str(&ss[0]);
      ss.remove(0);
    }
    comm
  }
}

#[cfg(test)]
mod tests {
  use super::PStat;
  use crate::resource::process::ProcState;

  #[test]
  fn stat_from_string() {
    let stat_str = "1 (systemd) S 0 1 1 0 -1 4194560 59614 113423667 181 12785 5220 2387 48543 108042 20 0 1 0 29 172421120 3080 18446744073709551615 1 1 0 0 0 0 671173123 4096 1260 0 0 0 17 5 0 0 8 0 0 0 0 0 0 0 0 0 0";
    let stat = PStat::from(String::from(stat_str));
    let correct_stat = PStat {
      pid: 1,
      comm: String::from("systemd"),
      state: ProcState::SLEEPING,
      ppid: 0,
      pgid: 1,
      sid: 1,
      tty_nr: 0,
      tty_pgrp: -1,
      flags: 4194560,
      min_flt: 59614,
      cmin_flt: 113423667,
      maj_flt: 181,
      cmaj_flt: 12785,
      utime: 5220,
      stime: 2387,
      cutime: 48543,
      cstime: 108042,
      priority: 20,
      nice: 0,
      nlwp: 1,
      starttime: 29,
      processor: 5,
    };
    assert_eq!(correct_stat, stat);
  }

  #[test]
  fn read_systemd_stat() {
    let stat = super::read_stat(1).unwrap();
    println!("{:?}", stat);
  }
}
