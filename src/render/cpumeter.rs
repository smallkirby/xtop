use crate::render::meter::Meter;
use crate::render::{meter, window};
use crate::resource::cpu;
use ncurses::*;

use super::window::WinManager;

static HEIGHT: i32 = 1;

#[derive(Debug)]
pub struct CPUMeter {
  pub height: i32,
  pub width: i32,
  win: WINDOW,
  cpu: Option<cpu::CPU>,
}

impl meter::Meter for CPUMeter {
  fn render(&mut self) {
    let win = self.win;
    wclear(win);

    let cpu = match self.cpu.as_ref() {
      Some(_cpu) => _cpu,
      None => {
        mvwprintw(win, 0, 0, &format!("[ERROR] CPU not initialized."));
        wrefresh(win);
        return;
      }
    };

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

  fn init_meter(wm: &mut WinManager, height: i32, width: i32, y: i32, x: i32) -> Self {
    let win = create_meter_win(wm.cpumeter_win.unwrap(), height, width, y, x);
    CPUMeter {
      height,
      width,
      win,
      cpu: None,
    }
  }
}

impl CPUMeter {
  pub fn set_cpu(&mut self, cpu: cpu::CPU) {
    self.cpu = Some(cpu);
  }
}

pub fn winsize_require(wm: &WinManager) -> (i32, i32) {
  let width = wm.screen_width;
  let height = if wm.plist.cpus.len() % 2 == 0 {
    wm.plist.cpus.len() / 2
  } else {
    wm.plist.cpus.len() / 2 + 1
  } as i32;

  (width, height)
}

pub fn init_meters(wm: &mut window::WinManager) -> Vec<CPUMeter> {
  let mut meters = vec![];
  let num_cpu = wm.plist.cpus.len();
  let width = wm.screen_width / 2;
  let height = HEIGHT;

  for i in 0..num_cpu {
    let (y, x) = pos_win_start(&wm.plist.cpus[i], width);
    let meter = CPUMeter::init_meter(wm, height, width, y, x);
    meters.push(meter);
  }

  meters
}

fn create_meter_win(parent: WINDOW, height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = derwin(parent, height, width, y, x);
  wrefresh(win);
  win
}

// returns the position where this CPUMeter's window starts, inside parent window.
// XXX the start position should be decided by class WindowManager
fn pos_win_start(cpu: &cpu::CPU, width: i32) -> (i32, i32) {
  let id = cpu.id;
  let x = if id % 2 == 0 { 0 } else { width };
  let y = id as i32 / 2;

  (y, x)
}
