use crate::resource::process;
use std::fs;

pub fn read_cmd_files(proc: &mut process::Process, dname: &str) {
  let cmdline = match fs::read_to_string(&format!("{}/cmdline", dname)) {
    Ok(_cmdline) => _cmdline,
    Err(_) => return,
  };

  if cmdline.len() <= 0 {
    if proc.state != process::ProcState::ZOMBIE {
      proc.is_kernel_thread = true;
    }
    proc.cmdline = String::from("[kthread]");
    return;
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
}
