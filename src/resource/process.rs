use crate::resource::pstat::pid_t;
use std::fs;

#[derive(PartialEq, Debug)]
pub enum ProcState {
  RUNNING,
  SLEEPING,
  ZOMBIE,
  STOPPED,
  DEAD,
  WAKING,
  UNKNOWN,
}

#[derive(Debug, Default)]
pub struct Process {
  pub pid: pid_t,
  pub tgid: pid_t,
  pub cminflt: u64,
  pub cmajflt: u64,
  pub utime: u64,
  pub stime: u64,
  pub cutime: u64,
  pub cstime: u64,
  pub m_share: i64,
  pub m_pss: i64,
  pub m_swap: i64,

  pub is_userland_thread: bool,
  pub is_kernel_thread: bool,
  pub is_updated: bool,
  pub show: bool,

  // read from statm
  pub m_virt: i64,     // total program size [kB]
  pub m_resident: i64, // resident set size  [kB]
  pub m_shared: i64,   // resident shared pages
  pub m_text: i64,     // text
  pub m_data: i64,     // data and stack
  pub m_dirty: i64,    // dirty pages
}

impl Process {
  pub fn new(pid: pid_t) -> Self {
    Self {
      pid,
      ..Default::default()
    }
  }
}

#[allow(non_snake_case)]
pub fn Char2ProcState(c: char) -> ProcState {
  match c {
    'R' => ProcState::RUNNING,
    'S' => ProcState::SLEEPING,
    'Z' => ProcState::ZOMBIE,
    'T' => ProcState::STOPPED,
    'X' => ProcState::DEAD,
    'W' => ProcState::WAKING,
    _ => ProcState::UNKNOWN,
  }
}

// search /proc/ and return a list of all existing pids.
pub fn list_all_pids() -> Vec<u64> {
  let mut pids = vec![];
  let proc_dir = fs::read_dir("/proc/").unwrap();
  for dir in proc_dir {
    let path = dir
      .unwrap()
      .path()
      .file_name()
      .unwrap()
      .to_string_lossy()
      .to_string();
    if let Ok(pid) = path.parse() {
      pids.push(pid);
    }
  }

  pids
}

#[cfg(test)]
mod tests {
  #[test]
  fn char2state_running() {
    use super::{Char2ProcState, ProcState};
    let state = Char2ProcState('R');
    assert_eq!(state, ProcState::RUNNING);
  }
}
