/*****

Implementation of IoMeter.
IoMeter shows the IO usages.

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};

use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.
static THRESHOLD: u64 = 500;

pub struct IoMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
}

impl Meter for IoMeter {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    // draw header
    mvwaddstr_color(win, 0, 1, &format!(" IO "), cpair::PAIR_HEAD);

    wrefresh(win);
  }

  fn init_meter(
    parent: WINDOW,
    wm: &mut WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    let height = std::cmp::min(height, MAXBUFSZ as i32);
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    wrefresh(win);

    IoMeter { width, height, win }
  }

  fn resize(&mut self, parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.height = height;
    self.width = width;
    wresize(self.win, height, width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
