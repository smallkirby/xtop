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
