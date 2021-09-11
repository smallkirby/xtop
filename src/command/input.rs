/*********

Input subcommands

*********/

use crate::resource::input::{get_devices, InputDevice, State};

use std::process::Command;

pub enum InputSubcommand {
  Kill,
  ReattachAll,
  Invalid,
}

impl InputSubcommand {
  pub fn from(s: &str) -> Self {
    use InputSubcommand::*;
    match s {
      "k" | "kill" => Kill,
      "r" => ReattachAll,
      _ => Invalid,
    }
  }
}

pub fn execute(_command: Vec<&str>) -> String {
  use InputSubcommand::*;
  if _command.is_empty() {
    return "invalid subcommand".into();
  }
  let mut command = _command.iter();

  let subcommand = InputSubcommand::from(command.next().unwrap());
  match subcommand {
    Kill => {
      if command.len() == 1 {
        let num = match command.next().unwrap().parse() {
          Ok(n) => n,
          Err(_) => return "invalid subcommand: i k <ID>".into(),
        };
        float_device(num)
      } else {
        "invalid subcommand".into()
      }
    }
    ReattachAll => reattach_all(),
    Invalid => "invalid subcommand".into(),
  }
}

fn float_device(id: u32) -> String {
  let output = match Command::new("xinput")
    .arg("float")
    .arg(id.to_string())
    .output()
  {
    Ok(_o) => format!("Successfully killed device: {}", id),
    Err(_e) => _e.to_string(),
  };

  output
}

fn reattach_all() -> String {
  let devices = get_devices();
  let masters: Vec<InputDevice> = devices
    .clone()
    .into_iter()
    .filter(|d| d.state == State::Master)
    .collect();

  let mut reattached_ids = vec![];
  let mut failed_ids = vec![];
  for device in devices.clone() {
    if device.state == State::Floating {
      match reattach(&device, &masters) {
        Ok(_) => reattached_ids.push(device.id),
        Err(_) => failed_ids.push(device.id),
      }
    }
  }

  if failed_ids.is_empty() {
    format!(
      "successfully reattached: {}",
      reattached_ids
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    )
  } else {
    format!(
      "failed to reattach: {}",
      failed_ids
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    )
  }
}

fn reattach(slave: &InputDevice, masters: &[InputDevice]) -> Result<(), ()> {
  // XXX is it possible to know the floating device's type?
  // for now, try reattaching to the all masters.
  // when type does not match, it just fails and try next master.
  for master in masters {
    match Command::new("xinput")
      .arg("reattach")
      .arg(slave.id.to_string())
      .arg(master.id.to_string())
      .output()
    {
      Ok(o) => {
        if o.stderr.is_empty() {
          return Ok(());
        }
      }
      Err(_) => return Err(()),
    };
  }
  Err(())
}
