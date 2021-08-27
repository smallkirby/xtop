/*****

/proc/tty related funcs.

*******/

use std::fs;

#[derive(Debug, PartialEq)]
pub struct TtyDriver {
  path: String,
  major: u32,
  minor_from: u32,
  minor_to: u32,
}

pub fn init_tty_drivers(proc_drivers: &mut Vec<TtyDriver>) {
  proc_drivers.clear();

  let drivers_s = fs::read_to_string("/proc/tty/drivers").unwrap();
  for line in drivers_s.split("\n").into_iter() {
    let driver = match parse_drivers_line(line) {
      Some(_driver) => _driver,
      None => continue,
    };
    proc_drivers.push(driver);
  }
  // XXX drivers should be sorted in order of major/minor numbers?
}

// parse a line like below to get TtyDriver.
// /dev/tty             /dev/tty        5       0 system:/dev/tty
fn parse_drivers_line(line: &str) -> Option<TtyDriver> {
  let mut tokens = line.split_whitespace().collect::<Vec<&str>>().into_iter();
  if tokens.len() != 5 {
    return None;
  }

  tokens.next(); // skip driver name
  let path = String::from(tokens.next().unwrap());
  let major = tokens.next().unwrap().parse().unwrap();
  let minor = tokens.next().unwrap();
  let (minor_from, minor_to) = if minor.contains("-") {
    let minors: Vec<&str> = minor.split("-").collect();
    (minors[0].parse().unwrap(), minors[1].parse().unwrap())
  } else {
    let _minor = minor.parse().unwrap();
    (_minor, _minor)
  };

  Some(TtyDriver {
    path,
    major,
    minor_from,
    minor_to,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_tty_driver_line() {
    let line1 = "/dev/console         /dev/console    5       1 system:console";
    let line2 = "pty_slave            /dev/pts      136 0-1048575 pty:slave";
    let correct1 = TtyDriver {
      path: String::from("/dev/console"),
      major: 5,
      minor_from: 1,
      minor_to: 1,
    };
    let correct2 = TtyDriver {
      path: String::from("/dev/pts"),
      major: 136,
      minor_from: 0,
      minor_to: 1048575,
    };
    let driver1 = parse_drivers_line(line1).unwrap();
    let driver2 = parse_drivers_line(line2).unwrap();
    assert_eq!(driver1, correct1);
    assert_eq!(driver2, correct2);
  }
}
