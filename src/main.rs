use xtop::render::window;

fn main() {
  let mut wm = window::WinManager::new();

  // this should be called inside window module, cuz the order is important.
  wm.init_meters();
  wm.qloop();
}
