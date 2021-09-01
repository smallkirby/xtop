/*****

Implementation of ProcessMeter.
ProcessMeter shows the list of processes.

*******/

use crate::render::meter::Meter;
use crate::render::{meter, window};
use crate::resource::process;
use ncurses::*;

static PID_WIDTH: i32 = 6;
static CPU_WIDTH: i32 = 6;

#[derive(Debug)]
pub struct SubWins {
  pub pid_win: WINDOW,
  pub cpu_win: WINDOW,
  pub comm_win: WINDOW,
}

#[derive(Debug)]
pub struct ProcessMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  pub subwins: SubWins,
  pub process: Option<process::Process>,
}

impl ProcessMeter {
  pub fn set_proc(&mut self, proc: process::Process) {
    self.process = Some(proc);
  }
}

impl meter::Meter for ProcessMeter {
  fn render(&mut self) {
    let win = self.win;
    let subwins = &self.subwins;
    wclear(win);

    let proc = match self.process.as_ref() {
      Some(_proc) => _proc,
      None => {
        mvwprintw(
          subwins.comm_win,
          0,
          0,
          &format!("[ERROR] process not initialized."),
        );
        wrefresh(win);
        return;
      }
    };

    mvwprintw(subwins.pid_win, 0, 0, &format!("{:>6}", proc.pid));
    mvwprintw(subwins.cpu_win, 0, 0, &format!("{:>3.2}", proc.percent_cpu));
    mvwprintw(subwins.comm_win, 0, 0, &format!("{}", proc.cmdline));
    wrefresh(win);
  }

  fn init_meter(
    _parent: WINDOW,
    wm: &mut window::WinManager,
    _height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    let alloc_width = match width {
      Some(_w) => _w,
      None => wm.screen_width,
    };
    let (win, subwins) = create_meter_win(wm.processmeter_win.unwrap(), alloc_width, y, x);
    ProcessMeter {
      height: 1,
      width: alloc_width,
      win,
      subwins,
      process: None,
    }
  }

  fn resize(&mut self) {}
}

pub fn init_meters(parent: WINDOW, wm: &mut window::WinManager, height: i32) -> Vec<ProcessMeter> {
  let mut meters = vec![];
  let width = wm.screen_width;
  for i in 0..height {
    let meter = ProcessMeter::init_meter(parent, wm, Some(height), Some(width), i, 0);
    meters.push(meter);
  }

  meters
}

fn create_meter_win(parent: WINDOW, width: i32, y: i32, x: i32) -> (WINDOW, SubWins) {
  // create entire window for single process
  let win = derwin(parent, 1, width, y, x);

  // create sub windows
  let mut cur_x = 0;
  let pid_win = derwin(win, 1, PID_WIDTH, 0, cur_x);
  cur_x += PID_WIDTH + 1;
  let cpu_win = derwin(win, 1, CPU_WIDTH, 0, cur_x);
  cur_x += CPU_WIDTH + 1;
  let comm_win = derwin(win, 1, width - cur_x, 0, cur_x);

  wrefresh(win);
  (
    win,
    SubWins {
      cpu_win,
      pid_win,
      comm_win,
    },
  )
}
