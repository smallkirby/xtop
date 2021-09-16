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
    let a = if a < 0 { 100 } else { a };
    let b = if b < 0 { 100 } else { b };
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
  fn get_single_col(d0: (Option<i32>, Option<i32>), d1: (Option<i32>, Option<i32>)) -> String {
    let (d00, d01) = d0;
    let (d10, d11) = d1;
    let mut d00 = d00.unwrap_or(-1);
    let mut d01 = d01.unwrap_or(-1);
    let mut d10 = d10.unwrap_or(-1);
    let mut d11 = d11.unwrap_or(-1);

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

  pub fn get_brails_single(maxheight: i32, min: f64, max: f64, d0: Vec<f64>) -> Vec<String> {
    let fake_d = vec![min - 100.0; d0.len()];

    get_brails(maxheight, min, max, d0, fake_d)
  }

  pub fn get_brails(maxheight: i32, min: f64, max: f64, d0: Vec<f64>, d1: Vec<f64>) -> Vec<String> {
    if d0.len() != d1.len() {
      eprintln!("Error: get_brails(): len of d0 and d1 differs.");
      return vec![];
    }

    let maxheight = (maxheight * 4) as u64;
    let mut res = vec![];
    let range = max - min;
    let pos = |v| (maxheight as f64 * ((v - min) / range)) as i32;

    let _d0: Vec<i32> = d0.iter().map(|d| pos(*d)).collect();
    let _d1: Vec<i32> = d1.iter().map(|d| pos(*d)).collect();
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

      let col = get_single_col((d00, d10), (d01, d11));
      res.push(col);
    }

    res
  }
}

// 3x2 bit Brailles.
pub mod b32 {
  use std::collections::HashSet;

  // Character and Color pair
  pub struct Cc {
    pub ch: char, // character
    pub co: i16,  // cpair
  }

  impl Cc {
    pub fn from(ch: char, co: i16) -> Self {
      Self { ch, co }
    }
  }

  static COMPLEMENT_THRESHOLD: f32 = 0.8;

  // 3x2 dot is named as below:
  //  - dot in left col is called ax. at right is called bx.
  //  - dot in y-th position is called xy. (0-indexed)
  // eg: `⠁` is called `a3`.
  // dot xy is given index of 2**y.
  // eg:`⠁` 's index is 2**2.
  // index of multiple dots in the same col is given by sum of their index.
  // eg: `⠃` 's index is 2**1 + 2**2 == 6.
  // eg: `⠖` is accessed by [3][2]
  static DOTS: [[&str; 8]; 8] = [
    [" ", "⠠", "⠐", "⠰", "⠈", "⠨", "⠘", "⠸"],
    ["⠄", "⠤", "⠔", "⠴", "⠌", "⠬", "⠜", "⠼"],
    ["⠂", "⠢", "⠒", "⠲", "⠊", "⠪", "⠚", "⠺"],
    ["⠆", "⠦", "⠖", "⠶", "⠎", "⠮", "⠞", "⠾"],
    ["⠁", "⠡", "⠑", "⠱", "⠉", "⠩", "⠙", "⠹"],
    ["⠅", "⠥", "⠕", "⠵", "⠍", "⠭", "⠝", "⠽"],
    ["⠃", "⠣", "⠓", "⠳", "⠋", "⠫", "⠛", "⠻"],
    ["⠇", "⠧", "⠗", "⠷", "⠏", "⠯", "⠟", "⠿"],
  ];

  // convert dots into index.
  fn dots_to_index(dots: Vec<i32>) -> usize {
    let dots = if dots.len() > 3 { vec![] } else { dots };

    dots.into_iter().fold(0, |sum, d| {
      if !(0..3).contains(&d) {
        sum
      } else {
        sum + 2_u32.pow(d as u32) as usize
      }
    }) as usize
  }

  // convert single value into dots.
  fn value_to_dots(v: i32) -> Vec<i32> {
    if !(0..3).contains(&v) {
      vec![-1]
    } else {
      vec![v]
    }
  }

