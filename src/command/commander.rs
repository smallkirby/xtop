use super::input;

#[derive(Debug)]
pub enum CommandType {
  Input,
  Process,
  Invalid,
}

impl CommandType {
  pub fn from(s: &str) -> Self {
    use CommandType::*;
    match s {
      "i" => Input,
      "p" => Process,
      _ => Invalid,
    }
  }
}

#[derive(Default)]
pub struct Commander {
  is_active: bool,
}

impl Commander {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn execute(&mut self, command: &str) -> String {
    use self::CommandType::*;
    self.is_active = false;
    let tokens = command.split_whitespace().collect::<Vec<&str>>();
    if tokens.is_empty() {
      return "".into();
    }

    let typ = CommandType::from(tokens[0]);
    match typ {
      Input => input::execute(tokens[1..].to_vec()),
      Process => "command not implemented".into(),
      Invalid => "invalid command".into(),
    }
  }

  pub fn is_active(&self) -> bool {
    self.is_active
  }

  pub fn start_input(&mut self) {
    self.is_active = true;
  }
}
