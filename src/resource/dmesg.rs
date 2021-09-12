/*****

/dev/kmsg related funcs.

*******/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{
  fs,
  io::{self, BufRead},
};

static BLOCK_LIMIT_MS: u64 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct KmsgLine {
  pub level: u8,
  pub id: u64,
  pub timestamp: u64, // microseconds
  pub log: String,
}

impl Default for KmsgLine {
  fn default() -> Self {
    Self {
      level: 0,
      id: 0,
      timestamp: 0,
      log: "[invalid log]".into(),
    }
  }
}

impl KmsgLine {
  pub fn from(s: &str) -> Self {
    let tokens: Vec<&str> = s.split(",").collect();
    if tokens.len() < 3 {
      return Self::default();
    }

    let level = tokens[0].parse().unwrap();
    let id = tokens[1].parse().unwrap();
    let timestamp = tokens[2].parse().unwrap();

    /* ignore flags  */

    let _rest = tokens[3..].to_vec().join(",");
    let rest: Vec<&str> = _rest.split(";").collect();
    let log = rest[1..].join(";");

    Self {
      level,
      id,
      timestamp,
      log: log,
    }
  }
}

pub fn get_kmsgs() -> Vec<KmsgLine> {
  let mut results = vec![];
  let lines = match read_kmsg_lines_noblock() {
    Ok(l) => l,
    Err(_) => return results,
  };

  for l in lines {
    results.push(KmsgLine::from(&l));
  }

  results
}

fn read_kmsg_lines_noblock() -> Result<Vec<String>, String> {
  let kmsg = match fs::File::open("/dev/kmsg") {
    Ok(f) => f,
    Err(_) => return Err("failed to open /dev/kmsg.".into()),
  };
  let must_kill = Arc::new(Mutex::new(false));
  let (value_tx, value_rx) = mpsc::channel();
  let mut result = vec![];

  let lines = io::BufReader::new(kmsg).lines();
  let line_read_must_kill = must_kill.clone();
  let _line_read_handler = thread::spawn(move || {
    for line in lines {
      if *line_read_must_kill.lock().unwrap() {
        break;
      }
      match line {
        Ok(s) => value_tx.send(s).unwrap(),
        Err(_) => break,
      }
    }
  });

  loop {
    match value_rx.try_recv() {
      Ok(v) => result.push(v),
      Err(_) => {
        // wait for a limit and retry
        thread::sleep(Duration::from_millis(BLOCK_LIMIT_MS));
        match value_rx.try_recv() {
          Ok(v) => result.push(v),
          Err(_) => {
            *must_kill.lock().unwrap() = true;
            break;
          }
        }
      }
    }
  }

  //let _ = line_read_handler.join();

  Ok(result)
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn test_read_kmsg() {
    let lines = read_kmsg_lines_noblock();
    assert_eq!(lines.is_ok(), true);
  }

  #[test]
  fn test_parse_line() {
    let a1 = "3,122793,256982503404,-;usb usb2-port2: Cannot enable. Maybe the USB cable is bad?";
    let b1 = KmsgLine::from(a1);
    let c1 = KmsgLine {
      level: 3,
      id: 122793,
      timestamp: 256982503404,
      log: "usb usb2-port2: Cannot enable. Maybe the USB cable is bad?".into(),
    };
    assert_eq!(c1, b1);
  }
}
