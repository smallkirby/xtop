use crate::render::window;
use crate::resource::cpu;
use ncurses::*;

use super::window::WinManager;

static BRACKET_START_X: i32 = 3;
static BRACKET_START_Y: i32 = 0;
static HEIGHT: i32 = 1;

#[derive(Debug)]
pub struct CPUMeter {
  cpu: cpu::CPU,
  height: i32,
  width: i32,
  win: WINDOW,
}

impl CPUMeter {}

pub fn winsize_require(wm: &WinManager, cpus: &Vec<cpu::CPU>) -> (i32, i32) {
  let width = wm.screen_width;
  let height = if cpus.len() % 2 == 0 {
    cpus.len() / 2
  } else {
    cpus.len() / 2 + 1
  } as i32;

  (width, height)
}

pub fn init_meters(wm: &window::WinManager, cpus: &Vec<cpu::CPU>) -> Vec<CPUMeter> {
  let mut meters = vec![];
  let num_cpu = cpus.len();
  let width = wm.screen_width / 2;
  let height = HEIGHT;
  for i in 0..num_cpu {
    let (y, x) = pos_win_start(&cpus[i], width);
    let win = create_meter_win(wm.cpumeter_win.unwrap(), height, width, y, x);
    let meter = CPUMeter {
      cpu: cpus[i],
      height,
      width,
      win,
    };
    meters.push(meter);

    waddstr(win, &format!("cpu{:>2} ", cpus[i].id));
    wrefresh(win);
  }

  meters
}

fn create_meter_win(parent: WINDOW, height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = subwin(parent, height, width, y, x);
  wrefresh(win);
  win
}

// returns the position where this CPUMeter's window starts, inside parent window.
fn pos_win_start(cpu: &cpu::CPU, width: i32) -> (i32, i32) {
  let id = cpu.id;
  let x = if id % 2 == 0 { 0 } else { width };
  let y = id as i32 / 2;

  (y, x)
}
