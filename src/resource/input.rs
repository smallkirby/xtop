/*****

Implementation of X input device utilities.
This is **just a wrapper of xinput**.

The device information can be read from sysfs.
e.g: /sys/devices/pci0000:00/0000:00:14.0/usb1/1-4/1-4:1.0/0003:4653:0001.0012/input/input35
The location of sysfs can be read from /proc/bus/input/devices.

XXX I have to read xlibx later...

*******/

use std::process::Command;

static FLOATMARK: &str = "∼";
static ATTACHEDMARK1: &str = " ";
static ATTACHEDMARK2: &str = "⎜";

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
  Pointer,
  Keyboard,
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
  Master,        // master input
  Attached(u32), // slave, attached to `id`:u32
  Floating,      // slave, floating
}

#[derive(Debug, Clone)]
pub struct InputDevice {
  pub name: String,
  pub typ: InputType,
  pub id: u32,
  pub state: State,
}

impl InputDevice {
  // generate InputDevice from a line.
  // NOTE: if `typ` member is `Attached`, its value becomes 0.
  pub fn from_line(st: &str) -> Option<InputDevice> {
    use InputType::*;
    use State::*;

    if st.len() <= 1 {
      return None;
    }

    let state = if st.starts_with(FLOATMARK) {
      Floating
    } else if st.starts_with(ATTACHEDMARK1) || st.starts_with(ATTACHEDMARK2) {
      Attached(0)
    } else {
      Master
    };

    let st = st.chars().collect::<Vec<_>>();
    let s: String = match state {
      Floating => st[2..].into_iter().collect(),
      Attached(_) => {
        let __s: String = st[2..].into_iter().collect();
        let _s = __s.trim().chars().collect::<Vec<_>>();
        _s[2..].into_iter().collect()
      }
      Master => {
        let _s: String = st[2..].into_iter().collect();
        _s.trim().to_string()
      }
    };

    let tokens: Vec<&str> = s.split_whitespace().collect();

    // parse name and ID
    let id_idx = tokens.iter().position(|&r| r.starts_with("id=")).unwrap();
    let name = tokens[0..id_idx].join(" ");
    let id: u32 = tokens[id_idx]["id=".len()..].parse().unwrap();

    // parse type and state
    let typ = match tokens[id_idx + 2] {
      "keyboard" => Keyboard,
      "pointer" => Pointer,
      _ => Pointer,
    };

    Some(Self {
      name,
      typ,
      state,
      id,
    })
  }
}

pub fn get_devices() -> Vec<InputDevice> {
  use State::*;

  let mut devices = vec![];
  let xinput_output = match read_xinput_list() {
    Ok(_o) => _o,
    Err(_) => return devices,
  };
  let mut current_master_id = 0;
  for s in xinput_output.split("\n") {
    let mut device = match InputDevice::from_line(s) {
      Some(_d) => _d,
      None => break,
    };
    match device.state {
      Master => current_master_id = device.id,
      Attached(_) => device.state = Attached(current_master_id),
      _ => {}
    };
    devices.push(device);
  }

  devices
}

fn read_xinput_list() -> Result<String, String> {
  let output = match Command::new("xinput").arg("list").arg("--short").output() {
    Ok(_o) => String::from_utf8(_o.stdout).unwrap(),
    Err(e) => return Err(e.to_string()),
  };

  Ok(output)
}

#[cfg(test)]
mod tests {
  use super::*;

  //#[test]
  #[allow(dead_code)]
  fn test_get_devices() {
    let devices = get_devices();
    println!("{:?}", devices);
  }
}
