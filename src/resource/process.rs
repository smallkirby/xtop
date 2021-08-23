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

pub fn Char2ProcState(c: char) -> ProcState {
  return match c {
    'R' => ProcState::RUNNING,
    'S' => ProcState::SLEEPING,
    'Z' => ProcState::ZOMBIE,
    'T' => ProcState::STOPPED,
    'X' => ProcState::DEAD,
    'W' => ProcState::WAKING,
    _ => ProcState::UNKNOWN,
  };
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
