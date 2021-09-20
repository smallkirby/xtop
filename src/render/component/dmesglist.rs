/*****

Implementation of DmesgList.
DmesgList just shows the output from /dev/kmsg

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};
use crate::resource::dmesg;
use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.

pub struct DmesgList {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  history: Vec<Option<dmesg::KmsgLine>>, // ring-buffer for history. 0 is latest.
  cur_hist_ix: usize,                    // always points to newly recorded value of history
}

impl DmesgList {
  pub fn set_dmesg(&mut self, dmesgs: Vec<dmesg::KmsgLine>) {
    for i in 0..self.history.len() {
      self.history[i] = None;
    }
    for (i, dmesg) in dmesgs.iter().rev().enumerate() {
      if i >= MAXBUFSZ {
        break;
      }
      self.history[i] = Some(dmesg.clone());
    }
  }

  // 0 is latest.
  // returned size can be smaller than `size`.
  fn get_recent_history(&self, size: usize) -> Vec<&dmesg::KmsgLine> {
    let mut res = vec![];
    let start = self.cur_hist_ix;
    eprintln!("{}", size);
    for i in (0..size).rev() {
      match &self.history[(start + MAXBUFSZ - i) % MAXBUFSZ] {
        Some(h) => res.push(h),
        None => continue,
      };
    }

    res
  }
}

impl Meter for DmesgList {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    // write logs
    let x_start = 1;
    let width = self.width - x_start - 1;
    let height = self.height - 2;
    if height < 0 {
      return;
    }
    let y_bottom = height;
    let hists = self.get_recent_history(height as usize);
    for (i, hist) in hists.iter().enumerate() {
      let mut cx = x_start;
      for c in hist.log.chars() {
        if cx >= width {
          break;
        }
        mvwaddstr(self.win, y_bottom - i as i32, cx, &c.to_string());
        cx += 1;
      }
    }

    // draw header
    mvwaddstr_color(win, 0, 1, " dmesg ", cpair::PAIR_HEAD);

    wrefresh(win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut WinManager,
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

    DmesgList {
      width,
      height,
      win,
      history: vec![None; MAXBUFSZ],
      cur_hist_ix: 0,
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
