use xtop::render::window;
use xtop::resource;

fn main() {
  let pids = resource::process::list_all_pids();
  window::test_just_window(&format!("# of procs: {}\n", pids.len()));
}
