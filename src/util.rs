pub fn clamp(v: f64, from: f64, to: f64) -> f64 {
  if v < from {
    from
  } else if v > to {
    to
  } else {
    v
  }
}

pub fn popi64(ss: &mut Vec<&str>) -> i64 {
  let n = ss[0].parse().unwrap();
  ss.remove(0);
  n
}

pub fn popc(ss: &mut Vec<&str>) -> char {
  let c = ss[0].chars().next().unwrap();
  ss.remove(0);
  c
}
