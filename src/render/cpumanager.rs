/*****

Implementation of CPUManager.
CPUManager manages CPUMeters.

*******/

use crate::render::{color, cpumeter, executer::manager::*, meter::*};
use crate::resource::cpu;
use ncurses::*;

#[derive(Debug)]
pub struct CpuManager {
  pub cpumeters: Vec<cpumeter::CpuMeter>,
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
}

impl CpuManager {
  pub fn set_cpus(&mut self, cpus: &[cpu::Cpu]) {
    for (i, cpu) in cpus
      .iter()
      .enumerate()
      .take(std::cmp::min(self.cpumeters.len(), self.cpumeters.len()))
    {
      self.cpumeters[i].set_cpu(*cpu);
    }
  }
}

impl Meter for CpuManager {
  fn render(&mut self) {
    for i in 0..self.cpumeters.len() {
      self.cpumeters[i].render();
    }
  }

  fn init_meter(
    _parent: WINDOW,
    wm: &mut WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    // init entire window
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(color::cpair::DEFAULT));
    wbkgd(
      win,
      ' ' as chtype | COLOR_PAIR(color::cpair::DEFAULT) as chtype,
    );

    // init each windows of cpumeter inside parent window.
    let cpumeters = init_meters(win, wm, height, width);

    CpuManager {
      cpumeters,
      width,
      height,
      win,
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.width = width;
    self.height = height;

    wresize(self.win, self.height, self.width);
    werase(self.win);
    mvwin(self.win, y, x);
    // moving subwindows is not recommended (though i don't know why).
    // hence, destroy and re-create subwins.
    for i in 0..self.cpumeters.len() {
      let (y, x) = pos_win_start(i as u32, self.width / 2);
      self.cpumeters[i].recreate(self.win, 1, self.width / 2, y, x);
    }

    wrefresh(self.win);
  }

  fn handle_click(&mut self, _y: i32, _x: i32) {}
}

fn init_meters(
  parent: WINDOW,
  wm: &mut WinManager,
  height: i32,
  width: i32,
) -> Vec<cpumeter::CpuMeter> {
  let mut meters = vec![];
  let num_cpu = wm.plist.cpus.len();
  let width = width / 2;
  let height = 1;

  for i in 0..num_cpu {
    let (y, x) = pos_win_start(wm.plist.cpus[i].id, width);
    let meter = cpumeter::CpuMeter::init_meter(parent, wm, height, width, y, x);
    meters.push(meter);
  }

  meters
}

// `width` is a width of each cpumeter, not a screen-width (manager width).
fn pos_win_start(id: u32, width: i32) -> (i32, i32) {
  let x = if id % 2 == 0 { 0 } else { width };
  let y = id as i32 / 2;

  (y, x)
}
