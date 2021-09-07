/*****

Implementation of CPUGraph.
CPUGraph shows the transition of CPU usage.

*******/

use crate::render::{color, meter::*};
use crate::resource::cpu;
use crate::symbol::block::lv;
use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.

pub struct CPUGraph {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  history: Vec<f64>,  // ring-buffer for history of CPU usage
  cur_hist_ix: usize, // always points to newly recorded value of history
  max_percent: f64,
}

impl CPUGraph {
  pub fn set_cpu(&mut self, acpu: &cpu::CPU) {
    self.cur_hist_ix += 1;
    if self.cur_hist_ix >= MAXBUFSZ {
      self.cur_hist_ix %= MAXBUFSZ;
    }

    self.history[self.cur_hist_ix] = acpu.percent();
  }

  fn draw_single_bar(&mut self, bar: &String, y_bottom: i32, x: i32) {
    let win = self.win;
    for (i, c) in bar.chars().enumerate() {
      mvwaddstr(win, y_bottom - i as i32, x, &c.to_string());
    }
  }

  // returns latest history whose size is decided by self.width.
  // oldest entry is at index 0.
  fn get_recent_history(&self, size: usize) -> Vec<f64> {
    let mut res = vec![0.0; size];
    let start = self.cur_hist_ix;
    for i in (0..size).rev() {
      res[i] = self.history[(start + MAXBUFSZ - i) % MAXBUFSZ];
    }

    res.reverse();
    res
  }

  fn get_bar(&self, maxheight: i32, percent: f64) -> String {
    lv::get_bar(maxheight, percent / self.max_percent / 100.0)
  }

  fn update_upper_limit(&mut self, recent_hists: &Vec<f64>) {
    let max_percent = recent_hists.iter().fold(0.0 / 0.0, |a, b| b.max(a));
    self.max_percent = if max_percent >= 50.0 { 1.0 } else { 0.5 }
  }
}

impl Meter for CPUGraph {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    // draw bars
    let x_start = 3;
    let width = self.width - 2 - x_start;
    let height = self.height - 2;
    let y_bottom = height;

    let hists = self.get_recent_history(width as usize);
    let current_usage = hists.last().copied().unwrap();
    for (i, hist) in hists.iter().enumerate() {
      let bar = self.get_bar(height, *hist);
      self.draw_single_bar(&bar, y_bottom, x_start + i as i32 + 1);
    }

    self.update_upper_limit(&hists);
    // draw header
    mvwaddstr(win, 0, 1, &format!(" CPU Usage ({:>3.2}) ", current_usage));

    // draw y-axes
    mvwaddstr(win, 1, 1, &format!("{:>3}", self.max_percent * 100.0));
    mvwaddstr(
      win,
      self.height / 2,
      1,
      &format!("{:>3}", self.max_percent * 0.5 * 100.0),
    );

    wrefresh(win);
  }

  fn init_meter(
    _parent: ncurses::WINDOW,
    _wm: &mut super::window::WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    if height.is_none() || width.is_none() {
      panic!("height and width must be specified for CPUGraph::init_meter().");
    }
    let height = std::cmp::min(height.unwrap(), MAXBUFSZ as i32);
    let width = width.unwrap();
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(color::CPAIR::DEFAULT));
    wbkgd(
      win,
      ' ' as chtype | COLOR_PAIR(color::CPAIR::DEFAULT) as chtype,
    );
    box_(win, 0, 0);
    wrefresh(win);

    CPUGraph {
      width,
      height,
      win,
      history: vec![0.0; MAXBUFSZ],
      cur_hist_ix: 0,
      max_percent: 0.5,
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: Option<i32>, width: Option<i32>, y: i32, x: i32) {
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
