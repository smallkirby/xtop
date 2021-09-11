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
  pub fn do_enter(&mut self) -> String {
    let command = self.command_buffer.clone();
    self.command_buffer.clear();
    self.render();

    command
  }

  pub fn addstr(&mut self, s: &str) {
    match s {
      // back space
      "\x08" => {
        if !self.command_buffer.is_empty() {
          self.command_buffer = self.command_buffer[0..(self.command_buffer.len() - 1)].to_string();
        }
      }
      // any other key
      _ => {
        self.command_buffer += s;
      }
    }
    self.render();
    wrefresh(self.win);
  }

  fn draw_header(&self) -> usize {
    use crate::render::color::cpair::*;

    let s = "  ❦ command ❦";
    wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
    mvwaddstr(self.win, 0, 0, s);
    wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());

    s.len()
  }
}

impl Meter for CommandBox {
  fn render(&mut self) {
    use crate::render::color::cpair::*;
    let mut x = 0;
    werase(self.win);

    // draw header
    x += self.draw_header() as i32;

    wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());

    mvwaddstr(self.win, 0, x, &self.command_buffer);

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
      command_buffer: "".to_string(),
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
