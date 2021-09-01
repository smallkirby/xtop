use xtop::render::window;

fn main() {
  let mut wm = window::WinManager::new();

  // this should be called inside window module, cuz the order is important.
  wm.init_cpumanager();
  wm.init_taskmeter();
  wm.init_cpugraph();
  wm.init_process_meters();
  wm.qloop();
}
