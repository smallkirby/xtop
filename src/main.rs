use xtop::render::window;

fn main() {
  let mut wm = window::WinManager::new();

  wm.init_cpu_meters();
  wm.qloop();
}
