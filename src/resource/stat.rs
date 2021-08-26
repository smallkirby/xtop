/*****

/proc/stat/ related funcs.

*******/

use super::cpu::*;
use std::fs;

#[derive(Debug)]
pub struct StatCpuTime {
  id: CPUID,
  usertime: u64,
  nicetime: u64,
  systemtime: u64,
  idletime: u64,
  iowait: u64,
  irq: u64,
  softirq: u64,
  steal: u64,
}

#[derive(Debug)]
enum CPUID {
  Id(u32),
  Average,
}

pub fn scan_cpu_time(cpus: &mut Vec<CPU>) {
  let cpu_times = get_cpu_time();

  for i in 0..cpu_times.len() {
    let info = &cpu_times[i];
    let id = match info.id {
      CPUID::Average => continue,
      CPUID::Id(_id) => _id as usize,
    };
    cpus[id].usertime = info.usertime;
    cpus[id].nicetime = info.nicetime;
    cpus[id].systemtime = info.systemtime;
    cpus[id].idletime = info.idletime;
    cpus[id].iowait = info.iowait;
    cpus[id].irq = info.irq;
    cpus[id].softirq = info.softirq;
    cpus[id].steal = info.steal;
  }
}

// index 0 is always average usage of CPUs.
fn get_cpu_time() -> Vec<StatCpuTime> {
  use util::*;

  let mut stat_s = read_stat_string();
  let mut result = vec![];

  loop {
    let cpu_line = if stat_s[0].starts_with("cpu") {
      let _s = stat_s[0].to_owned();
      stat_s.remove(0);
      _s
    } else {
      break;
    };
    let mut times = cpu_line.split_whitespace().collect::<Vec<&str>>();

    let id = {
      let _id = times[0];
      times.remove(0);
      match _id {
        "cpu" => CPUID::Average,
        _ => CPUID::Id(_id[3..].parse().unwrap()),
      }
    };
    let usertime = popi(&mut times);
    let nicetime = popi(&mut times);
    let systemtime = popi(&mut times);
    let idletime = popi(&mut times);
    let iowait = popi(&mut times);
    let irq = popi(&mut times);
    let softirq = popi(&mut times);
    let steal = popi(&mut times);

    result.push(StatCpuTime {
      id,
      usertime,
      nicetime,
      systemtime,
      idletime,
      iowait,
      irq,
      softirq,
      steal,
    });
  }

  result
}

fn read_stat_string() -> Vec<String> {
  fs::read_to_string("/proc/stat")
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    .collect()
}

mod util {
  // pop a first element and parse into u64
  pub fn popi(s: &mut Vec<&str>) -> u64 {
    let n = s[0].parse().unwrap();
    s.remove(0);
    n
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_cpu_time() {
    let cpu_times = get_cpu_time();
    assert_eq!(cpu_times.len() >= 1, true);
    println!("cpu times: {:?}", cpu_times);
  }
}
