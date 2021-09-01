/*****

/proc/uptime related funcs.

*******/

use std::fs;

#[derive(Debug, PartialEq, Clone)]
pub struct Uptime {
  pub uptime: f64,   // uptime since last boot
  pub idle_sum: f64, // total idle time of each cores
}

impl Uptime {
  pub fn new() -> Self {
    read_uptime()
  }

  pub fn update(&mut self) {
    let new = read_uptime();
    self.uptime = new.uptime;
    self.idle_sum = new.idle_sum;
  }

  pub fn into_readable_string(&self) -> String {
    let u = self.uptime as u64;
    let seconds = u % 60;
    let minutes = (u / 60) % 60;
    let hours = (u / 3600) % 24;
    let days = u / 86400;

    let days_s = if days == 0 {
      "".to_string()
    } else if days == 1 {
      "1 day ".to_string()
    } else {
      format!("{} days ", days)
    };

    format!("{}{:>02}:{:>02}:{:>02}", days_s, hours, minutes, seconds)
  }
}

fn read_uptime() -> Uptime {
  let uptime_s = fs::read_to_string("/proc/uptime").unwrap();
  let tokens: Vec<&str> = uptime_s.split(" ").map(|s| s.trim()).collect();
  if tokens.len() != 2 {
    panic!("uouo fish life");
  }
  let uptime = tokens[0].parse().unwrap();
  let idle_sum = tokens[1].parse().unwrap();

  Uptime { uptime, idle_sum }
}
