/*****

statm, smaps, smaps_rollup (under /proc/<pid>/) related funcs.

*******/

use crate::resource::process;
use std::fs;

// These values read from /proc/pid/statm can be inaccurate.
pub struct Statm {
  pub m_virt: i64,      // total program size [page]
  pub m_resident: i64,  // resident set size  [page]
  pub m_shared: i64,    // resident shared pages
  pub m_text: i64,      // text
  pub _unused_lib: i64, // unused
  pub m_data: i64,      // data and stack
  pub m_dirty: i64,     // dirty pages
}

impl Statm {
  pub fn new(s: &str) -> Self {
    use crate::util::popi64;

    let mut ss: Vec<&str> = s.split(" ").map(|_s| _s.trim()).collect();
    let m_virt = popi64(&mut ss);
    let m_resident = popi64(&mut ss);
    let m_shared = popi64(&mut ss);
    let m_text = popi64(&mut ss);
    let _unused_lib = popi64(&mut ss);
    let m_data = popi64(&mut ss);
    let m_dirty = popi64(&mut ss);

    Self {
      m_virt,
      m_resident,
      m_shared,
      m_text,
      _unused_lib,
      m_data,
      m_dirty,
    }
  }
}

pub fn read_statm(proc: &mut process::Process, parent_dir: &str) {
  use crate::consts::*;

  let statm_s = fs::read_to_string(format!("{}/statm", parent_dir)).unwrap();
  let statm = Statm::new(&statm_s);

  proc.m_virt = statm.m_virt * PAGESIZE_KB;
  proc.m_resident = statm.m_resident * PAGESIZE_KB;
  proc.m_shared = statm.m_shared;
  proc.m_text = statm.m_text;
  proc.m_data = statm.m_data;
  proc.m_dirty = statm.m_dirty;
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::resource::process;

  #[test]
  fn test_read_statm() {
    let path = "/proc/1";
    let mut process = process::Process::new(1);
    read_statm(&mut process, path);
    println!("{:?}", process);
  }
}
