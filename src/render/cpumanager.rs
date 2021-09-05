/*****

Implementation of CPUManager.
CPUManager manages CPUMeters.

*******/

use crate::render::{cpumeter, meter::*, window::*};
use crate::resource::cpu;
use ncurses::*;

#[derive(Debug)]
pub struct CPUManager {
  pub cpumeters: Vec<cpumeter::CPUMeter>,
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
}

impl CPUManager {
  pub fn set_cpus(&mut self, cpus: &Vec<cpu::CPU>) {
    for i in 0..std::cmp::min(self.cpumeters.len(), self.cpumeters.len()) {
      self.cpumeters[i].set_cpu(cpus[i].clone());
    }
  }
}

impl Meter for CPUManager {
  fn render(&mut self) {
    for i in 0..self.cpumeters.len() {
      self.cpumeters[i].render();
    }
  }

  fn init_meter(
    _parent: WINDOW,
    wm: &mut super::window::WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    // init entire window
    let (_width, _height) = winsize_require(wm.screen_width, wm.plist.cpus.len());
    let width = match width {
      Some(w) => w,
      None => _width,
    };
    let height = match height {
      Some(h) => h,
      None => _height,
    };
    let win = newwin(height, width, y, x);

    // init each windows of cpumeter inside parent window.
    let cpumeters = init_meters(win, wm);
    refresh();

    CPUManager {
      cpumeters,
      width,
      height,
      win,
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
    for i in 0..self.cpumeters.len() {
      let (y, x) = pos_win_start(i as u32, self.width / 2);
      self.cpumeters[i].resize(self.win, None, Some(self.width / 2), y, x);
    }

    self.render();
    wrefresh(self.win);
  }
}

fn init_meters(parent: WINDOW, wm: &mut WinManager) -> Vec<cpumeter::CPUMeter> {
  let mut meters = vec![];
  let num_cpu = wm.plist.cpus.len();
  let width = wm.screen_width / 2;
  let height = 1;

  for i in 0..num_cpu {
    let (y, x) = pos_win_start(wm.plist.cpus[i].id, width);
    let meter = cpumeter::CPUMeter::init_meter(parent, wm, Some(height), Some(width), y, x);
    meters.push(meter);
  }

  meters
}

fn winsize_require(screen_width: i32, num_cpu: usize) -> (i32, i32) {
  let height = if num_cpu % 2 == 0 {
    num_cpu / 2
  } else {
    num_cpu / 2 + 1
  } as i32;

  (screen_width, height)
}

// `width` is a width of each cpumeter, not a screen-width (manager width).
fn pos_win_start(id: u32, width: i32) -> (i32, i32) {
  let x = if id % 2 == 0 { 0 } else { width };
  let y = id as i32 / 2;

  (y, x)
}
