use xtop::render::window;

fn main() {
  let mut wm = window::WinManager::new();

  wm.init_cpumanager();
  wm.init_taskmeter();
  wm.init_process_meters();
  wm.qloop();
}
