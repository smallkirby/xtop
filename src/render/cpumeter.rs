use crate::render::window;
use crate::resource::cpu;
use ncurses::*;

use super::window::WinManager;

static HEIGHT: i32 = 1;

#[derive(Debug)]
pub struct CPUMeter {
  cpu: cpu::CPU,
  height: i32,
  width: i32,
  win: WINDOW,
}

impl CPUMeter {
  pub fn render(&mut self) {
    self.cpu.update_time_and_period(); // XXX actually, update of these values should be at once.

    let win = self.win;
    let cpu = self.cpu;
    let max_width = self.width - "cpuxx []".len() as i32 - 1;
    let percent = cpu.percent() * 0.01;
    let divs = (0..((max_width as f64 * percent) as u32))
      .map(|_| "|")
      .collect::<String>();
    let spaces = (0..(max_width - divs.len() as i32))
      .map(|_| " ")
      .collect::<String>();
    mvwprintw(win, 0, 0, &format!("cpu{:>2} [{}{}]", cpu.id, divs, spaces));
    wrefresh(win);
  }
}

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
    let mut meter = CPUMeter {
      cpu: cpus[i],
      height,
      width,
      win,
    };
    meter.render();
    meters.push(meter);
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
