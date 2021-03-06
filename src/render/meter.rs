/*****

Implementation of Meter trait.
Meter trait decides the necessary funcs for each meters.

*******/

use super::executer::manager::WinManager;
use ncurses::*;

pub trait Meter {
  // render updated values and refresh.
  fn render(&mut self);

  // init a meter and returns created meter.
  fn init_meter(
    parent: WINDOW,
    wm: &mut WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self
  where
    Self: Sized;

  // resize the meter.
  fn resize(&mut self, parent: WINDOW, height: i32, width: i32, y: i32, x: i32);

  // click handler.
  fn handle_click(&mut self, _y: i32, _x: i32) {}
}
