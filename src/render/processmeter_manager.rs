/*****

Implementation of ProcessMeterManager.
ProcessMeterManager manages ProcessMeter and directly communicaste with WinManager.

*******/

use crate::render::{color::*, meter::*, processmeter::*, window::*};
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
  highlighted_pid: Option<i32>,
}

impl ProcessMeterManager {
  fn set_highlighted_pid(&mut self) {
    for i in 0..self.processmeters.len() {
      self.processmeters[i].highlighted_pid = self.highlighted_pid;
    }
  }

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
    mvwaddstr_color(header.pid_win, 0, 0, "PID", cpair::PAIR_HEAD);
    mvwaddstr_color(header.cpu_win, 0, 0, "CPU", cpair::PAIR_HEAD);
    mvwaddstr_color(header.comm_win, 0, 0, "COMM", cpair::PAIR_HEAD);
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
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    // entire window for ProcessMeters
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    // header sub-window
    let header_win = derwin(win, 1, width, 0, 0);
    let header_subwins = create_header_win(header_win, width, 0, 0); // XXX should hold subwins?
                                                                     // process meters
    let processmeters_win = derwin(win, height - 1, width, 1, 0);
    let processmeters = init_meters(processmeters_win, wm, height - 1);

    Self {
      height,
      width,
      win,
      header_win,
      header_subwins,
      processmeters_win,
      processmeters,
      sorted_procs: vec![],
      highlighted_pid: None,
    }
  }

  // it doesn NOT resize horizontally.
  // change the size of vertical size and height of processmeters.
  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    let old_height = self.height;
    self.width = width;
    self.height = height;

    // resize
    let proc_height = std::cmp::max(self.height - 1, 1);
    wresize(self.win, self.height, self.width);
    wresize(self.processmeters_win, proc_height, self.width);
    wresize(self.header_win, 1, self.width);
    self.header_subwins.resize(self.width);
    for i in 0..self.processmeters.len() {
      let mut x = 0;
      let mut y = 0;
      getbegyx(self.processmeters[i].win, &mut y, &mut x);
      self.processmeters[i].resize(self.processmeters_win, 1, self.width, y, x);
    }

    werase(self.win);
    werase(self.processmeters_win);
    mvwin(self.win, y, x); // XXX subwindows also moves?

    // if height becomes larger, delete current wins.
    if self.height > old_height {
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
    }

    // refresh all
    self.render();
  }

  fn handle_click(&mut self, y: i32, x: i32) {
    use crate::util::clamp;
    let meter_ix = clamp((y - 1) as f64, 0.0, self.processmeters.len() as f64) as usize;
    let pid = self.processmeters[meter_ix].process.as_ref().unwrap().pid;
    match self.highlighted_pid {
      Some(current_pid) => {
        if current_pid == pid {
          self.highlighted_pid = None;
        } else {
          self.highlighted_pid = Some(pid);
        }
      }
      None => self.highlighted_pid = Some(pid),
    }

    self.set_highlighted_pid();
    self.processmeters[meter_ix].handle_click(0, x);
    self.render();
  }
}

fn init_meters(parent: WINDOW, wm: &mut WinManager, height: i32) -> Vec<ProcessMeter> {
  let mut meters = vec![];
  let width = wm.screen_width;
  for i in 0..height {
    let meter = ProcessMeter::init_meter(parent, wm, height, width, i, 0);
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
