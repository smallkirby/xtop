pub fn clamp(v: f64, from: f64, to: f64) -> f64 {
  if v < from {
    from
  } else if v > to {
    to
  } else {
    v
  }
}
