use xtop::proclist::list;
use xtop::render::window;
use xtop::resource::cpu;

fn main() {
  //let mut wm = window::WinManager::new();
  //let cpus = cpu::init_cpus();
  let mut plist = list::ProcList::new();

  plist.recurse_proc_tree(None, "/proc"); // XXX
  for (k, v) in plist.plist {
    // XXX
    println!("{}", v.tty_name);
  }

  //wm.init_cpu_meters(&cpus);

  //wm.qloop();
}
