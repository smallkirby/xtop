use xtop::render::window;

#[cfg(all(target_os = "linux"))]
fn main() {
  let mut wm = window::WinManager::new();

  wm.init_meters();
  wm.qloop();
}

#[cfg(not(all(target_os = "linux")))]
fn main() {
  println!("xtop needs below:");
  println!(" - Linux environment (intended on Ubuntu only)");
  println!(" - ncursesw is installed and wide feature is enabled.");
  println!("Some conditions are not fulfilled in your environment.");
}
