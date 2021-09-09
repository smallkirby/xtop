/*****

Implementation of CPUMeter.
CPUMeter shows information about each CPUs.

*******/

use crate::render::{color, meter};
use crate::resource::cpu;
use ncurses::*;

use super::window::WinManager;

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
    werase(win);

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

  fn init_meter(
    parent: WINDOW,
    _wm: &mut WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    let win = create_meter_win(parent, height, width, y, x);
    CPUMeter {
      height,
      width,
      win,
      cpu: None,
    }
  }

  fn resize(&mut self, parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.height = height;
    self.width = width;
    wresize(self.win, height, width);
    werase(self.win);
    mvwin(self.win, y, x);
  }
}

impl CPUMeter {
  pub fn set_cpu(&mut self, cpu: cpu::CPU) {
    self.cpu = Some(cpu);
  }
}

fn create_meter_win(parent: WINDOW, height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = derwin(parent, height, width, y, x);
  wattron(win, COLOR_PAIR(color::cpair::DEFAULT));
  wrefresh(win);
  win
}
