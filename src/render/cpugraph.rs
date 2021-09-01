/*****

Implementation of CPUGraph.
CPUGraph shows the transition of CPU usage.

*******/

use crate::render::meter::*;
use crate::resource::cpu;
use crate::symbol::block::lv;
use ncurses::ll::mvwaddch;
use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.

pub struct CPUGraph {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  history: Vec<f64>,  // ring-buffer for history of CPU usage
  cur_hist_ix: usize, // always points to newly recorded value of history
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
}

impl Meter for CPUGraph {
  fn render(&mut self) {
    let win = self.win;
    let width = self.width - 2;
    let height = self.height - 2;
    let y_bottom = height;

    let hists = self.get_recent_history(width as usize);
    for (i, hist) in hists.into_iter().enumerate() {
      let bar = get_bar(height, hist);
      self.draw_single_bar(&bar, y_bottom, i as i32 + 1);
    }
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
    box_(win, 0, 0);
    wrefresh(win);

    CPUGraph {
      width,
      height,
      win,
      history: vec![0.0; MAXBUFSZ],
      cur_hist_ix: 0,
    }
  }

  fn resize(&mut self) {
    todo!()
  }
}

fn get_bar(maxheight: i32, percent: f64) -> String {
  // XXX set upper limit as 50% for now. it should be dynamically decided.
  lv::get_bar(maxheight, percent / 0.5 / 100.0)
}
