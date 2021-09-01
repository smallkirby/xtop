/*****

Implementation of Meter trait.
Meter trait decides the necessary funcs for each meters.

*******/

use super::window::WinManager;
use ncurses::*;

pub trait Meter {
  // render updated values and refresh.
  fn render(&mut self);

  // init a meter and returns created meter.
  // if @height or @width is None, the meter can allocate window of any size as it wants.
  fn init_meter(
    parent: WINDOW,
    wm: &mut WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self;

  // resize the meter.
  fn resize(&mut self);
}
