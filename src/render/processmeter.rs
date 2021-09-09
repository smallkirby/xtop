/*****

Implementation of ProcessMeter.
ProcessMeter shows the list of processes.

*******/

use crate::render::{color, meter, window};
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

  pub fn del(&mut self) {
    delwin(self.win);
  }

  // XXX this is not comm, it's cmdline.
  fn render_comm(&self, full_comm: &str) {
    use crate::render::color::{cpair, mvwaddstr_color};
    use crate::util::*;

    let tokens: Vec<&str> = full_comm.split(" ").collect();
    let comm_win = self.subwins.comm_win;
    let (exe_path_dir, exe_path_file) = get_dir_file(tokens[0]);
    let args = if tokens.len() > 1 {
      tokens[1..].join(" ")
    } else {
      "".into()
    };

    let mut cur_x = 0;
    mvwprintw(comm_win, 0, cur_x, &format!("{}", exe_path_dir));
    cur_x += exe_path_dir.len() as i32;
    mvwaddstr_color(
      comm_win,
      0,
      cur_x,
      &format!("{}", exe_path_file),
      cpair::PAIR_COMM,
    );
    cur_x += exe_path_file.len() as i32 + 1;
    mvwprintw(comm_win, 0, cur_x, &args);
  }
}

impl meter::Meter for ProcessMeter {
  fn render(&mut self) {
    let win = self.win;
    let subwins = &self.subwins;
    werase(win);

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
    let cmdline = &proc.cmdline;
    self.render_comm(cmdline);

    wrefresh(win);
  }

  fn init_meter(
    parent: WINDOW,
    _wm: &mut window::WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    let (win, subwins) = create_meter_win(parent, width, y, x);
    ProcessMeter {
      height,
      width,
      win,
      subwins,
      process: None,
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, _y: i32, _x: i32) {
    self.width = width;
    self.height = height;

    // resize entire window
    wresize(self.win, height, width);
    // resize sub windows
    let comm_win = self.subwins.comm_win;
    let new_width = self.width - (PID_WIDTH + 1 + CPU_WIDTH + 1);
    wresize(comm_win, 1, new_width);

    wrefresh(comm_win);
    wrefresh(self.win);
  }
}

// create header windows inside `parent`.
pub fn create_header_win(parent: WINDOW, width: i32, _y: i32, _x: i32) -> SubWins {
  // create sub windows
  let mut cur_x = 0;
  let pid_win = derwin(parent, 1, PID_WIDTH, 0, cur_x);
  cur_x += PID_WIDTH + 1;
  let cpu_win = derwin(parent, 1, CPU_WIDTH, 0, cur_x);
  cur_x += CPU_WIDTH + 1;
  let comm_win = derwin(parent, 1, width - cur_x, 0, cur_x);
  wattron(comm_win, COLOR_PAIR(color::cpair::DEFAULT));
  bkgd(' ' as chtype | COLOR_PAIR(color::cpair::DEFAULT) as chtype);

  wrefresh(parent);
  SubWins {
    cpu_win,
    pid_win,
    comm_win,
  }
}

impl SubWins {
  pub fn resize(&mut self, width: i32) {
    let comm_win = self.comm_win;
    let new_width = width - (PID_WIDTH + 1 + CPU_WIDTH + 1);
    wresize(comm_win, 1, new_width);
    self.refresh();
  }

  pub fn refresh(&mut self) {
    wrefresh(self.pid_win);
    wrefresh(self.cpu_win);
    wrefresh(self.comm_win);
  }
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

// XXX too dirty
pub fn _init_meter(parent: WINDOW, width: i32, y: i32, x: i32) -> ProcessMeter {
  let (win, subwins) = create_meter_win(parent, width, y, x);

  ProcessMeter {
    height: 1,
    width: width,
    win,
    subwins,
    process: None,
  }
}
