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

pub fn execute(_command: Vec<&str>) {
  use InputSubcommand::*;
  if _command.is_empty() {
    return;
  }
  let mut command = _command.iter();

  let subcommand = InputSubcommand::from(command.next().unwrap());
  match subcommand {
    Kill => {
      if command.len() == 1 {
        let num = match command.next().unwrap().parse() {
          Ok(n) => n,
          Err(_) => return,
        };
        float_device(num);
      }
    }
    ReattachAll => {
      reattach_all();
    }
    Invalid => {}
  }
}

fn float_device(id: u32) {
  let _output = match Command::new("xinput")
    .arg("float")
    .arg(id.to_string())
    .output()
  {
    Ok(_o) => String::from_utf8(_o.stdout).unwrap(),
    Err(_e) => return, // XXX
  };
}

fn reattach_all() {
  let devices = get_devices();
  let masters: Vec<InputDevice> = devices
    .clone()
    .into_iter()
    .filter(|d| d.state == State::Master)
    .collect();

  for device in devices.clone() {
    if device.state == State::Floating {
      reattach(&device, &masters);
    }
  }
}

fn reattach(slave: &InputDevice, masters: &[InputDevice]) {
  // XXX is it possible to know the floating device's type?
  // for now, try reattaching to the all masters.
  // when type does not match, it just fails and try next master.
  for master in masters {
    let _output = match Command::new("xinput")
      .arg("reattach")
      .arg(slave.id.to_string())
      .arg(master.id.to_string())
      .output()
    {
      Ok(_o) => String::from_utf8(_o.stdout).unwrap(),
      Err(e) => e.to_string(),
    };
  }
}
