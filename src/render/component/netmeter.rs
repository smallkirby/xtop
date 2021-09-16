/*****

Implementation of NetMeter.
NetGraph shows the transition of net usage.

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};
use crate::resource::net;
use crate::symbol::brail::b32::*;
use crate::util::{DataSize, DataUnit::*};

use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.
static THRESHOLD: u64 = 500;

pub struct NetMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  history: Vec<(DataSize<u64>, DataSize<u64>)>, // ring-buffer for history of (rx, tx) [KBytes]
  cur_hist_ix: usize,                           // always points to newly recorded value of history
  max_rx_kb: DataSize<u64>,
  max_tx_kb: DataSize<u64>,
  total_rx: DataSize<u64>, // Bytes
  total_tx: DataSize<u64>, // Bytes
  diff_rx: DataSize<u64>,  // Bytes
  diff_tx: DataSize<u64>,  // Bytes
}

impl NetMeter {
  fn update_upper_limit(&mut self, recent_hists: &[(DataSize<u64>, DataSize<u64>)]) {
    let recent_rx: Vec<u64> = recent_hists.iter().map(|(r, _t)| r.convert(Kb)).collect();
    let recent_tx: Vec<u64> = recent_hists.iter().map(|(_r, t)| t.convert(Kb)).collect();
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

    self.max_rx_kb = DataSize::new(max_kb_rx, Kb);
    self.max_tx_kb = DataSize::new(max_kb_tx, Kb);
  }

  // returns latest history whose size is decided by self.width.
  // oldest entry is at index 0.
  fn get_recent_history(&self, size: usize) -> Vec<(DataSize<u64>, DataSize<u64>)> {
    let mut res = vec![(DataSize::new(0, Kb), DataSize::new(0, Kb)); size];
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

    if self.total_rx.val == 0 || self.total_tx.val == 0 {
      self.total_rx = DataSize::new(total_rx, B);
      self.total_tx = DataSize::new(total_tx, B);
    }
    self.diff_rx = DataSize::new(total_rx - self.total_rx.convert(B), B);
    self.diff_tx = DataSize::new(total_tx - self.total_tx.convert(B), B);
    self.total_rx = DataSize::new(total_rx, B);
    self.total_tx = DataSize::new(total_tx, B);

    self.history[self.cur_hist_ix] = (
      DataSize::new(self.diff_rx.convert(Kb), Kb),
      DataSize::new(self.diff_tx.convert(Kb), Kb),
    );
  }

  fn draw_single_col(&self, bar: &[Cc], y_bottom: i32, x: i32) {
    // draw from bottom.
    for (i, cc) in bar.iter().enumerate() {
      mvwaddstr_color(self.win, y_bottom - i as i32, x, &cc.ch.to_string(), cc.co);
    }
  }

  fn draw_yaxes(&self) {
    let win = self.win;
    let rx_unit = if self.max_rx_kb.convert(Mb) >= 1 {
      Mb
    } else {
      Kb
    };
    let tx_unit = if self.max_tx_kb.convert(Mb) >= 1 {
      Mb
    } else {
      Kb
    };

    // left y-axe (rx)
    let s = &format!("{:>3.0}", self.max_rx_kb.convert(rx_unit));
    mvwaddstr(win, 1, 1, s);
    let s = &format!("{:>3.0}", self.max_rx_kb.convert(rx_unit) as f64 * 0.5);
    mvwaddstr(win, self.height / 2, 1, s);
    let s = &format!("[{}]", rx_unit);
    mvwaddstr(win, self.height - 2, 1, s);

    // right y-axe (tx)
    let s = &format!("{:>3.0}", self.max_tx_kb.convert(tx_unit));
    mvwaddstr_color(win, 1, self.width - 1 - s.len() as i32, s, cpair::PAIR_COMM);
    let s = &format!("{:>3.0}", self.max_tx_kb.convert(tx_unit) as f64 * 0.5);
    mvwaddstr_color(
      win,
      self.height / 2,
      self.width - 1 - s.len() as i32,
      s,
      cpair::PAIR_COMM,
    );
    let s = &format!("[{}]", tx_unit);
    mvwaddstr(win, self.height - 2, self.width - 1 - s.len() as i32, s);
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
    let rx_hists: Vec<f64> = hists
      .iter()
      .map(|(rx, _tx)| rx.convert(Kb) as f64)
      .collect();
    let tx_hists: Vec<f64> = hists
      .iter()
      .map(|(_rx, tx)| tx.convert(Kb) as f64)
      .collect();
    let brails = get_brails_complement_2sep_axes_color(
      height - 1,
      (0.0, self.max_rx_kb.convert(Kb) as f64),
      (0.0, self.max_tx_kb.convert(Kb) as f64),
      (rx_hists, cpair::DEFAULT),
      (tx_hists, cpair::PAIR_COMM),
    );

    for (i, col) in brails.iter().enumerate() {
      self.draw_single_col(col, y_bottom, x_start + i as i32 + 1);
    }

    // draw header
    let rx_kb = self.diff_rx.convert(Kb) as f64 / 1024.0;
    let tx_kb = self.diff_tx.convert(Kb) as f64 / 1024.0;
    mvwaddstr_color(
      win,
      0,
      1,
      &format!(" Net ({:>5.02} / {:>5.02} kB/s) ", rx_kb, tx_kb),
      cpair::PAIR_HEAD,
    );

    // draw y-axes
    self.draw_yaxes();

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
      history: vec![(DataSize::new(0, Kb), DataSize::new(0, Kb)); MAXBUFSZ],
      cur_hist_ix: 0,
      max_rx_kb: DataSize::new(THRESHOLD, Kb),
      max_tx_kb: DataSize::new(THRESHOLD, Kb),
      total_rx: DataSize::new(0, Kb),
      total_tx: DataSize::new(0, Kb),
      diff_rx: DataSize::new(0, Kb),
      diff_tx: DataSize::new(0, Kb),
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
