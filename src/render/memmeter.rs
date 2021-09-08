/*****

Implementation of MemMeter.
MemMeter shows the statistics of memory usage.

*******/

use crate::render::color::*;
use crate::resource::mem;
use ncurses::*;

use crate::render::{meter::Meter, window::WinManager};

static UNIT_MB: u64 = 1024;
static UNIT_GB: u64 = UNIT_MB * 1024;

pub struct MemMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  usage: Option<mem::MemInfo>,
}

impl MemMeter {
  pub fn set_usage(&mut self, usage: &mem::MemInfo) {
    self.usage = Some(usage.clone());
  }
}

impl Meter for MemMeter {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    let mut x0 = 1;
    let mut cx = x0;
    let mut cy = 1;
    let usage = self.usage.as_ref().unwrap();

    // draw data
    let s = &format!("total: {} MB", usage.total / UNIT_MB);
    mvwaddstr(win, cy, cx, s);
    cy += 1;

    // draw bars
    let x_start = 3;
    let width = self.width - 2 - x_start;
    let height = self.height - 2;
    let y_bottom = height;

    // draw header
    mvwaddstr_color(win, 0, 1, &format!(" Memory () "), cpair::PAIR_HEAD);

    wrefresh(win);
  }

  fn init_meter(
    parent: WINDOW,
    wm: &mut WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    if height.is_none() || width.is_none() {
      panic!("height and width must be specified for MemMeter::init_meter().");
    }
    let height = height.unwrap();
    let width = width.unwrap();
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    wrefresh(win);

    MemMeter {
      width,
      height,
      win,
      usage: None,
    }
  }

  fn resize(&mut self, parent: WINDOW, height: Option<i32>, width: Option<i32>, y: i32, x: i32) {
    self.width = match width {
      Some(w) => w,
      None => self.width,
    };
    self.height = match height {
      Some(h) => h,
      None => self.height,
    };

    wresize(self.win, self.height, self.width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
