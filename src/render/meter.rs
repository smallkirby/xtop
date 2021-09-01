use super::window::WinManager;

pub trait Meter {
  // render updated values and refresh.
  fn render(&mut self);

  // init a meter and returns created meter.
  fn init_meter(wm: &mut WinManager, height: i32, width: i32, y: i32, x: i32) -> Self;
}
