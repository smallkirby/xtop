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
  result_buffer: String,
  is_active: bool,
}

impl CommandBox {
  pub fn set_result(&mut self, result: &str) {
    self.result_buffer = result.into();
    self.render();
  }

  pub fn start_input(&mut self) {
    self.is_active = true;
    self.command_buffer.clear();
    self.result_buffer.clear();
    self.render();
  }

  pub fn do_enter(&mut self) -> String {
    let command = self.command_buffer.clone();
    self.command_buffer.clear();
    self.is_active = false;
    self.render();

    command
  }

  pub fn addstr(&mut self, s: &str) {
    self.result_buffer.clear();
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
    let mut total_len = 0;

    let s = " ❦ ";
    if self.is_active {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD() | A_BLINK());
      mvwaddstr(self.win, 0, 0, s);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD() | A_BLINK());
    } else {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
      mvwaddstr(self.win, 0, 0, s);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
    }
    total_len += s.chars().count();

    let s = "command";
    mvwaddstr(self.win, 0, total_len as i32, s);
    total_len += s.chars().count();

    let s = " ❦  ";
    if self.is_active {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD() | A_BLINK());
      mvwaddstr(self.win, 0, total_len as i32, s);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD() | A_BLINK());
    } else {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
      mvwaddstr(self.win, 0, total_len as i32, s);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
    }
    total_len += s.chars().count();

    total_len
  }
}

impl Meter for CommandBox {
  fn render(&mut self) {
    use crate::render::color::cpair::*;
    let mut x = 0;
    werase(self.win);

    // draw header
    x += self.draw_header() as i32;

    if self.result_buffer.is_empty() {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
      mvwaddstr(self.win, 0, x, &self.command_buffer);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
    } else {
      wattron(self.win, COLOR_PAIR(PAIR_DARK) | A_BOLD());
      mvwaddstr(self.win, 0, x, &self.result_buffer);
      wattroff(self.win, COLOR_PAIR(PAIR_DARK) | A_BOLD());
    }

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
      command_buffer: "".into(),
      result_buffer: "".into(),
      is_active: false,
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
