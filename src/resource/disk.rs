/*****

/proc/diskstats related funcs.
cf: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats

xtop assumes only kernel 5.5+.

*******/

use std::{fs, ops};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiskStat {
  pub major: u32,      // major number
  pub minor: u32,      // minor number
  pub name: String,    // device name
  pub rd_io: u64,      // # of read operations toward the dev
  pub rd_merge: u64,   // # of merged read request
  pub rd_sector: u64,  // # of sectors read
  pub rd_tick: u64,    // time of read request in queue
  pub wr_io: u64,      // # of write operations toward the dev
  pub wr_merge: u64,   // # of merged write request
  pub wr_sector: u64,  // # of sectors written
  pub wr_tick: u64,    // time of write request in queue
  pub io_pgr: u64,     // # of IOs in progress
  pub total_tick: u64, // # of ticks for IOs.
  pub req_tick: u64,   // # of ticks requests spent/consumed in queue
  pub dc_io: u64,      // # of discard operations toward the dev
  pub dc_merge: u64,   // # of merged discard request
  pub dc_sector: u64,  // # of sectors discarded
  pub dc_tick: u64,    // time spent discarding
  pub fl_io: u64,      // # of flush operations toward the dev
  pub fl_tick: u64,    // time spent flushing
}

impl ops::AddAssign for DiskStat {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.clone() + rhs;
  }
}

impl ops::Add for DiskStat {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      major: 0,
      minor: 0,
      name: "sum".into(),
      rd_io: self.rd_io + other.rd_io,
      rd_merge: self.rd_merge + other.rd_merge,
      rd_sector: self.rd_sector + other.rd_sector,
      rd_tick: self.rd_tick + other.rd_tick,
      wr_io: self.wr_io + other.wr_io,
      wr_merge: self.wr_merge + other.wr_merge,
      wr_sector: self.wr_sector + other.wr_sector,
      wr_tick: self.wr_tick + other.wr_tick,
      io_pgr: self.io_pgr + other.io_pgr,
      total_tick: self.total_tick + other.total_tick,
      req_tick: self.req_tick + other.req_tick,
      dc_io: self.dc_io + other.dc_io,
      dc_merge: self.dc_merge + other.dc_merge,
      dc_sector: self.dc_sector + other.dc_sector,
      dc_tick: self.dc_tick + other.dc_tick,
      fl_io: self.fl_io + other.fl_io,
      fl_tick: self.fl_tick + other.fl_tick,
    }
  }
}

impl DiskStat {
  pub fn from(line: &str) -> Self {
    use crate::util::popu64 as p;

    let mut tokens: Vec<&str> = line.trim().split_whitespace().collect();
    if tokens.len() != 20 {
      // xtop assumes kernel5.5+ only.
      Self::default()
    } else {
      let major = p(&mut tokens) as u32;
      let minor = p(&mut tokens) as u32;
      let name = tokens[2].into();
      tokens.remove(0);
      let rd_io = p(&mut tokens);
      let rd_merge = p(&mut tokens);
      let rd_sector = p(&mut tokens);
      let rd_tick = p(&mut tokens);
      let wr_io = p(&mut tokens);
      let wr_merge = p(&mut tokens);
      let wr_sector = p(&mut tokens);
      let wr_tick = p(&mut tokens);
      let io_pgr = p(&mut tokens);
      let total_tick = p(&mut tokens);
      let req_tick = p(&mut tokens);
      let dc_io = p(&mut tokens);
      let dc_merge = p(&mut tokens);
      let dc_sector = p(&mut tokens);
      let dc_tick = p(&mut tokens);
      let fl_io = p(&mut tokens);
      let fl_tick = p(&mut tokens);

      Self {
        major,
        minor,
        name,
        rd_io,
        rd_merge,
        rd_sector,
        rd_tick,
        wr_io,
        wr_merge,
        wr_sector,
        wr_tick,
        io_pgr,
        total_tick,
        req_tick,
        dc_io,
        dc_merge,
        dc_sector,
        dc_tick,
        fl_io,
        fl_tick,
      }
    }
  }

  pub fn tps(&self, rhs: &Self, update_interval: f64) -> f64 {
    ((self.rd_io + self.wr_io + self.dc_io) - (rhs.rd_io + rhs.wr_io + rhs.dc_io)) as f64
      / update_interval
  }

  pub fn kb_read_persec(&self, rhs: &Self, update_interval: f64) -> f64 {
    // 1 sector is 2048-Bytes. So dividing by 2 means conversion into kB.
    let rsectors = self.rd_sector - rhs.rd_sector;
    (rsectors as f64 / update_interval) / 2.0
  }

  pub fn kb_write_persec(&self, rhs: &Self, update_interval: f64) -> f64 {
    // 1 sector is 2048-Bytes. So dividing by 2 means conversion into kB.
    let wsectors = self.wr_sector - rhs.wr_sector;
    (wsectors as f64 / update_interval) / 2.0
  }
}

pub fn get_diskstats() -> Vec<DiskStat> {
  let mut result = vec![];
  let invalid_stat = DiskStat::default();
  let diskstat_s = match fs::read_to_string("/proc/diskstats") {
    Ok(s) => s,
    Err(_) => return result,
  };

  let lines: Vec<&str> = diskstat_s.split('\n').collect();
  for line in lines {
    let stat = DiskStat::from(line);
    if stat.eq(&invalid_stat) {
      continue;
    }
    result.push(stat);
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_diskstat_from() {
    let line = " 259       0 nvme0n1 677898 87625 25725175 99967 7895662 4138190 233376044 10324527 0 6238032 12556215 122228 0 215446992 129925 775702 2001795";
    let diskstat = DiskStat::from(line);
    let answer = DiskStat {
      major: 259,
      minor: 0,
      name: "87625".into(),
      rd_io: 677898,
      rd_merge: 87625,
      rd_sector: 25725175,
      rd_tick: 99967,
      wr_io: 7895662,
      wr_merge: 4138190,
      wr_sector: 233376044,
      wr_tick: 10324527,
      io_pgr: 0,
      total_tick: 6238032,
      req_tick: 12556215,
      dc_io: 122228,
      dc_merge: 0,
      dc_sector: 215446992,
      dc_tick: 129925,
      fl_io: 775702,
      fl_tick: 2001795,
    };
    assert_eq!(answer, diskstat);
  }

  #[test]
  fn test_get_diskstats() {
    let diskstats = get_diskstats();
    assert_eq!(diskstats.is_empty(), false);
  }
}
