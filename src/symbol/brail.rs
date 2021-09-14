/*****

Defines braille symbols.
cf.https://en.wikipedia.org/wiki/Braille_Patterns

*******/

pub mod b42 {
  const B01: [&str; 11] = ["⣤", "⣔", "⣌", "⣄", "⡴", "⡬", "⡤", "⡜", "⡔", "⡌", "⡄"];
  const B02: [&str; 11] = ["⣢", "⣒", "⣊", "⣂", "⡲", "⡪", "⡢", "⡚", "⡒", "⡊", "⡂"];
  const B03: [&str; 11] = ["⣡", "⣑", "⣉", "⣁", "⡱", "⡩", "⡡", "⡙", "⡑", "⡉", "⡁"];
  const B0N: [&str; 11] = ["⣠", "⣐", "⣈", "⣀", "⡰", "⡨", "⡠", "⡘", "⡐", "⡈", "⡀"];

  const B12: [&str; 11] = ["⢦", "⢖", "⢎", "⢆", "⠶", "⠮", "⠦", "⠞", "⠖", "⠎", "⠆"];
  const B13: [&str; 11] = ["⢥", "⢕", "⢍", "⢅", "⠵", "⠭", "⠥", "⠝", "⠕", "⠍", "⠅"];
  const B1N: [&str; 11] = ["⢤", "⢔", "⢌", "⢄", "⠴", "⠬", "⠤", "⠜", "⠔", "⠌", "⠄"];

  const B23: [&str; 11] = ["⢣", "⢓", "⢋", "⢃", "⠳", "⠫", "⠣", "⠛", "⠓", "⠋", "⠃"];
  const B2N: [&str; 11] = ["⢢", "⢒", "⢊", "⢂", "⠲", "⠪", "⠢", "⠚", "⠒", "⠊", "⠂"];

  const B3N: [&str; 11] = ["⢡", "⢑", "⢉", "⢁", "⠱", "⠩", "⠡", "⡙", "⠑", "⠉", "⠁"];

  const BNN: [&str; 11] = ["⢠", "⢐", "⢈", "⢀", "⠰", "⠨", "⠠", "⠘", "⠐", "⠈", "⠀"];

  const DOTS: [[&str; 11]; 11] = [B01, B02, B03, B0N, B12, B13, B1N, B23, B2N, B3N, BNN];

  fn index(a: i32, b: i32) -> usize {
    let v1 = std::cmp::min(a, b);
    let v2 = std::cmp::max(a, b);
    match (v1, v2) {
      (0, 1) => 0,
      (0, 2) => 1,
      (0, 3) => 2,
      (0, _) => 3,
      (1, 2) => 4,
      (1, 3) => 5,
      (1, _) => 6,
      (2, 3) => 7,
      (2, _) => 8,
      (3, _) => 9,
      _ => 10,
    }
  }

  fn get_brail(d0: (i32, i32), d1: (i32, i32)) -> &'static str {
    let (v00, v01) = d0;
    let (v10, v11) = d1;

    let i0 = index(v00, v01);
    let i1 = index(v10, v11);

    DOTS[i0][i1]
  }

  // `height` should be multiplied by 4 beforehand.
  fn get_single_col(d0: (Option<u32>, Option<u32>), d1: (Option<u32>, Option<u32>)) -> String {
    let (d00, d01) = d0;
    let (d10, d11) = d1;
    let mut d00 = match d00 {
      Some(d) => d as i32,
      None => -1,
    };
    let mut d01 = match d01 {
      Some(d) => d as i32,
      None => -1,
    };
    let mut d10 = match d10 {
      Some(d) => d as i32,
      None => -1,
    };
    let mut d11 = match d11 {
      Some(d) => d as i32,
      None => -1,
    };

    let mut res = String::new();
    loop {
      if d00 < 0 && d01 < 0 && d10 < 0 && d11 < 0 {
        break;
      }

      let brail = get_brail((d00, d01), (d10, d11));
      res.push_str(brail);

      d00 -= 4;
      d01 -= 4;
      d10 -= 4;
      d11 -= 4;
    }

    res
  }

  pub fn get_brails(maxheight: i32, min: f64, max: f64, d0: Vec<f64>, d1: Vec<f64>) -> Vec<String> {
    use crate::util::clamp;

    if d0.len() != d1.len() {
      eprintln!("Error: get_brails(): len of d0 and d1 differs.");
      return vec![];
    }

    let maxheight = (maxheight * 3) as u64;
    let mut res = vec![];
    let range = max - min;
    let pos = |v| {
      let v = clamp(v, min, max);
      (maxheight as f64 * ((v - min) / range)) as u32
    };

    let _d0: Vec<u32> = d0.iter().map(|d| pos(*d)).collect();
    let _d1: Vec<u32> = d1.iter().map(|d| pos(*d)).collect();
    let mut d0 = _d0.into_iter();
    let mut d1 = _d1.into_iter();

    loop {
      let d00 = d0.next();
      let d01 = d0.next();
      let d10 = d1.next();
      let d11 = d1.next();
      if d00.is_none() && d01.is_none() && d10.is_none() && d11.is_none() {
        break;
      }

      let col = get_single_col((d00, d01), (d10, d11));
      res.push(col);
    }

    res
  }
}

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
