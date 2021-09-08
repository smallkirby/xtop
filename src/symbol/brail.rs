/*****

Defines braille symbols.
cf.https://en.wikipedia.org/wiki/Braille_Patterns

*******/

// 3x2 bit Brailles.
pub mod b32 {
  use crate::util::clamp;

  const B0N: &str = "⠄";
  const B1N: &str = "⠂";
  const B2N: &str = "⠁";

  const BN0: &str = "⠠";
  const BN1: &str = "⠐";
  const BN2: &str = "⠈";

  const B00: &str = "⠤";
  const B10: &str = "⠑";
  const B20: &str = "⠡";

  const B01: &str = "⠔";
  const B11: &str = "⠒";
  const B21: &str = "⠑";

  const B02: &str = "⠌";
  const B12: &str = "⠊";
  const B22: &str = "⠉";

  const BNN: &str = " ";

  pub fn get_brails(maxheight: i32, min: f64, max: f64, data: Vec<f64>) -> Vec<String> {
    let maxheight = (maxheight * 3) as u64;
    let mut res = vec![];
    let range = max - min;
    let pos = |_v| {
      let v = clamp(_v, min, max);
      (maxheight as f64 * ((v - min) / range)) as u32
    };

    let mut rest = data.iter();
    loop {
      if rest.len() == 0 {
        break;
      }

      let _d0 = rest.next();
      let _d1 = rest.next();
      match _d1 {
        None => {
          let d0 = pos(*_d0.unwrap());
          res.push(get_single_col(Some(d0), None));
        }
        Some(d1) => {
          let d0 = pos(*_d0.unwrap());
          let d1 = pos(*d1);
          res.push(get_single_col(Some(d0), Some(d1)));
        }
      }
    }

    res
  }

  // `height` should be multiplied by 3 beforehand.
  fn get_single_col(_d0: Option<u32>, _d1: Option<u32>) -> String {
    let mut d0 = match _d0 {
      Some(_d) => _d as i32 - 1,
      None => -1,
    };
    let mut d1 = match _d1 {
      Some(_d) => _d as i32 - 1,
      None => -1,
    };

    let mut res = String::new();
    loop {
      if d0 < 0 && d1 < 0 {
        return res;
      }

      match (d0, d1) {
        (0, 0) => res.push_str(B00),
        (0, 1) => res.push_str(B01),
        (0, 2) => res.push_str(B02),
        (0, _) => res.push_str(B0N),
        (1, 0) => res.push_str(B10),
        (1, 1) => res.push_str(B11),
        (1, 2) => res.push_str(B12),
        (1, _) => res.push_str(B1N),
        (2, 0) => res.push_str(B20),
        (2, 1) => res.push_str(B21),
        (2, 2) => res.push_str(B22),
        (2, _) => res.push_str(B2N),
        (_, 0) => res.push_str(BN0),
        (_, 1) => res.push_str(BN1),
        (_, 2) => res.push_str(BN2),
        _ => res.push_str(BNN),
      }

      d0 -= 3;
      d1 -= 3;
    }
  }
}
