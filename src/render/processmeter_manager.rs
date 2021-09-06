/*****

Implementation of ProcessMeterManager.
ProcessMeterManager manages ProcessMeter and directly communicaste with WinManager.

*******/

use crate::render::{meter::*, processmeter::*, window::*};
use crate::resource::process;
use ncurses::*;

pub struct ProcessMeterManager {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  header_win: WINDOW,
  header_subwins: SubWins,
  processmeters_win: WINDOW,
  sorted_procs: Vec<process::Process>,
  processmeters: Vec<ProcessMeter>,
}

impl ProcessMeterManager {
  fn set_procs_meter(&mut self) {
    let proc_height = std::cmp::max(self.height - 1, 1) as usize;
    let num_meters = self.processmeters.len();
    let actual_height = std::cmp::min(proc_height, num_meters);
    for i in 0..actual_height {
      self.processmeters[i].set_proc(self.sorted_procs[i].clone());
    }
  }

  // the argument must be sorted.
  pub fn set_sorted_procs(&mut self, procs: Vec<process::Process>) {
    self.sorted_procs = procs;
    self.set_procs_meter();
  }

  // XXX should align in center
  fn render_header(&self) {
    let header = &self.header_subwins;
    mvwaddstr(header.pid_win, 0, 0, "PID");
    mvwaddstr(header.cpu_win, 0, 0, "CPU");
    mvwaddstr(header.comm_win, 0, 0, "COMM");
    wrefresh(header.pid_win);
    wrefresh(header.cpu_win);
    wrefresh(header.comm_win);
  }
}

impl Meter for ProcessMeterManager {
  fn render(&mut self) {
    self.render_header();
    for i in 0..self.processmeters.len() {
      self.processmeters[i].render();
    }
  }

  fn init_meter(
    _parent: WINDOW,
    wm: &mut WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    let default_width = wm.screen_width;
    let default_height = 2;
    let width = match width {
      Some(w) => w,
      None => default_width,
    };
    let height = match height {
      Some(h) => h,
      None => default_height,
    };
    // entire window for ProcessMeters
    let win = newwin(height, width, y, x);
    // header sub-window
    let header_win = derwin(win, 1, width, 0, 0);
    let header_subwins = create_header_win(header_win, width, 0, 0); // XXX should hold subwins?
                                                                     // process meters
    let processmeters_win = derwin(win, height - 1, width, 1, 0);
    let processmeters = init_meters(processmeters_win, wm, height - 1);
    refresh();

    Self {
      height,
      width,
      win,
      header_win,
      header_subwins,
      processmeters_win,
      processmeters,
      sorted_procs: vec![],
    }
  }

  // it doesn NOT resize horizontally.
  // change the size of vertical size and height of processmeters.
  fn resize(&mut self, parent: WINDOW, height: Option<i32>, width: Option<i32>, y: i32, x: i32) {
    let old_height = self.height;
    self.width = match width {
      Some(w) => w,
      None => self.width,
    };
    self.height = match height {
      Some(h) => h,
      None => self.height,
    };

    // resize, erase, and move.
    let proc_height = std::cmp::max(self.height - 1, 1);
    wresize(self.win, self.height, self.width);
    wresize(self.processmeters_win, proc_height, self.width);
    if self.height <= old_height {
      return;
    }
    werase(self.win);
    werase(self.processmeters_win);
    mvwin(self.win, y, x); // XXX subwindows also moves?

    // delete all processmeters windows. create new ones.
    for i in 0..self.processmeters.len() {
      self.processmeters[i as usize].del();
    }
    delwin(self.processmeters_win);

    // create new one
    self.processmeters_win = derwin(self.win, proc_height, self.width, 1, 0);
    self.processmeters = _init_meters(self.processmeters_win, proc_height, self.width);

    // update
    self.set_procs_meter();
    self.render();
  }
}

fn init_meters(parent: WINDOW, wm: &mut WinManager, height: i32) -> Vec<ProcessMeter> {
  let mut meters = vec![];
  let width = wm.screen_width;
  for i in 0..height {
    let meter = ProcessMeter::init_meter(parent, wm, Some(height), Some(width), i, 0);
    meters.push(meter);
  }

  meters
}

// XXX too dirty
fn _init_meters(parent: WINDOW, height: i32, width: i32) -> Vec<ProcessMeter> {
  let mut meters = vec![];
  for i in 0..height {
    let meter = _init_meter(parent, width, i, 0);
    meters.push(meter);
  }

  meters
}
