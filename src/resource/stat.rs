/*****

/proc/stat/ related funcs.

*******/

use super::cpu::*;
use std::fs;

#[derive(Debug)]
pub struct StatCpuTime {
  pub id: CPUID,
  pub usertime: u64,
  pub nicetime: u64,
  pub systemtime: u64,
  pub idletime: u64,
  pub iowait: u64,
  pub irq: u64,
  pub softirq: u64,
  pub steal: u64,
  pub guest: u64,
  pub guestnice: u64,
}

#[derive(Debug)]
pub enum CPUID {
  Id(u32),
  Average,
}

pub fn get_btime() -> i64 {
  let stat = fs::read_to_string("/proc/stat").unwrap();
  for s in stat.split("\n").into_iter() {
    if !s.starts_with("btime") {
      continue;
    }
    let ss: Vec<&str> = s.split(" ").collect();
    return ss[1].parse().unwrap();
  }
  panic!("failed to fetch btime.");
}

// scan one cpu and update both its time and period.
fn _scan_cpu_time(cpu: &mut CPU) {
  let cpu_times = get_cpu_time();

  for i in 0..cpu_times.len() {
    let info = &cpu_times[i];
    let id = match info.id {
      CPUID::Average => continue,
      CPUID::Id(_id) => _id,
    };
    if id != cpu.id {
      continue;
    }

    let saturate_diff = |a, b| if a > b { a - b } else { 0 };

    // guest is included in usertime/nicetime
    let usertime = info.usertime - info.guest;
    let nicetime = info.nicetime - info.guestnice;

    // classify them
    let idle_alltime = info.idletime + info.iowait;
    let system_alltime = info.systemtime + info.irq + info.softirq;
    let virt_alltime = info.guest + info.guestnice;
    let totaltime = usertime + nicetime + system_alltime + idle_alltime + info.steal + virt_alltime;

    // update period
    cpu.usertime_period = saturate_diff(usertime, cpu.usertime);
    cpu.nicetime_period = saturate_diff(nicetime, cpu.nicetime);
    cpu.systemtime_period = saturate_diff(info.systemtime, cpu.systemtime);
    cpu.system_allperiod = saturate_diff(system_alltime, cpu.system_alltime);
    cpu.idletime_period = saturate_diff(info.idletime, cpu.idletime);
    cpu.idle_allperiod = saturate_diff(idle_alltime, cpu.idle_alltime);
    cpu.iowait_period = saturate_diff(info.iowait, cpu.iowait);
    cpu.irq_period = saturate_diff(info.irq, cpu.irq);
    cpu.softirq_period = saturate_diff(info.softirq, cpu.softirq);
    cpu.steal_period = saturate_diff(info.steal, cpu.steal);
    cpu.guest_period = saturate_diff(virt_alltime, cpu.guest_period);
    cpu.totaltime_period = saturate_diff(totaltime, cpu.totaltime);

    // update absolute times
    cpu.usertime = usertime;
    cpu.nicetime = nicetime;
    cpu.systemtime = info.systemtime;
    cpu.system_alltime = system_alltime;
    cpu.idletime = info.idletime;
    cpu.idle_alltime = idle_alltime;
    cpu.iowait = info.iowait;
    cpu.irq = info.irq;
    cpu.softirq = info.softirq;
    cpu.steal = info.steal;
    cpu.guesttime = virt_alltime;
    cpu.totaltime = totaltime;
  }
}

// index 0 is always aggregated usage of CPUs.
pub fn get_cpu_time() -> Vec<StatCpuTime> {
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
    let guest = popi(&mut times);
    let guestnice = popi(&mut times);

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
      guest,
      guestnice,
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
