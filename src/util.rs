/*****

Utility functions used globally.

*******/

use std::cmp::Ordering;
use std::fmt;
use std::os::unix::fs::MetadataExt;
use std::{fs, path};

/* data unit funcs */
#[derive(Clone, Copy)]
pub enum DataUnit {
  B,
  Kb,
  Mb,
  Gb,
}

impl fmt::Display for DataUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use DataUnit::*;
    match self {
      B => write!(f, "B"),
      Kb => write!(f, "KB"),
      Mb => write!(f, "MB"),
      Gb => write!(f, "GB"),
    }
  }
}

#[derive(Clone, Copy)]
pub struct DataSize<T> {
  pub val: T,
  pub unit: DataUnit,
}

impl<T> DataSize<T>
where
  T: std::ops::Mul<Output = T> + std::ops::Div<Output = T> + From<u32> + Copy,
{
  pub fn new(val: T, unit: DataUnit) -> Self {
    Self { val, unit }
  }

  pub fn convert(&self, unit: DataUnit) -> T {
    use DataUnit::*;
    match (unit, self.unit) {
      (B, B) => self.val,
      (B, Kb) => self.val * (1024).into(),
      (B, Mb) => self.val * (1024 * 1024).into(),
      (B, Gb) => self.val * (1024 * 1024 * 1024).into(),
      (Kb, B) => self.val / (1024).into(),
      (Kb, Kb) => self.val,
      (Kb, Mb) => self.val * (1024).into(),
      (Kb, Gb) => self.val * (1024 * 1024).into(),
      (Mb, B) => self.val / (1024 * 1024).into(),
      (Mb, Kb) => self.val / (1024).into(),
      (Mb, Mb) => self.val,
      (Mb, Gb) => self.val * (1024).into(),
      (Gb, B) => self.val / (1024 * 1024 * 1024).into(),
      (Gb, Kb) => self.val / (1024 * 1024).into(),
      (Gb, Mb) => self.val / (1024).into(),
      (Gb, Gb) => self.val,
    }
  }
}

/* number processing funcs */

// round @v to fit in from..=to
pub fn clamp(v: f64, from: f64, to: f64) -> f64 {
  if v < from {
    from
  } else if v > to {
    to
  } else {
    v
  }
}

// compare to return Ordering
pub fn spaceship_number_u32(a: u32, b: u32) -> Ordering {
  match a.cmp(&b) {
    Ordering::Greater => Ordering::Greater,
    Ordering::Less => Ordering::Less,
    Ordering::Equal => Ordering::Equal,
  }
}

/* queue related funcs */
pub fn popi64(ss: &mut Vec<&str>) -> i64 {
  let n = ss[0].parse().unwrap();
  ss.remove(0);
  n
}

pub fn popu64(ss: &mut Vec<&str>) -> u64 {
  let n = ss[0].parse().unwrap();
  ss.remove(0);
  n
}

pub fn popc(ss: &mut Vec<&str>) -> char {
  let c = ss[0].chars().next().unwrap();
  ss.remove(0);
  c
}

/* dev related funcs */

// receive dev_t like number and return major
pub fn major(nr: u64) -> u32 {
  (((nr >> 8) & 0xFFF) | ((nr >> 32) & !(0xFFF_u64))) as u32
}

// receive dev_t like number and return minor
pub fn minor(nr: u64) -> u32 {
  ((nr & 0xFF) | ((nr >> 12) & !(0xFF_u64))) as u32
}

pub fn get_dev_number(path: &str) -> Option<u64> {
  let meta = match fs::metadata(path) {
    Ok(_meta) => _meta,
    Err(_) => return None,
  };
  Some(meta.dev())
}

/* Path related funcs */

// receives full path or file name and return (directory, file) name pair.
// NOTE: returned `directory string
//          : does NOT contain trailing "/" if given `full_path` is just a file name.
//          : contains trailing "/" if given 'full_path' is a path.
pub fn get_dir_file(full_path: &str) -> (String, String) {
  let exe_name = full_path;
  if !exe_name.contains('/') {
    ("".into(), exe_name.into())
  } else {
    let exe_path = path::Path::new(exe_name);
    let exe_path_file = exe_path.file_name().unwrap().to_str().unwrap();
    let exe_path_dir = if exe_name.contains('/') {
      &exe_name[0..(exe_name.len() - exe_path_file.len())]
    } else {
      ""
    };

    (exe_path_dir.into(), exe_path_file.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dev_number() {
    let nr1 = 34841;
    let major1 = major(nr1);
    let minor1 = minor(nr1);
    assert_eq!(major1, 136);
    assert_eq!(minor1, 25);
  }

  #[test]
  fn test_get_dir_file() {
    let ex1 = "/home/wataru/.config/Code/User/globalStorage/matklad.rust-analyzer/rust-analyzer-x86_64-unknown-linux-gnu";
    let (ex1_dir, ex1_file) = get_dir_file(ex1);
    let ex2 = "./waiwai/uouo";
    let (ex2_dir, ex2_file) = get_dir_file(ex2);
    let ex3 = "xtop";
    let (ex3_dir, ex3_file) = get_dir_file(ex3);

    assert_eq!(
      ex1_dir,
      "/home/wataru/.config/Code/User/globalStorage/matklad.rust-analyzer/"
    );
    assert_eq!(ex1_file, "rust-analyzer-x86_64-unknown-linux-gnu");
    assert_eq!(ex2_dir, "./waiwai/");
    assert_eq!(ex2_file, "uouo");
    assert_eq!(ex3_dir, "");
    assert_eq!(ex3_file, "xtop");
  }
}
