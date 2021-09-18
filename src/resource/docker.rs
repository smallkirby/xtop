/*****

Docker related funcs.

xtop assumes only cgroups v1. (defualt of Ubuntu)
This file is partialy just a wrapper of docker command,
but also search sys filesystem directly.

cf: https://docs.docker.com/config/containers/runmetrics/

*******/

use crate::util::popfirst;

use std::fs;
use std::process::Command;

#[derive(Debug)]
enum DockerUptimeUnit {
  Second,
  Minute,
  Hour,
  Day,
  Month,
  Invalid,
}

impl DockerUptimeUnit {
  pub fn from(s: &str) -> Self {
    if s.starts_with("second") {
      Self::Second
    } else if s.starts_with("minute") {
      Self::Minute
    } else if s.starts_with("hour") {
      Self::Hour
    } else if s.starts_with("day") {
      Self::Day
    } else if s.starts_with("month") {
      Self::Month
    } else {
      Self::Invalid
    }
  }
}

impl std::fmt::Display for DockerUptimeUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use DockerUptimeUnit::*;
    let s = match self {
      Second => "Sec",
      Minute => "Min",
      Hour => "Hour",
      Day => "Day",
      Month => "Mon",
      Invalid => "?",
    };
    write!(f, "{}", s)
  }
}

#[derive(Debug)]
pub struct DockerUptime {
  val: u32,
  unit: DockerUptimeUnit,
}

impl std::fmt::Display for DockerUptime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:>3} {}", self.val, self.unit)
  }
}

impl DockerUptime {
  pub fn from(s: &str) -> Self {
    let mut tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens[0].contains("About") {
      popfirst(&mut tokens);
    };
    let val = match tokens[0] {
      "a" | "an" => 1,
      _ => tokens[0].parse().unwrap(),
    };
    let unit = DockerUptimeUnit::from(tokens[1]);

    Self { val, unit }
  }
}

#[derive(Debug)]
pub struct DockerExtInfo {
  pub psinfo: DockerPsInfo,
  cpuuser: u64,   // total jiffies spent on user procs in container
  cpusystem: u64, // total jiffies spent on kernel
  uptime: f64,
}

impl DockerExtInfo {
  pub fn from(psinfo: DockerPsInfo) -> Self {
    Self {
      psinfo,
      cpuuser: 0,
      cpusystem: 0,
      uptime: 0.0,
    }
  }

  pub fn update(&mut self) {
    let uptime: f64 = match fs::read_to_string("/proc/uptime") {
      Ok(o) => {
        let tokens: Vec<&str> = o.split_whitespace().collect();
        tokens[0].parse().unwrap()
      }
      Err(_) => return,
    };
    let prev_uptime = self.uptime;
    self.uptime = uptime;

    self.update_cpu(prev_uptime);
  }

  fn update_cpu(&mut self, prev_uptime: f64) {
    if let Some((u, s)) = read_cpustat(&self.psinfo.full_id) {
      self.cpuuser = u;
      self.cpusystem = s;
    }
  }
}

#[derive(Debug)]
pub struct DockerPsInfo {
  pub full_id: String,
  pub short_id: String,
  pub image: String,
  pub command: String,
  pub created: DockerUptime,
  pub uptime: DockerUptime,
  pub ports: Vec<String>,
  pub name: String,
}

impl DockerPsInfo {
  pub fn try_from(line: &str) -> Option<Self> {
    let mut tokens: Vec<&str> = line.split_whitespace().collect();

    let short_id = popfirst(&mut tokens)?.into();
    let image = popfirst(&mut tokens)?.into();

    let mut command = String::new();
    loop {
      let t = popfirst(&mut tokens)?;
      if t.starts_with('"') && t.ends_with('"') {
        command.push_str(&t[1..(t.len() - 1)]);
        break;
      } else if t.ends_with('"') {
        command.push_str(&t[..(t.len() - 1)]);
        break;
      } else if t.starts_with('"') {
        command.push_str(&t[1..]);
      } else {
        command.push_str(&t);
      }
    }

    let mut created_str = String::new();
    if tokens[0] == "About" {
      popfirst(&mut tokens)?;
    }
    for _ in 0..3 {
      created_str.push_str(popfirst(&mut tokens)?);
      created_str.push_str(" ");
    }
    let created = DockerUptime::from(&created_str);

    if popfirst(&mut tokens)? != "Up" {
      return None;
    }

    if tokens[0] == "About" {
      popfirst(&mut tokens)?;
    }
    let mut status_str = String::new();
    for _ in 0..2 {
      status_str.push_str(popfirst(&mut tokens)?);
      status_str.push_str(" ");
    }
    let uptime = DockerUptime::from(&status_str);

    let mut ports = vec![];
    if tokens.len() != 1 {
      loop {
        let t = popfirst(&mut tokens)?;
        if t.ends_with(",") {
          ports.push(t.replace(",", ""));
        } else {
          ports.push(t.into());
          break;
        }
      }
    }

    let name: String = popfirst(&mut tokens)?.into();

    let full_id = get_full_id(&name);

    Some(Self {
      full_id,
      short_id,
      image,
      command,
      created,
      uptime,
      ports,
      name,
    })
  }
}

// get only Up containers
pub fn get_docker_ps_up_ext() -> Vec<DockerExtInfo> {
  let containers = get_docker_ps_up();
  let mut result = vec![];
  for container in containers {
    result.push(DockerExtInfo::from(container));
  }

  result
}

fn get_docker_ps_up() -> Vec<DockerPsInfo> {
  let mut result = vec![];
  let ps_result = match Command::new("docker").arg("ps").output() {
    Ok(output) => String::from_utf8(output.stdout).unwrap(),
    Err(_) => return result,
  };

  let ps_result_lines: Vec<&str> = ps_result.split("\n").collect();
  if ps_result_lines.len() == 1 {
    return result;
  }
  for line in ps_result_lines[1..].into_iter() {
    match DockerPsInfo::try_from(line) {
      Some(container) => result.push(container),
      _ => {}
    }
  }

  result
}

fn get_full_id(name: &str) -> String {
  match Command::new("docker")
    .arg("inspect")
    .arg("-f")
    .arg("'{{ .Id }}'")
    .arg(name)
    .output()
  {
    Ok(output) => {
      let id_with_quote = String::from_utf8(output.stdout).unwrap();
      id_with_quote.trim()[1..(id_with_quote.len() - 2)].into()
    }
    Err(_) => "".into(),
  }
}

fn read_cpustat(cgroup_id: &str) -> Option<(u64, u64)> {
  let stat_str = match fs::read_to_string(format!(
    "/sys/fs/cgroup/cpuacct/docker/{}/cpuacct.stat",
    cgroup_id
  )) {
    Ok(o) => o,
    Err(_) => return None,
  };

  let lines: Vec<&str> = stat_str.split("\n").collect();
  let user = lines[0].split_whitespace().collect::<Vec<&str>>()[1];
  let system = lines[1].split_whitespace().collect::<Vec<&str>>()[1];

  Some((user.parse().unwrap(), system.parse().unwrap()))
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn test_get_docker_ps_up() {
    let containers = get_docker_ps_up();
    for container in containers {
      let mut ext = DockerExtInfo::from(container);
      ext.update();
      println!("{:?}", ext);
    }
  }
}
