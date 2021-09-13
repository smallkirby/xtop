/*********

Implementation of CommandBox.
CommandBox shows the box to input user input.
And it shows help box.

*********/

use crate::command::commander;
use crate::render::{color::*, meter::*};

use ncurses::*;

pub struct CommandBox {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  complete_win: Option<WINDOW>,
  command_buffer: String,
  result_buffer: String,
  is_active: bool,
  completions: Vec<String>,
}

impl CommandBox {
  pub fn set_result(&mut self, result: &str) {
    self.result_buffer = result.into();
    self.render();
  }

  pub fn start_input(&mut self, co: &mut commander::Commander) {
    self.is_active = true;
    self.command_buffer.clear();
    self.result_buffer.clear();

    self.completions = co.complete(" ");
    self.render();
  }

  pub fn do_enter(&mut self) -> String {
    let command = self.command_buffer.clone();
    self.command_buffer.clear();
    self.is_active = false;
    self.completions = vec![];
    self.render();

    command
  }

  pub fn render_usage(&mut self) {
    let comps = &self.completions;
    if comps.is_empty() {
      return;
    }

    // calculate where to place the window. (+2 is for boxing)
    let needed_height = comps.len() as i32 + 2;
    let needed_width = 40 + 2; // XXX
    let mut x0 = 0;
    let mut y0 = 0;
    getbegyx(self.win, &mut y0, &mut x0);
    x0 += 12;
    y0 -= needed_height;

    // delete current completion window
    if let Some(complete_win) = self.complete_win {
      werase(complete_win);
      delwin(complete_win);
    }

    // create new completion window
    let win = newwin(needed_height, needed_width, y0, x0);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    self.complete_win = Some(win);

    // render each completions
    for (i, comp) in self.completions.iter().enumerate() {
      mvwaddstr(win, i as i32 + 1, 1, comp);
    }
    wrefresh(win);
  }

  pub fn addstr(&mut self, s: &str, co: &mut commander::Commander) {
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
    self.completions = co.complete(&self.command_buffer);
    self.render();
    wrefresh(self.win);
  }

  fn render_header(&self) -> usize {
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
    x += self.render_header() as i32;

    if self.result_buffer.is_empty() {
      wattron(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
      mvwaddstr(self.win, 0, x, &self.command_buffer);
      wattroff(self.win, COLOR_PAIR(PAIR_CUTE) | A_BOLD());
    } else {
      wattron(self.win, COLOR_PAIR(PAIR_DARK) | A_BOLD());
      mvwaddstr(self.win, 0, x, &self.result_buffer);
      wattroff(self.win, COLOR_PAIR(PAIR_DARK) | A_BOLD());
    }

    // render completion
    self.render_usage();

    wrefresh(self.win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut super::executer::manager::WinManager,
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
      complete_win: None,
      command_buffer: "".into(),
      result_buffer: "".into(),
      is_active: false,
      completions: vec![],
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
