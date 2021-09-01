/*****

Defines block symbols
cf. https://en.wikipedia.org/wiki/Block_Elements

*******/

// Low-alighed verticasl blocks
pub mod lv {
  pub const FULL: &str = "▇";
  pub const SEVEN_EIGHTHS: &str = "▇";
  pub const THREE_QUARTERS: &str = "▆";
  pub const FIVE_EIGHTHS: &str = "▅";
  pub const HALF: &str = "▄";
  pub const THREE_EIGHTHS: &str = "▃";
  pub const ONE_QUARTER: &str = "▂";
  pub const ONE_EIGHTH: &str = "▁";

  pub fn get_bar(maxheight: i32, r: f64) -> String {
    let r = if r > 1.0 { 1.0 } else { r };

    let mut res = String::new();
    let mut n = maxheight as f64 * r; // num of full blocks
    loop {
      if n >= 1.0 {
        res.push_str(FULL);
        n -= 1.0;
      } else if n >= 0.875 {
        res.push_str(SEVEN_EIGHTHS);
        break;
      } else if n >= 0.75 {
        res.push_str(THREE_QUARTERS);
        break;
      } else if n >= 0.625 {
        res.push_str(FIVE_EIGHTHS);
        break;
      } else if n >= 0.5 {
        res.push_str(HALF);
        break;
      } else if n >= 0.375 {
        res.push_str(THREE_EIGHTHS);
        break;
      } else if n >= 0.25 {
        res.push_str(ONE_QUARTER);
        break;
      } else if n >= 0.125 {
        res.push_str(ONE_EIGHTH);
        break;
      } else {
        break;
      }
    }

    res
  }
}
