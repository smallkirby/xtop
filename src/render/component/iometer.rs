/*****

Implementation of IoMeter.
IoMeter shows the IO usages.

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};
use crate::resource::disk;
use crate::symbol::brail::b32::*;

use ncurses::*;

static MAXBUFSZ: usize = 300; // XXX should decide dynamically.
static THRESHOLD: u64 = 500;

pub struct IoMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  cur_hist_ix: usize,
  history: Vec<(f64, f64)>, // ring-buffer for history of (R[kB/s], W[kB/s])
  tps: f64,                 // current TPS (# of transfer requests toward any of device per sec.)
  current_stat: Option<disk::DiskStat>,
  max_kb: u64,
}

impl IoMeter {
  pub fn set_statistics(&mut self, statistics: Vec<disk::DiskStat>, update_interval: f64) {
    self.cur_hist_ix += 1;
    if self.cur_hist_ix >= MAXBUFSZ {
      self.cur_hist_ix %= MAXBUFSZ;
    }

    // add values of all interfaces
    let mut sum = disk::DiskStat {
      ..Default::default()
    };
    for statistic in statistics.into_iter() {
      sum += statistic;
    }

    // calculate and update values
    let (r_kb, w_kb) = match self.current_stat.as_ref() {
      Some(cur) => {
        self.tps = sum.tps(cur, update_interval);
        (
          sum.kb_read_persec(cur, update_interval),
          sum.kb_write_persec(cur, update_interval),
        )
      }
      None => {
        self.tps = 0.0;
        (0.0, 0.0)
      }
    };
    self.history[self.cur_hist_ix] = (r_kb, w_kb);

    // save current statistic for later calculation
    self.current_stat = Some(sum);
  }

  fn draw_header(&self, y: i32, x: i32) {
    let (r_kb, w_kb) = self.history[self.cur_hist_ix];
    let s = &format!(
      " IO ({:>2.2} tps : {:>2.2} / {:>2.2} kB/s) ",
      self.tps, r_kb, w_kb
    );
    mvwaddstr_color(self.win, y, x, s, cpair::PAIR_HEAD);
  }

  // returns latest history whose size is decided by `size`.
  // oldest entry is at index 0.
  fn get_recent_history(&self, size: usize) -> Vec<(f64, f64)> {
    let mut res = vec![(0.0, 0.0); size];
    let start = self.cur_hist_ix;
    for i in (0..size).rev() {
      res[i] = self.history[(start + MAXBUFSZ - i) % MAXBUFSZ];
    }

    res.reverse();
    res
  }

  fn update_upper_limit(&mut self, recent_hists: &[(f64, f64)]) {
    // use u64 instead of f64
    let recent_rd: Vec<u64> = recent_hists.iter().map(|(r, _w)| *r as u64).collect();
    let recent_wr: Vec<u64> = recent_hists.iter().map(|(_r, w)| *w as u64).collect();
    let max_rd = recent_rd.into_iter().fold(0_u64, |a, b| b.max(a));
    let max_wr = recent_wr.into_iter().fold(0_u64, |a, b| b.max(a));
    let max_kb_rd = if max_rd > THRESHOLD {
      (max_rd + THRESHOLD) / THRESHOLD * THRESHOLD
    } else {
      THRESHOLD
    };
    let max_kb_wr = if max_wr > THRESHOLD {
      (max_wr + THRESHOLD) / THRESHOLD * THRESHOLD
    } else {
      THRESHOLD
    };

    self.max_kb = std::cmp::max(max_kb_rd, max_kb_wr);
  }

  fn draw_single_col(&self, bar: &[Cc], y_bottom: i32, x: i32) {
    // draw from bottom.
    for (i, cc) in bar.iter().enumerate() {
      mvwaddstr_color(self.win, y_bottom - i as i32, x, &cc.ch.to_string(), cc.co);
    }
  }
}

impl Meter for IoMeter {
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
    let rd_hists: Vec<f64> = hists.iter().map(|(rd, _wr)| *rd as f64).collect();
    let wr_hists: Vec<f64> = hists.iter().map(|(_rd, wr)| *wr as f64).collect();
    let brails = get_brails_complement_2axes_color(
      height - 1,
      0.0,
      self.max_kb as f64,
      (rd_hists, cpair::DEFAULT),
      (wr_hists, cpair::PAIR_COMM),
    );

    for (i, col) in brails.iter().enumerate() {
      self.draw_single_col(col, y_bottom, x_start + i as i32 + 1);
    }

    // draw y-axes
    mvwaddstr(win, 1, 1, &format!("{:>5}", self.max_kb));
    mvwaddstr(
      win,
      self.height / 2,
      1,
      &format!("{:>5}", (self.max_kb as f64 * 0.5) as u64),
    );

    // draw header
    self.draw_header(0, 1);

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

    IoMeter {
      width,
      height,
      win,
      current_stat: None,
      tps: 0.0,
      cur_hist_ix: 0,
      history: vec![(0.0, 0.0); MAXBUFSZ],
      max_kb: THRESHOLD,
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
