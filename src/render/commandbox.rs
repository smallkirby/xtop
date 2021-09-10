/*********

Implementation of CommandBox.
CommandBox shows the box to input user input.
And it shows help box.

*********/

use crate::render::{color::*, meter::*};

use ncurses::*;

pub struct CommandBox {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  command_buffer: String,
}

impl CommandBox {
  fn draw_header(&self) {
    use crate::render::color::cpair::*;
    let mut x = 0;

    let s = "  ❦";
    wattron(self.win, COLOR_PAIR(PAIR_CUTE));
    mvwaddstr(self.win, 0, x, s);
    wattroff(self.win, COLOR_PAIR(PAIR_CUTE));
    x += s.len() as i32;

    let s = &self.command_buffer;
    mvwaddstr(self.win, 0, x, s);
    x += s.len() as i32;

    let s = " ❦ ";
    wattron(self.win, COLOR_PAIR(PAIR_CUTE));
    mvwaddstr(self.win, 0, x, s);
    wattroff(self.win, COLOR_PAIR(PAIR_CUTE));
  }
}

impl Meter for CommandBox {
  fn render(&mut self) {
    // draw header
    self.draw_header();

    wrefresh(self.win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut super::window::WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    wrefresh(win);

    CommandBox {
      width,
      height,
      win,
      command_buffer: "command".to_string(),
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.height = height;
    self.width = width;
    wresize(self.win, height, width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
