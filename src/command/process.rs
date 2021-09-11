/*********

Process subcommands

*********/

use crate::render::processmeter_manager::{FilterType, ProcessMeterManager};

pub enum ProcCommand {
  Search,
  UnsetFilter,
  Invalid,
}

impl ProcCommand {
  pub fn from(s: &str) -> Self {
    use ProcCommand::*;
    match s {
      "s" | "search" => Search,
      "u" | "unset" => UnsetFilter,
      _ => Invalid,
    }
  }

  pub fn to_usage(&self) -> String {
    use ProcCommand::*;
    match self {
      Search => "s <pid | cmd>: filter processes".into(),
      UnsetFilter => "u: unset all filters".into(),
      Invalid => "".into(),
    }
  }

  pub fn all_usage() -> Vec<String> {
    use ProcCommand::*;
    let subs = [Search, UnsetFilter];
    subs.iter().map(|s| s.to_usage()).collect()
  }
}

#[derive(Default)]
pub struct ProcUsage {}

impl ProcUsage {
  pub fn complete(part: &str) -> Option<String> {
    let tokens: Vec<&str> = part.split_whitespace().collect();
    if tokens.is_empty() {
      return None;
    }
    let subcommand = ProcCommand::from(tokens[0]);

    Some(subcommand.to_usage())
  }
}

pub fn execute(_command: Vec<&str>, procmanager: &mut ProcessMeterManager) -> String {
  use ProcCommand::*;
  if _command.is_empty() {
    return "invalid subcommand".into();
  }
  let mut command = _command.iter();

  let subcommand = ProcCommand::from(command.next().unwrap());
  match subcommand {
    Search => {
      if command.len() == 1 {
        let keyword = command.next().unwrap();
        let filter = create_filter(keyword);
        procmanager.set_filter(filter.clone());
        match filter {
          FilterType::Cmd(cmd) => format!("Set filter by cmd: {}", cmd),
          FilterType::Pid(pid) => format!("Set filter by PID: {}", pid),
          FilterType::Nothing => "failed to set filter.".into(),
        }
      } else {
        "invalid subcommand".into()
      }
    }
    UnsetFilter => {
      let filter = FilterType::Nothing;
      procmanager.set_filter(filter);
      "Unset all filters".into()
    }
    Invalid => "invalid subcommand".into(),
  }
}

fn create_filter(keyword: &str) -> FilterType {
  match keyword.parse::<i32>() {
    Ok(pid) => FilterType::Pid(pid),
    Err(_) => FilterType::Cmd(keyword.into()),
  }
}
