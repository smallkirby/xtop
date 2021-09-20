/******************

version of running system -related funcs.

******************/

use std::fs;

pub fn get_os_version() -> String {
  let os_s = match fs::read_to_string("/etc/os-release") {
    Ok(s) => s,
    Err(_) => return "".into(),
  };

  let os_lines: Vec<&str> = os_s.split('\n').collect();
  let mut osname: Option<String> = None;
  let mut osversion: Option<String> = None;
  for line in os_lines {
    if line.starts_with("NAME=") {
      osname = Some(line.trim()[6..(line.len() - 1)].into());
    }
    if line.starts_with("VERSION") {
      osversion = Some(line.trim()[9..(line.len() - 1)].into());
    }
    if osname.is_some() && osversion.is_some() {
      break;
    }
  }

  let osname = if let Some(_osname) = osname {
    _osname
  } else {
    "".into()
  };
  let osversion = if let Some(_osversion) = osversion {
    _osversion
  } else {
    "".into()
  };

  format!("{} {}", osname, osversion)
}

pub fn get_kernel_version() -> String {
  let version_s = match fs::read_to_string("/proc/version") {
    Ok(s) => s,
    Err(_) => return "".into(),
  };
  let tokens: Vec<&str> = version_s.split_whitespace().collect();

  if tokens.len() <= 3 {
    version_s
  } else {
    tokens[0..3].join(" ").trim().into()
  }
}
