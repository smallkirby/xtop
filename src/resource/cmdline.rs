/*****

/proc/<pid>/stat/cmdline related funcs.

*******/

use crate::resource::process;
use std::fs;

pub fn read_cmd_files(proc: &mut process::Process, dname: &str) -> Result<(), ()> {
  let cmdline = match fs::read_to_string(&format!("{}/cmdline", dname)) {
    Ok(_cmdline) => _cmdline,
    Err(_) => return Err(()),
  };

  if cmdline.is_empty() {
    if proc.state != process::ProcState::Zombie {
      proc.is_kernel_thread = true;
    }
    proc.cmdline = String::from("[kthread]");
    return Ok(());
  }
  proc.cmdline = cmdline.replace("\x00", " ");

  proc.comm = match fs::read_to_string(&format!("{}/comm", dname)) {
    Ok(_comm) => _comm,
    Err(_) => "".to_string(),
  };

  // XXX should consider deleted exe
  proc.exe = match fs::read_link(&format!("{}/exe", dname)) {
    Ok(_link) => _link.to_str().unwrap().to_string(),
    Err(_) => "".to_string(),
  };

  Ok(())
}
