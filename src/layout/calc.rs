/********

Calculate layout related things.

********/

use super::config::*;

#[derive(Debug)]
pub struct LayoutFixed {
  pub name: MeterName,
  pub y: i32,
  pub x: i32,
  pub height: i32,
  pub width: i32,
}

pub fn check_layout_validity(layouts: &[Layout]) -> Result<(), String> {
  match get_fixed_layouts_may_error(layouts, 100, 100) {
    Ok(_) => Ok(()),
    Err(s) => Err(s),
  }
}

pub fn get_fixed_layouts(layouts: &[Layout], sheight: i32, swidth: i32) -> Vec<LayoutFixed> {
  match get_fixed_layouts_may_error(layouts, sheight, swidth) {
    Ok(fixed_layouts) => fixed_layouts,
    Err(_) => vec![],
  }
}

fn get_fixed_layouts_may_error(
  layouts: &[Layout],
  sheight: i32,
  swidth: i32,
) -> Result<Vec<LayoutFixed>, String> {
  let mut fixed_layouts = vec![];
  let mut max_height_in_line = 0;
  let mut line_width = swidth;
  let mut multiline_waiting_queue = None;
  let mut x_start = 0;
  let mut cur_x = x_start;
  let mut cur_y = 0;

  for layout in layouts {
    let mut go_newline = false;

    let width = match layout.ratio {
      Size::Ratio(r) => (line_width as f64 * r) as i32,
      Size::Rest => {
        go_newline = true;
        line_width - cur_x
      }
    };

    let height = match layout.height {
      Height::Line(l) => l as i32,
      Height::Rest => sheight - cur_y,
      Height::Minus(l) => (sheight - cur_y) - l as i32,
      // if this component uses multiple Line, wait until Line height is fixed
      Height::Multiple(l) => {
        if l <= 1 {
          return Err("Multline component should have more than two Lines.".into());
        }
        if !(cur_x == 0 || go_newline) {
          return Err("Multiline component should be at left/right-most for now.".into());
        }
        if cur_x == 0 {
          x_start = width;
        } else {
          line_width = swidth - width;
        }
        if multiline_waiting_queue.is_some() {
          return Err("Multiline component in the same Line is not allowed for now.".into());
        } else {
          multiline_waiting_queue = Some((
            LayoutFixed {
              name: layout.name.clone(),
              y: cur_y,
              x: cur_x,
              width,
              height: 0,
            },
            l as i64,
          ));
        }

        -1
      }
    };

    if height != -1 {
      max_height_in_line = std::cmp::max(max_height_in_line, height);
      fixed_layouts.push(LayoutFixed {
        name: layout.name.clone(),
        y: cur_y,
        x: cur_x,
        height,
        width,
      });
    }

    cur_x += width;
    if go_newline {
      if let Some((lay, remained_line)) = multiline_waiting_queue {
        let remained_line = remained_line - 1;
        if remained_line == 0 {
          multiline_waiting_queue = None;
          x_start = 0;
          line_width = swidth;

          let height = (cur_y + max_height_in_line) - lay.y;
          fixed_layouts.push(LayoutFixed {
            name: lay.name,
            y: lay.y,
            x: lay.x,
            height,
            width: lay.width,
          });
        } else {
          multiline_waiting_queue = Some((lay, remained_line));
        }
      }

      cur_y += max_height_in_line;
      max_height_in_line = 0;
      cur_x = x_start;
    }
  }

  Ok(fixed_layouts)
}

// receives (y,x) position and retturns clicked layout component and clicked offset inside the component.
pub fn get_layout_from_click(
  layouts: &[Layout],
  sheight: i32,
  swidth: i32,
  y: i32,
  x: i32,
) -> Option<(MeterName, (i32, i32))> {
  let fixed_layouts = get_fixed_layouts(layouts, sheight, swidth);

  for layout in fixed_layouts {
    if layout.y <= y
      && y < (layout.y + layout.height)
      && layout.x <= x
      && x < (layout.x + layout.width)
    {
      return Some((layout.name, (y - layout.y, x - layout.x)));
    }
  }

  None
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::layout::config;

  //#[test]
  #[allow(dead_code)]
  fn test_calc_layout() {
    let layout = config::read_layout_config();
    let fixed_layout = get_fixed_layouts(&layout, 400, 1000);
    println!("{:?}", fixed_layout);
  }
}
