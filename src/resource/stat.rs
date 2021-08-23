use super::process::{Char2ProcState, ProcState};

type pid_t = i32;

#[derive(Debug, PartialEq)]
pub struct Stat {
  pub pid: pid_t,
  pub comm: String,
  pub state: ProcState,
  pub ppid: pid_t,
  pub pgid: pid_t,
  pub sid: pid_t,       // session id
  pub tty_nr: i32,
  pub tty_pgrp: i32,
  pub flags: u32,
  pub min_flt: u64,     // minor fault counter
  pub cmin_flt: u64,
  pub maj_flt: u64,
  pub cmaj_flt: u64,
  pub utime: u64,
  pub stime: u64,
  // TODO
}

impl Stat {
  pub fn from(s: String) -> Stat {
    use self::util::*;
    let mut ss = s.split(" ").collect::<Vec<&str>>();

    let pid = popi(&mut ss) as pid_t;
    let comm = extract_comm(&mut ss);
    let state = Char2ProcState(popc(&mut ss));
    let ppid = popi(&mut ss) as pid_t;
    let pgid = popi(&mut ss) as pid_t;
    let sid = popi(&mut ss) as pid_t;
    let tty_nr = popi(&mut ss) as i32;
    let tty_pgrp = popi(&mut ss) as i32;
    let flags = popi(&mut ss) as u32;
    let min_flt = popi(&mut ss) as u64;
    let cmin_flt = popi(&mut ss) as u64;
    let maj_flt = popi(&mut ss) as u64;
    let cmaj_flt = popi(&mut ss) as u64;
    let utime = popi(&mut ss) as u64;
    let stime = popi(&mut ss) as u64;

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
    }
  }
} 

mod util {
  pub fn popi(ss: &mut Vec<&str>) -> i64 {
    let n = ss[0].parse().unwrap();
    ss.remove(0);
    n
  }

  pub fn popc(ss: &mut Vec<&str>) -> char {
    let c = ss[0].chars().nth(0).unwrap();
    ss.remove(0);
    c
  }

  pub fn extract_comm(ss: &mut Vec<&str>) -> String {
    let mut comm = String::from("");
    if ss[0].ends_with(')') {
      comm.push_str(&ss[0][1..(ss[0].len()-1)]);
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
    use crate::resource::process::ProcState;
    use super::Stat;

  #[test]
  fn stat_from_string() {
    let stat_str = "1586595 (bash) S 13042 1586595 1586595 34821 1616731 4194304 2487 9875 0 0 2 2";
    let stat = Stat::from(String::from(stat_str));
    let correct_stat = Stat {
      pid: 1586595,
      comm: String::from("bash"),
      state: ProcState::SLEEPING,
      ppid: 13042,
      pgid: 1586595,
      sid: 1586595,
      tty_nr: 34821,
      tty_pgrp: 1616731,
      flags: 4194304,
      min_flt: 2487,
      cmin_flt: 9875,
      maj_flt: 0,
      cmaj_flt: 0,
      utime: 2,
      stime: 2,
    };
    assert_eq!(correct_stat, stat);
  }
}
