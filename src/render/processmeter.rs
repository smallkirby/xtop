use crate::render::window;
use crate::resource::process;
use ncurses::*;

#[derive(Debug)]
pub struct ProcessMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
}

impl ProcessMeter {
  pub fn render(&mut self, proc: &process::Process) {
    let win = self.win;
    wclear(win);
    mvwprintw(
      win,
      0,
      0,
      &format!("{} {:>3.2} {} ", proc.pid, proc.percent_cpu, proc.cmdline),
    );
    wrefresh(win);
  }
}

pub fn init_meters(wm: &mut window::WinManager, height: i32) -> Vec<ProcessMeter> {
  let mut meters = vec![];
  let width = wm.screen_width;
  for i in 0..height {
    let win = create_meter_win(wm.processmeter_win.unwrap(), 1, width, i, 0);
    let meter = ProcessMeter {
      height: 1,
      width,
      win,
    };
    meters.push(meter);
  }

  meters
}

fn create_meter_win(parent: WINDOW, height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = derwin(parent, height, width, y, x);
  wrefresh(win);
  win
}
