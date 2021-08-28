/*****

/proc/loadavg related funcs.

*******/

use std::fs;

#[derive(Debug, PartialEq)]
pub struct LoadAvg {
  // CPU and IO load of past x-minutes
  pub one: f64,
  pub five: f64,
  pub fifteen: f64,

  // last executed pid
  pub last: u32,
}

impl LoadAvg {
  pub fn new() -> Self {
    read_loadavg()
  }

  pub fn update(&mut self) {
    let new = read_loadavg();
    self.one = new.one;
    self.five = new.five;
    self.fifteen = new.fifteen;
    self.last = new.last;
  }
}

pub fn read_loadavg() -> LoadAvg {
  let s_loadavg = fs::read_to_string("/proc/loadavg").unwrap();
  let tokens: Vec<&str> = s_loadavg.split(" ").map(|s| s.trim()).collect();

  let one = tokens[0].parse().unwrap();
  let five = tokens[1].parse().unwrap();
  let fifteen = tokens[2].parse().unwrap();
  let last = tokens[4].parse().unwrap();

  LoadAvg {
    one,
    five,
    fifteen,
    last,
  }
}
