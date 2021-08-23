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

#[cfg(test)]
mod tests {
  #[test]
  fn char2state_running() {
    use super::{Char2ProcState, ProcState};
    let state = Char2ProcState('R');
    assert_eq!(state, ProcState::RUNNING);
  }
}
