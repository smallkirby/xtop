use std::cmp::Ordering;
use std::fs;
use std::os::unix::fs::MetadataExt;

/* number processing funcs*/

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
  if a > b {
    Ordering::Greater
  } else if a < b {
    Ordering::Less
  } else {
    Ordering::Equal
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
  (((nr >> 8) & 0xFFF) | ((nr >> 32) & !(0xFFF as u64))) as u32
}

// receive dev_t like number and return minor
pub fn minor(nr: u64) -> u32 {
  ((nr & 0xFF) | ((nr >> 12) & !(0xFF as u64))) as u32
}

pub fn get_dev_number(path: &str) -> Option<u64> {
  let meta = match fs::metadata(path) {
    Ok(_meta) => _meta,
    Err(_) => return None,
  };
  Some(meta.dev())
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
}
