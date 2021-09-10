/*****

Implementation of MemMeter.
MemMeter shows the statistics of memory usage.

*******/

use crate::render::color::*;
use crate::resource::mem;
use crate::symbol::brail::b32;
use ncurses::*;

use crate::render::{meter::Meter, window::WinManager};

static UNIT_MB: u64 = 1024;
#[allow(dead_code)]
static UNIT_GB: u64 = UNIT_MB * 1024;

static MAXBUFSZ: usize = 1000; // XXX should decide dynamically.

pub struct MemMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  usage: Option<mem::MemInfo>,
  history: Vec<f64>,  // ring-buffer for history of memory usage.
  cur_hist_ix: usize, // always points to newly recorded value of history
  max_percent: f64,   // [0.0, 1.0]
}

impl MemMeter {
  pub fn set_usage(&mut self, usage: &mem::MemInfo) {
    {
      let percent = usage.used as f64 / usage.total as f64;

      self.cur_hist_ix += 1;
      if self.cur_hist_ix >= MAXBUFSZ {
        self.cur_hist_ix %= MAXBUFSZ; // XXX buggy???
      }
      self.history[self.cur_hist_ix] = percent;
    }
    self.usage = Some(usage.clone());
  }

  // returns latest history whose size is decided by self.width.
  // oldest entry is at index 0.
  fn get_recent_history(&self, size: usize) -> Vec<f64> {
    let size = if size > MAXBUFSZ { MAXBUFSZ } else { size };
    let mut res = vec![0.0; size];
    let start = self.cur_hist_ix;
    for i in (0..size).rev() {
      res[i] = self.history[(start + MAXBUFSZ - i) % MAXBUFSZ];
    }

    res.reverse();
    res
  }

  fn draw_single_col(&self, bar: &str, y_bottom: i32, x: i32) {
    // draw from bottom.
    for (i, c) in bar.chars().enumerate() {
      mvwaddstr(self.win, y_bottom - i as i32, x, &c.to_string());
    }
  }

  fn update_upper_limit(&mut self, recent_hists: &[f64]) {
    let max_percent = recent_hists.iter().fold(0.0, |a, b| b.max(a));
    self.max_percent = if max_percent >= 0.5 { 1.0 } else { 0.5 };
  }
}

impl Meter for MemMeter {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    let mut cy = 1;
    let used_percent = {
      let usage = self.usage.as_ref().unwrap();

      // draw data
      let s = &format!("total: {:>7} MB", usage.total / UNIT_MB);
      let x = self.width - s.len() as i32 - 2;
      mvwaddstr(win, cy, x, s);
      cy += 1;
      let s = &format!("avail: {:>7} MB", usage.avail / UNIT_MB);
      let x = self.width - s.len() as i32 - 2;
      mvwaddstr(win, cy, x, s);
      cy += 1;
      let s = &format!("used : {:>7} MB", usage.used / UNIT_MB);
      let x = self.width - s.len() as i32 - 2;
      mvwaddstr(win, cy, x, s);
      cy += 1;

      (usage.used as f64 / usage.total as f64) * 100.0
    };

    // draw brails
    let x0 = 4;
    let width = self.width - 1 - x0;
    let height = self.height - 1 - cy;
    let hists = self.get_recent_history(width as usize * 2);
    self.update_upper_limit(&hists);

    let brails = b32::get_brails(height, 0.0, self.max_percent, hists);
    for (i, brail) in brails.iter().enumerate() {
      self.draw_single_col(brail, cy + height - 1, x0 + i as i32);
    }

    // draw header
    mvwaddstr_color(
      win,
      0,
      1,
      &format!(" Memory ({:>3.2} %) ", used_percent),
      cpair::PAIR_HEAD,
    );

    // draw y-axes
    mvwaddstr(win, cy, 1, &format!("{:>3}", self.max_percent * 100.0));
    mvwaddstr(
      win,
      cy + height / 2,
      1,
      &format!("{:>3}", self.max_percent * 0.5 * 100.0),
    );

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
      history: vec![0.0; MAXBUFSZ],
      cur_hist_ix: 0,
      max_percent: 1.0,
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
