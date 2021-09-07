/*****

Implementation of InputMeter.
InputMeter shows the list of X input devices and their hierachy.

*******/

use crate::render::{color, meter::*};
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

fn get_input_blocks(devices: &Vec<InputDevice>) -> Vec<InputBlock> {
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
  blocks.push(InputBlock::Floating(
    floating_devices.map(|d| d.clone()).collect(),
  ));

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
    let mut cursor_y = 1;
    for block in &self.devices {
      let mut cursor_x = 1;
      match block {
        Floating(slaves) => {
          mvwaddstr(self.win, cursor_y, cursor_x, "Floating");
          cursor_y += 1;
          cursor_x = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cursor_y, cursor_x, slave_str);
            cursor_x += slave_str.len() as i32;
            if cursor_x > (self.width * 2 / 3) {
              cursor_x = 2;
              cursor_y += 1;
            }
          }
        }
        Keyboard(hierachy) => {
          let master = &hierachy.master;
          let slaves = &hierachy.slaves;
          mvwaddstr(
            self.win,
            cursor_y,
            cursor_x,
            &format!("Keyboards: {}({})", master.name, master.id),
          );
          cursor_y += 1;
          cursor_x = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cursor_y, cursor_x, slave_str);
            cursor_x += slave_str.len() as i32;
            if cursor_x > (self.width * 2 / 3) {
              cursor_x = 2;
              cursor_y += 1;
            }
          }
        }
        Pointer(hierachy) => {
          let master = &hierachy.master;
          let slaves = &hierachy.slaves;
          mvwaddstr(
            self.win,
            cursor_y,
            cursor_x,
            &format!("Pointers: {}({})", master.name, master.id),
          );
          cursor_y += 1;
          cursor_x = 2;
          for slave in slaves {
            let slave_str = &format!("{}({}), ", slave.name, slave.id);
            mvwaddstr(self.win, cursor_y, cursor_x, slave_str);
            cursor_x += slave_str.len() as i32;
            if cursor_x > (self.width * 2 / 3) {
              cursor_x = 2;
              cursor_y += 1;
            }
          }
        }
      }
    }

    // draw header
    box_(win, 0, 0);
    mvwaddstr(win, 0, 1, " X Inputs ");

    wrefresh(win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut super::window::WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    if height.is_none() || width.is_none() {
      panic!("height and width must be specified for InputMeter::init_meter().");
    }
    let height = height.unwrap();
    let width = width.unwrap();
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(color::cpair::DEFAULT));
    wbkgd(
      win,
      ' ' as chtype | COLOR_PAIR(color::cpair::DEFAULT) as chtype,
    );
    box_(win, 0, 0);
    wrefresh(win);

    InputMeter {
      width,
      height,
      win,
      devices: vec![],
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: Option<i32>, width: Option<i32>, y: i32, x: i32) {
    self.width = match width {
      Some(w) => w,
      None => self.width,
    };
    self.height = match height {
      Some(h) => h,
      None => self.height,
    };

    wresize(self.win, self.height, self.width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
