/*****

Implementation of NetMeter.
NetGraph shows the transition of net usage.

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};
use crate::resource::net;
use crate::symbol::brail::b32::*;

use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.
static THRESHOLD: u64 = 500;

pub struct NetMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  history: Vec<(u64, u64)>, // ring-buffer for history of (rx, tx) [KBytes]
  cur_hist_ix: usize,       // always points to newly recorded value of history
  max_kb: u64,
  total_rx: u64, // Bytes
  total_tx: u64, // Bytes
  diff_rx: u64,  // Bytes
  diff_tx: u64,  // Bytes
}

impl NetMeter {
  fn update_upper_limit(&mut self, recent_hists: &[(u64, u64)]) {
    let recent_rx: Vec<u64> = recent_hists.iter().map(|(r, _t)| *r).collect();
    let recent_tx: Vec<u64> = recent_hists.iter().map(|(_r, t)| *t).collect();
    let max_rx = recent_rx.into_iter().fold(0_u64, |a, b| b.max(a));
    let max_tx = recent_tx.into_iter().fold(0_u64, |a, b| b.max(a));
    let max_kb_rx = if max_rx > THRESHOLD {
      (max_rx + THRESHOLD) / THRESHOLD * THRESHOLD
    } else {
      THRESHOLD
    };
    let max_kb_tx = if max_tx > THRESHOLD {
      (max_tx + THRESHOLD) / THRESHOLD * THRESHOLD
    } else {
      THRESHOLD
    };

    self.max_kb = std::cmp::max(max_kb_rx, max_kb_tx);
  }

  // returns latest history whose size is decided by self.width.
  // oldest entry is at index 0.
  fn get_recent_history(&self, size: usize) -> Vec<(u64, u64)> {
    let mut res = vec![(0, 0); size];
    let start = self.cur_hist_ix;
    for i in (0..size).rev() {
      res[i] = self.history[(start + MAXBUFSZ - i) % MAXBUFSZ];
    }

    res.reverse();
    res
  }

  pub fn set_statistics(&mut self, statistics: &[net::NetStatistics]) {
    self.cur_hist_ix += 1;
    if self.cur_hist_ix >= MAXBUFSZ {
      self.cur_hist_ix %= MAXBUFSZ;
    }

    let mut total_rx = 0;
    let mut total_tx = 0;
    for statistic in statistics {
      total_rx += statistic.rx_bytes;
      total_tx += statistic.tx_bytes;
    }

    if self.total_rx == 0 || self.total_tx == 0 {
      self.total_rx = total_rx;
      self.total_tx = total_tx;
    }
    self.diff_rx = total_rx - self.total_rx;
    self.diff_tx = total_tx - self.total_tx;
    self.total_rx = total_rx;
    self.total_tx = total_tx;

    self.history[self.cur_hist_ix] = (self.diff_rx / 1024, self.diff_tx / 1024);
  }

  fn draw_single_col(&self, bar: &[Cc], y_bottom: i32, x: i32) {
    // draw from bottom.
    for (i, cc) in bar.iter().enumerate() {
      mvwaddstr_color(self.win, y_bottom - i as i32, x, &cc.ch.to_string(), cc.co);
    }
  }
}

impl Meter for NetMeter {
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
    self.update_upper_limit(&hists);
    let rx_hists: Vec<f64> = hists.iter().map(|(rx, _tx)| *rx as f64).collect();
    let tx_hists: Vec<f64> = hists.iter().map(|(_rx, tx)| *tx as f64).collect();
    let brails = get_brails_complement_2axes_color(
      height - 1,
      0.0,
      self.max_kb as f64,
      (rx_hists, cpair::DEFAULT),
      (tx_hists, cpair::PAIR_COMM),
    );

    for (i, col) in brails.iter().enumerate() {
      self.draw_single_col(col, y_bottom, x_start + i as i32 + 1);
    }

    // draw header
    let rx_kb = self.diff_rx as f64 / 1024.0;
    let tx_kb = self.diff_tx as f64 / 1024.0;
    mvwaddstr_color(
      win,
      0,
      1,
      &format!(" Net ({:>5.02} / {:>5.02} kB/s) ", rx_kb, tx_kb),
      cpair::PAIR_HEAD,
    );

    // draw y-axes
    mvwaddstr(win, 1, 1, &format!("{:>5}", self.max_kb));
    mvwaddstr(
      win,
      self.height / 2,
      1,
      &format!("{:>5}", (self.max_kb as f64 * 0.5) as u64),
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
    let height = std::cmp::min(height, MAXBUFSZ as i32);
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    wrefresh(win);

    NetMeter {
      width,
      height,
      win,
      history: vec![(0, 0); MAXBUFSZ],
      cur_hist_ix: 0,
      max_kb: 1000,
      total_rx: 0,
      total_tx: 0,
      diff_rx: 0,
      diff_tx: 0,
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
