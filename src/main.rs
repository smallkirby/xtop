use xtop::render::window;
use xtop::resource::cpu;

fn main() {
  let mut wm = window::WinManager::new();
  let cpus = cpu::init_cpus();
  wm.init_cpu_meters(&cpus);

  wm.qloop();
}
