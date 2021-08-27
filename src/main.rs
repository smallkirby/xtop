use xtop::proclist::list;
use xtop::render::window;
use xtop::resource::cpu;

fn main() {
  //let mut wm = window::WinManager::new();
  //let cpus = cpu::init_cpus();
  let mut plist = list::ProcList::new();

  plist.recurse_proc_tree(None, "/proc"); // XXX

  //wm.init_cpu_meters(&cpus);

  //wm.qloop();
}
