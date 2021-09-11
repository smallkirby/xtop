use super::{input, process};
use crate::render::processmeter_manager::ProcessMeterManager;

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

  pub fn to_usage(&self) -> String {
    use CommandType::*;
    match self {
      Input => "i: xinput operation".into(),
      Process => "p: process list operation".into(),
      Invalid => "".into(),
    }
  }
}

#[derive(Default)]
pub struct Commander {
  is_active: bool,
}

pub struct CommanderUsage {}
impl CommanderUsage {
  pub fn all_usage() -> Vec<String> {
    use CommandType::*;
    let types = [Input, Process];
    types.iter().map(|t| t.to_usage()).collect()
  }
}

impl Commander {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn complete(&mut self, part: &str) -> Vec<String> {
    let mut completions = vec![];

    let tokens: Vec<&str> = part.split_whitespace().collect();
    let command = if tokens.is_empty() {
      CommandType::Invalid
    } else {
      CommandType::from(tokens[0])
    };

    match command {
      CommandType::Input => {
        completions.extend(input::InputCommand::all_usage().iter().cloned());
      }
      CommandType::Process => {
        completions.extend(process::ProcCommand::all_usage().iter().cloned());
      }
      CommandType::Invalid => {
        completions.extend(CommanderUsage::all_usage().iter().cloned());
      }
    }

    completions
  }

  pub fn execute(&mut self, command: &str, procmanager: &mut ProcessMeterManager) -> String {
    use self::CommandType::*;
    self.is_active = false;
    let tokens = command.split_whitespace().collect::<Vec<&str>>();
    if tokens.is_empty() {
      return "".into();
    }

    let typ = CommandType::from(tokens[0]);
    match typ {
      Input => input::execute(tokens[1..].to_vec()),
      Process => process::execute(tokens[1..].to_vec(), procmanager),
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
