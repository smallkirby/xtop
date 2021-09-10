/*****

Implementation of InputMeter.
InputMeter shows the list of X input devices and their hierachy.

*******/

use crate::render::{color::*, meter::*};
use crate::resource::input::{self, InputDevice};
use ncurses::*;

#[derive(Debug)]
struct InputHierachy {
  master: input::InputDevice,
  slaves: Vec<input::InputDevice>,
}

#[derive(Debug)]
enum InputBlock {
  Floating(Vec<InputDevice>),
  Keyboard(InputHierachy),
  Pointer(InputHierachy),
}

fn get_input_blocks(devices: &[InputDevice]) -> Vec<InputBlock> {
  let mut blocks = vec![];
  let masters = devices.iter().filter(|&d| d.state == input::State::Master);

  // search for Keyboard/Pointer
  for master in masters {
    let mut slaves = vec![];

    for cand in devices {
      if cand.id == master.id || cand.typ != master.typ {
        continue;
      }
      slaves.push(cand.clone());
    }

    let hierachy = InputHierachy {
      master: master.clone(),
      slaves,
    };
    let block = match master.typ {
      input::InputType::Keyboard => InputBlock::Keyboard(hierachy),
      input::InputType::Pointer => InputBlock::Pointer(hierachy),
    };
    blocks.push(block);
  }

  // Search for Floatings
  let floating_devices = devices
    .iter()
    .filter(|&d| d.state == input::State::Floating);
  blocks.push(InputBlock::Floating(floating_devices.cloned().collect()));

  blocks
}

pub struct InputMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  devices: Vec<InputBlock>,
}

impl InputMeter {
  pub fn update_inputs(&mut self) {
    let devices = input::get_devices();
    self.devices = get_input_blocks(&devices);
  }
}

impl Meter for InputMeter {
  fn render(&mut self) {
    use InputBlock::*;

    let win = self.win;
    // erase and draw box
    werase(win);

    // draw picture
    let mut cy = 1;
    for block in &self.devices {
      let mut cx = 1;
      match block {
        Floating(slaves) => {
          mvwaddstr_color(self.win, cy, cx, "Floating", cpair::PAIR_COMM);
          cy += 1;
          cx = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cy, cx, slave_str);
            cx += slave_str.len() as i32;
            if cx > (self.width * 2 / 3) {
              cx = 2;
              cy += 1;
            }
          }
        }
        Keyboard(hierachy) => {
          let master = &hierachy.master;
          let slaves = &hierachy.slaves;
          let s = "Keyboards";
          mvwaddstr_color(self.win, cy, cx, s, cpair::PAIR_COMM);
          cx += s.len() as i32;
          let s = &format!(": {}({})", master.name, master.id);
          mvwaddstr(self.win, cy, cx, s);
          cy += 1;
          cx = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cy, cx, slave_str);
            cx += slave_str.len() as i32;
            if cx > (self.width * 2 / 3) {
              cx = 2;
              cy += 1;
            }
          }
        }
        Pointer(hierachy) => {
          let master = &hierachy.master;
          let slaves = &hierachy.slaves;
          let s = "Pointers";
          mvwaddstr_color(self.win, cy, cx, s, cpair::PAIR_COMM);
          cx += s.len() as i32;
          let s = &format!(": {}({})", master.name, master.id);
          mvwaddstr(self.win, cy, cx, s);
          cy += 1;
          cx = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cy, cx, slave_str);
            cx += slave_str.len() as i32;
            if cx > (self.width * 2 / 3) {
              cx = 2;
              cy += 1;
            }
          }
        }
      }
    }

    // draw header
    box_(win, 0, 0);
    mvwaddstr_color(win, 0, 1, " X Inputs ", cpair::PAIR_HEAD);

    wrefresh(win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut super::window::WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self {
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    wrefresh(win);

    InputMeter {
      width,
      height,
      win,
      devices: vec![],
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.height = height;
    self.width = width;
    wresize(self.win, height, width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
