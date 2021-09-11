/*****

/sys/class/net/<interface>/statistics related funcs.

*******/

use std::{fs, path};

#[derive(Debug)]
pub struct NetStatistics {
  pub interface: String,
  pub rx_bytes: u64,
  pub tx_bytes: u64,
}

fn list_interfaces() -> Vec<String> {
  let interfaces_dir = fs::read_dir("/sys/class/net").unwrap();
  interfaces_dir
    .map(|d| {
      d.unwrap()
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
    })
    .collect()
}

pub fn get_statistic(interface: &str) -> Option<NetStatistics> {
  let netpath = path::PathBuf::from(format!("/sys/class/net/{}/statistics", interface));
  let rx_file = format!("{}/rx_bytes", netpath.to_str().unwrap());
  let tx_file = format!("{}/tx_bytes", netpath.to_str().unwrap());

  let rx_bytes = match fs::read_to_string(rx_file) {
    Ok(s) => s.trim().parse::<u64>().unwrap(),
    Err(_) => return None,
  };
  let tx_bytes = match fs::read_to_string(tx_file) {
    Ok(s) => s.trim().parse::<u64>().unwrap(),
    Err(_) => return None,
  };

  Some(NetStatistics {
    interface: interface.into(),
    rx_bytes,
    tx_bytes,
  })
}

pub fn get_statistic_all() -> Vec<NetStatistics> {
  let mut result = vec![];
  let interfaces = list_interfaces();
  for interface in interfaces {
    match get_statistic(&interface) {
      Some(s) => result.push(s),
      None => continue,
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_all_interfaces() {
    let is = get_statistic_all();
    assert_eq!(is.is_empty(), false);
  }
}
