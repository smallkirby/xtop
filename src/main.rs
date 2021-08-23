use xtop::resource;

fn main() {
  let pids = resource::process::list_all_pids();
  println!("# of process: {}", pids.len());
}
