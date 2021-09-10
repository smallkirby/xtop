/********

Calculate layout related things.

********/

use super::config::*;

// receives (y,x) position and retturns clicked layout component and clicked offset inside the component.
pub fn get_layout_from_click(
  layouts: &[Layout],
  sheight: i32,
  swidth: i32,
  y: i32,
  x: i32,
) -> Option<(Layout, (i32, i32))> {
  let mut cx = 0;
  let mut cy = 0;
  let mut max_height_in_line = 0;

  for layout in layouts {
    let mut go_newline = false;
    let width = match layout.ratio {
      Size::Ratio(r) => (swidth as f64 * r) as i32,
      Size::Rest => {
        go_newline = true;
        swidth - cx
      }
    };
    let height = match layout.height {
      Height::Line(l) => l as i32,
      Height::Rest => sheight - cy,
      Height::Minus(l) => (sheight - cy) - l as i32,
    };
    max_height_in_line = std::cmp::max(max_height_in_line, height);

    if cx <= x && x < cx + width && cy <= y && y < cy + height {
      return Some((layout.clone(), (y - cy, x - cx)));
    }

    cx += width;
    if go_newline {
      cy += max_height_in_line;
      max_height_in_line = 0;
      cx = 0;
    }
  }

  None
}
