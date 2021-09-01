use crate::render::{cpumeter, meter::*, window::*};
use crate::resource::cpu;
use ncurses::*;

#[derive(Debug)]
pub struct CPUManager {
  pub cpumeters: Vec<cpumeter::CPUMeter>,
  pub height: i32,
  pub width: i32,
  win: WINDOW,
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
    let (_width, _height) = winsize_require(wm);
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
}

fn init_meters(parent: WINDOW, wm: &mut WinManager) -> Vec<cpumeter::CPUMeter> {
  let mut meters = vec![];
  let num_cpu = wm.plist.cpus.len();
  let width = wm.screen_width / 2;
  let height = 1;

  for i in 0..num_cpu {
    let (y, x) = pos_win_start(&wm.plist.cpus[i], width);
    let meter = cpumeter::CPUMeter::init_meter(parent, wm, Some(height), Some(width), y, x);
    meters.push(meter);
  }

  meters
}

fn winsize_require(wm: &WinManager) -> (i32, i32) {
  let width = wm.screen_width;
  let height = if wm.plist.cpus.len() % 2 == 0 {
    wm.plist.cpus.len() / 2
  } else {
    wm.plist.cpus.len() / 2 + 1
  } as i32;

  (width, height)
}

// returns the position where this CPUMeter's window starts, inside parent window.
// XXX the start position should be decided by class WindowManager
fn pos_win_start(cpu: &cpu::CPU, width: i32) -> (i32, i32) {
  let id = cpu.id;
  let x = if id % 2 == 0 { 0 } else { width };
  let y = id as i32 / 2;

  (y, x)
}