  // convert index into a brail.
  fn get_brail(d0: i32) -> &'static str {
    let dot = value_to_dots(d0);
    let i = dots_to_index(dot);
    DOTS[i][0]
  }

  // convert one data into brails.
  fn get_single_col(d0: i32) -> String {
    let mut d0 = d0;
    let mut res = String::new();
    loop {
      if d0 < 0 {
        break;
      }

      let brail = get_brail(d0);
      res.push_str(brail);

      d0 -= 3;
    }

    res
  }

  // convert one data into brails, which is complemented by back/forward data.
  fn get_single_col_complement(maxheight: i32, v: i32, a: Option<i32>, b: Option<i32>) -> String {
    let is_dot_on_line_right = |x: i32, y: i32| {
      let r_right = (b.unwrap() as f32 - v as f32) / 2.0;
      let lx_right = |y| (y - v as f32) / r_right;
      let lx = lx_right(y as f32);
      lx.is_nan() || (lx - x as f32).abs() < COMPLEMENT_THRESHOLD
    };

    let is_dot_on_line_left = |x: i32, y: i32| {
      let r_left = (v as f32 - a.unwrap() as f32) / 2.0;
      let lx_left = |y| (y - v as f32) / r_left;
      let lx = lx_left(y as f32);
      lx.is_nan() || (lx - x as f32).abs() <= COMPLEMENT_THRESHOLD
    };

    let is_dot_not_overrun = |v: i32, a: i32, b: i32| {
      if b >= a {
        v <= b
      } else {
        v >= b
      }
    };

    let mut dots0 = HashSet::new();
    let mut dots1 = HashSet::new();
    let mut brails = String::new();
    for y in 0..maxheight {
      if b.is_some() && is_dot_on_line_right(0, y) && is_dot_not_overrun(y, v, b.unwrap()) {
        dots0.insert(y % 3);
      }
      if b.is_some() && is_dot_on_line_right(1, y) && is_dot_not_overrun(y, v, b.unwrap()) {
        dots1.insert(y % 3);
      }
      if a.is_some()
        && is_dot_on_line_left(0, y)
        && b.is_some()
        && is_dot_not_overrun(y, v, b.unwrap())
      {
        dots0.insert(y % 3);
      }
      if a.is_none() && b.is_none() && y == v {
        dots0.insert(y % 3);
      }

      if y % 3 == 2 {
        if let Some(_b) = b {
          if dots1.is_empty() && dots0.len() == 1 {
            if _b > v && !dots0.contains(&2) {
              dots1.insert((v + 1) % 3);
            } else if _b < v && !dots0.contains(&0) {
              dots1.insert((v - 1) % 3);
            }
          }
        }
        let li = dots_to_index(dots0.clone().into_iter().collect());
        let ri = dots_to_index(dots1.clone().into_iter().collect());
        let brail = DOTS[li][ri];
        brails.push_str(brail);
        dots0.clear();
        dots1.clear();
      }
    }

    brails
  }

  // convert data pair into single line of colorized brails.
  fn get_single_col_complement_2axes(
    maxheight: i32,
    ent0: (i32, Option<i32>, Option<i32>, i16), // y0, y1, y2, color
    ent1: (i32, Option<i32>, Option<i32>, i16), // y0, y1, y2, color
  ) -> Vec<Cc> {
    let (v0, a0, b0, c0) = ent0;
    let (v1, a1, b1, c1) = ent1;

    let is_dot_on_line_right0 = |x: i32, y: i32| {
      let r_right = (b0.unwrap() as f32 - v0 as f32) / 2.0;
      let lx_right = |y| (y - v0 as f32) / r_right;
      let lx = lx_right(y as f32);
      lx.is_nan() || (lx - x as f32).abs() < COMPLEMENT_THRESHOLD
    };

    let is_dot_on_line_right1 = |x: i32, y: i32| {
      let r_right = (b1.unwrap() as f32 - v1 as f32) / 2.0;
      let lx_right = |y| (y as f32 - v1 as f32) / r_right;
      let lx = lx_right(y);
      lx.is_nan() || (lx - x as f32).abs() < COMPLEMENT_THRESHOLD
    };

    let is_dot_on_line_left0 = |x: i32, y: i32| {
      let r_left = (v0 as f32 - a0.unwrap() as f32) / 2.0;
      let lx_left = |y| (y - v0 as f32) / r_left;
      let lx = lx_left(y as f32);
      lx.is_nan() || (lx - x as f32).abs() <= COMPLEMENT_THRESHOLD
    };

    let is_dot_on_line_left1 = |x: i32, y: i32| {
      let r_left = (v1 as f32 - a1.unwrap() as f32) / 2.0;
      let lx_left = |y| (y - v1 as f32) / r_left;
      let lx = lx_left(y as f32);
      lx.is_nan() || (lx - x as f32).abs() <= COMPLEMENT_THRESHOLD
    };

    let is_dot_not_overrun = |v: i32, a: i32, b: i32| {
      if b >= a {
        v <= b
      } else {
        v >= b
      }
    };

    let mut dots00 = HashSet::new();
    let mut dots01 = HashSet::new();
    let mut dots10 = HashSet::new();
    let mut dots11 = HashSet::new();

    let mut res = vec![];
    for y in 0..maxheight {
      if b0.is_some() && is_dot_on_line_right0(0, y) && is_dot_not_overrun(y, v0, b0.unwrap()) {
        dots00.insert(y % 3);
      }
      if b0.is_some() && is_dot_on_line_right0(1, y) && is_dot_not_overrun(y, v0, b0.unwrap()) {
        dots01.insert(y % 3);
      }
      if a0.is_some()
        && is_dot_on_line_left0(0, y)
        && b0.is_some()
        && is_dot_not_overrun(y, v0, b0.unwrap())
      {
        dots00.insert(y % 3);
      }
      if a0.is_none() && b0.is_none() && y == v0 {
        dots00.insert(y % 3);
      }

      if b1.is_some() && is_dot_on_line_right1(0, y) && is_dot_not_overrun(y, v1, b1.unwrap()) {
        dots10.insert(y % 3);
      }
      if b1.is_some() && is_dot_on_line_right1(1, y) && is_dot_not_overrun(y, v1, b1.unwrap()) {
        dots11.insert(y % 3);
      }
      if a1.is_some()
        && is_dot_on_line_left1(0, y)
        && b1.is_some()
        && is_dot_not_overrun(y, v1, b1.unwrap())
      {
        dots10.insert(y % 3);
      }
      if a1.is_none() && b1.is_none() && y == v1 {
        dots10.insert(y % 3);
      }

      if y % 3 == 2 {
        if let Some(_b0) = b0 {
          if dots01.is_empty() && dots00.len() == 1 {
            if _b0 > v0 && !dots00.contains(&2) {
              dots01.insert((v0 + 1) % 3);
            } else if _b0 < v0 && !dots00.contains(&0) {
              dots01.insert((v0 - 1) % 3);
            }
          }
        }
        if let Some(_b1) = b1 {
          if dots11.is_empty() && dots10.len() == 1 {
            if _b1 > v1 && !dots10.contains(&2) {
              dots11.insert((v1 + 1) % 3);
            } else if _b1 < v1 && !dots10.contains(&0) {
              dots11.insert((v1 - 1) % 3);
            }
          }
        }
        let li0 = dots_to_index(dots00.clone().into_iter().collect());
        let ri0 = dots_to_index(dots01.clone().into_iter().collect());
        let li1 = dots_to_index(dots10.clone().into_iter().collect());
        let ri1 = dots_to_index(dots11.clone().into_iter().collect());
        let brail = if (li0, ri0) != (0, 0) {
          Cc::from(DOTS[li0][ri0].chars().next().unwrap(), c0)
        } else {
          Cc::from(DOTS[li1][ri1].chars().next().unwrap(), c1)
        };
        res.push(brail);
        dots00.clear();
        dots01.clear();
        dots10.clear();
        dots11.clear();
      }
    }

    res
  }

  // convert two data into colorized line-chart, using two independent range.
  pub fn get_brails_complement_2sep_axes_color(
    maxheight: i32,
    range0: (f64, f64),
    range1: (f64, f64),
    d0: (Vec<f64>, i16),
    d1: (Vec<f64>, i16),
  ) -> Vec<Vec<Cc>> {
    let (min0, max0) = range0;
    let (min1, max1) = range1;
    let (data0, color0) = d0;
    let (data1, color1) = d1;

    // normalize data to positive
    let normalize = |data: Vec<f64>, min: f64, max: f64| {
      let zero_diff = 0.0 - min;
      let normalized_data: Vec<f64> = data.into_iter().map(|d| d + zero_diff).collect();
      (normalized_data, 0.0, max + zero_diff)
    };
    let (data0, min0, max0) = normalize(data0, min0, max0);
    let (data1, _min1, max1) = normalize(data1, min1, max1);

    // use (min0, max0) for range and apply linear-transformation for data1
    let trans_ratio = max0 / max1;
    let aligned_data1: Vec<f64> = data1.into_iter().map(|d| d * trans_ratio).collect();

    get_brails_complement_2axes_color(
      maxheight,
      min0,
      max0,
      (data0, color0),
      (aligned_data1, color1),
    )
  }

  // convert two data into colorized line-chart.
  pub fn get_brails_complement_2axes_color(
    maxheight: i32,
    min: f64,
    max: f64,
    d0: (Vec<f64>, i16),
    d1: (Vec<f64>, i16),
  ) -> Vec<Vec<Cc>> {
    let (d0, color0) = d0;
    let (d1, color1) = d1;
    if d0.len() != d1.len() {
      eprintln!("Error: get_brails_complement_2axes_color(): len of d0 and d1 differs.");
      return vec![];
    }

    let maxheight = maxheight * 3;
    let mut res = vec![];
    let range = max - min;
    let pos = |v| (maxheight as f64 * ((v - min) / range)) as i32;
    let d0: Vec<i32> = d0.into_iter().map(pos).collect();
    let d1: Vec<i32> = d1.into_iter().map(pos).collect();

    for i in 0..d0.len() {
      let v0 = d0[i];
      let a0 = if i == 0 { None } else { Some(d0[i - 1]) };
      let b0 = if i == d0.len() - 1 {
        None
      } else {
        Some(d0[i + 1])
      };
      let v1 = d1[i];
      let a1 = if i == 0 { None } else { Some(d1[i - 1]) };
      let b1 = if i == d1.len() - 1 {
        None
      } else {
        Some(d1[i + 1])
      };

      let col =
        get_single_col_complement_2axes(maxheight, (v0, a0, b0, color0), (v1, a1, b1, color1));
      res.push(col);
    }

    res
  }

  // convert goven data into vec of braills, which are complemented.
  pub fn get_brails_complement(maxheight: i32, min: f64, max: f64, d0: Vec<f64>) -> Vec<String> {
    let maxheight = maxheight * 3;
    let mut res = vec![];
    let range = max - min;
    let pos = |v| (maxheight as f64 * ((v - min) / range)) as i32;
    let d: Vec<i32> = d0.into_iter().map(pos).collect();

    for i in 0..d.len() {
      let v = d[i];
      let a = if i == 0 { None } else { Some(d[i - 1]) };
      let b = if i == d.len() - 1 {
        None
      } else {
        Some(d[i + 1])
      };

      let brail = get_single_col_complement(maxheight, v, a, b);
      res.push(brail);
    }

    res
  }

  // convert given data into vec of braills
  pub fn get_brails(maxheight: i32, min: f64, max: f64, d0: Vec<f64>) -> Vec<String> {
    let maxheight = (maxheight * 3) as u64;
    let mut res = vec![];
    let range = max - min;
    let pos = |v| (maxheight as f64 * ((v - min) / range)) as i32;

    // normalize given data into [0, maxheight - 1]
    let d0: Vec<i32> = d0.into_iter().map(pos).collect();
    let mut d0 = d0.into_iter();

    // get column with brail til data get empty
    loop {
      let v = d0.next();
      if v.is_none() {
        break;
      }
      let col = get_single_col(v.unwrap());
      res.push(col);
    }

    res
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn show_brail(brails: Vec<String>, maxheight: i32) {
    println!("---------------------");
    for i in 0..maxheight {
      let j = (maxheight - (i + 1)) as usize;
      for _brail in &brails {
        let brail = _brail.clone();
        let mut chars = brail.chars();
        if chars.clone().count() <= j as usize {
          print!(" ");
        } else {
          print!("{}", chars.nth(j).unwrap());
        }
      }
      println!();
    }
    println!("---------------------");
  }

  #[test]
  fn test_show_brail42() {
    let a = vec![
      "ABCDE".into(),
      "HIJKLMN".into(),
      "OPQRS".into(),
      "VWXYZab".into(),
    ];
    // `a` is shown as A is at left-bottom, and b is at right-top;
    show_brail(a, 10);

    // cross sign
    let d0: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let d1: Vec<u32> = d0.clone().into_iter().rev().collect();

    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let d1: Vec<f64> = d1.into_iter().map(|d| d as f64).collect();

    let height = 12;
    let brail = b42::get_brails(height, 0.0, 16.0, d0, d1);
    show_brail(brail, height);

    // parallel lines
    let d0: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let d1: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let d1: Vec<f64> = d1.into_iter().map(|d| d as f64).collect();

    let height = 20;
    let brail = b42::get_brails(height, 0.0, 20.0, d0, d1);
    show_brail(brail, height);

    // single line
    let d0: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();

    let height = 20;
    let brail = b42::get_brails_single(height, 0.0, 20.0, d0);
    show_brail(brail, height);
  }

  #[test]
  fn test_show_brail32() {
    let d0: Vec<u32> = vec![0, 1, 2, 3];
    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let height = 4;
    let brail = b32::get_brails_complement(height, 0.0, 4.0, d0);
    show_brail(brail, height);

    let d0: Vec<u32> = vec![2, 8, 9, 9, 10, 5, 1, 2];
    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let height = 12;
    let brail = b32::get_brails_complement(height, 0.0, 11.0, d0);
    show_brail(brail, height);

    // watch if 0-3 line doesn't overrun
    let d0: Vec<u32> = vec![0, 3, 3, 3, 3, 3];
    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let height = 4;
    let brail = b32::get_brails_complement(height, 0.0, 4.0, d0);
    show_brail(brail, height);

    // watch if 0-3 line is well complemented
    let d0: Vec<u32> = vec![0, 3, 0, 0, 0, 3, 0, 0, 9, 0];
    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let height = 6;
    let brail = b32::get_brails_complement(height, 0.0, 30.0, d0);
    show_brail(brail, height);

    let d0: Vec<u32> = vec![0, 3, 0, 0, 0, 3, 0, 0, 9, 0, 19, 3];
    let d0: Vec<f64> = d0.into_iter().map(|d| d as f64).collect();
    let height = 6;
    let brail = b32::get_brails_complement(height, 0.0, 30.0, d0);
    show_brail(brail, height);
  }
}
