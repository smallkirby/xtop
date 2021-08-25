use xtop::resource;
use xtop::render::window;

fn main() {
  let pids = resource::process::list_all_pids();
  window::test_just_window(&format!("# of procs: {}\n", pids.len()));
}
