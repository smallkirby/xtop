/*****

/proc/tty related funcs.

*******/

use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq)]
pub struct TtyDriver {
  path: String,
  major: u32,
  minor_from: u32,
  minor_to: u32,
}

impl PartialOrd for TtyDriver {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    use crate::util::spaceship_number_u32;
    match spaceship_number_u32(self.major, other.major) {
      Ordering::Equal => Some(spaceship_number_u32(self.minor_from, other.minor_from)),
      Ordering::Greater => Some(Ordering::Greater),
      Ordering::Less => Some(Ordering::Less),
    }
  }
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

  sort_drivers(proc_drivers);
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

fn sort_drivers(drivers: &mut Vec<TtyDriver>) {
  let sorter = |a: &TtyDriver, b: &TtyDriver| a.partial_cmp(b).unwrap();
  drivers.sort_by(sorter);
}

// get recent name of TTY device of the proc.
pub fn get_updated_tty_driver(drivers: &Vec<TtyDriver>, tty_nr: u64) -> String {
  use crate::util::*;
  let min = minor(tty_nr);
  let maj = major(tty_nr);

  for i in 0..drivers.len() {
    let driver = &drivers[i];
    if driver.path.is_empty() || maj < driver.major {
      break;
    }
    if maj > driver.major {
      continue;
    }
    if min < driver.minor_from {
      break;
    }
    if min > driver.minor_to {
      continue;
    }

    let mut idx = min - driver.minor_from;
    let mut fullpath = String::new();
    loop {
      // step1: check /`tty_path`/idx
      fullpath = format!("{}/{}", driver.path, idx);
      match get_dev_number(&fullpath) {
        Some(n) => {
          if n == maj as u64 && n == min as u64 {
            return fullpath;
          }
        }
        None => {}
      };

      // step2: check /`tty_path``idx`
      fullpath = format!("{}{}", driver.path, idx);
      match get_dev_number(&fullpath) {
        Some(n) => {
          if n == maj as u64 && n == min as u64 {
            return fullpath;
          }
        }
        None => {}
      };

      if idx == min {
        break;
      }
      idx = min;
    }

    // step3: check simple path
    fullpath = format!("{}", driver.path);
    match get_dev_number(&fullpath) {
      Some(n) => {
        if tty_nr == n {
          return fullpath;
        }
      }
      None => {}
    };
  }

  // step4: last
  format!("/dev/{}:{}", maj, min)
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

  #[test]
  fn test_sort_drivers() {
    let mut drivers = vec![
      parse_drivers_line("rfcomm               D   216 6-255 serial").unwrap(),
      parse_drivers_line("/dev/console         B    5       1 system:console").unwrap(),
      parse_drivers_line("/dev/tty             A        5       0 system:/dev/tty").unwrap(),
      parse_drivers_line("rfcomm               C   216 3-255 serial").unwrap(),
    ];
    sort_drivers(&mut drivers);
    assert_eq!(drivers[0].path, String::from("A"));
    assert_eq!(drivers[1].path, String::from("B"));
    assert_eq!(drivers[2].path, String::from("C"));
    assert_eq!(drivers[3].path, String::from("D"));
  }
}
