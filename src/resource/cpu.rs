/*****

CPU related structures and funcs.

*******/

use crate::resource::stat;

use std::{
  fmt::{self},
  fs,
};

#[derive(Copy, Clone)]
pub enum CPUFREQ {
  Valid(u64), // kHz
  Absent,
  Offline,
}

impl fmt::Display for CPUFREQ {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Self::Valid(freq) => write!(f, "{:>4} MHz", freq / 1000),
      Self::Absent => write!(f, "absent"),
      Self::Offline => write!(f, "offline"),
    }
  }
}

impl fmt::Debug for CPUFREQ {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Debug, Copy, Clone)]
pub struct CPU {
  pub freq: CPUFREQ,
  pub id: u32,

  // times
  pub usertime: u64,
  pub nicetime: u64,
  pub systemtime: u64,
  pub idletime: u64,
  pub iowait: u64,
  pub irq: u64,
  pub softirq: u64,
  pub steal: u64,
  pub guesttime: u64,

  // periods
  pub usertime_period: u64,
  pub nicetime_period: u64,
  pub systemtime_period: u64,
  pub idletime_period: u64,
  pub iowait_period: u64,
  pub irq_period: u64,
  pub softirq_period: u64,
  pub steal_period: u64,
  pub guest_period: u64,

  // allperiods
  pub system_allperiod: u64,
  pub idle_allperiod: u64,
  pub totaltime_period: u64,

  // alltimes
  pub system_alltime: u64,
  pub idle_alltime: u64,
  pub virt_alltime: u64,
  pub totaltime: u64,
}

impl Default for CPU {
  fn default() -> Self {
    Self {
      freq: CPUFREQ::Absent,
      id: 0,
      usertime: 0,
      nicetime: 0,
      systemtime: 0,
      idletime: 0,
      iowait: 0,
      irq: 0,
      softirq: 0,
      steal: 0,
      guesttime: 0,
      usertime_period: 0,
      nicetime_period: 0,
      systemtime_period: 0,
      idletime_period: 0,
      iowait_period: 0,
      irq_period: 0,
      softirq_period: 0,
      steal_period: 0,
      guest_period: 0,
      system_allperiod: 0,
      idle_allperiod: 0,
      totaltime_period: 0,
      system_alltime: 0,
      idle_alltime: 0,
      virt_alltime: 0,
      totaltime: 0,
    }
  }
}

impl CPU {
  pub fn new(id: u32) -> Self {
    let freq = get_cpu_freq(id);

    Self {
      freq: CPUFREQ::Valid(freq),
      id,
      ..Default::default()
    }
  }

  pub fn freq_update(&mut self) {
    let freq = get_cpu_freq(self.id);
    self.freq = CPUFREQ::Valid(freq);
  }

  pub fn clear_state(&mut self) {
    self.freq = CPUFREQ::Offline;
  }

  pub fn percent(&self) -> f64 {
    let total = match self.totaltime_period {
      0 => 1,
      _ => self.totaltime_period,
    };
    let calc = |n| n as f64 / total as f64 * 100.0;

    let nice = calc(self.nicetime_period);
    let normal = calc(self.usertime_period);
    let kernel = calc(self.systemtime_period);
    let irq = calc(self.irq_period);
    let softirq = calc(self.softirq_period);
    let steal = calc(self.steal_period);
    let guest = calc(self.guest_period);
    let iowait = calc(self.iowait_period);

    crate::util::clamp(
      nice + normal + kernel + irq + softirq + steal + guest + iowait,
      0.0,
      100.0,
    )
  }
}

pub fn init_cpus() -> Vec<CPU> {
  let mut cpus = vec![];
  let avail_cpus = num_available_cpus();
  for i in 0..avail_cpus {
    cpus.push(CPU::new(i));
  }

  cpus
}

pub fn num_available_cpus() -> u32 {
  let mut total = 0;
  let cpudir = fs::read_dir("/sys/devices/system/cpu/").unwrap();
  for file in cpudir {
    let fname = file.unwrap().file_name().to_string_lossy().to_string();
    if fname.starts_with("cpu") && fname[3..].parse::<u32>().is_ok() {
      total += 1;
    }
  }
  total
}

pub fn get_cpus_freq(avail_cpus: u32) -> Vec<u64> {
  let mut result = vec![];

  for i in 0..avail_cpus {
    let freq = get_cpu_freq(i);
    result.push(freq);
  }

  result
}

pub fn is_scaling_cur_freq_supported() -> bool {
  std::path::Path::new("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq").exists()
}

// get cpu freqency in kHz
fn get_cpu_freq(cpu: u32) -> u64 {
  if is_scaling_cur_freq_supported() {
    _get_cpu_freq(cpu)
  } else {
    _get_cpu_freq_fallback(cpu)
  }
}

fn _get_cpu_freq(cpu: u32) -> u64 {
  let scaling_freq = fs::read_to_string(format!(
    "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
    cpu
  ))
  .unwrap();
  scaling_freq.trim().parse().unwrap()
}

fn _get_cpu_freq_fallback(cpu: u32) -> u64 {
  let mut is_target = false;
  let _cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
  let cpuinfo: Vec<&str> = _cpuinfo.split("\n").collect();

  for l in cpuinfo {
    let params: Vec<&str> = l.split(":").map(|p| p.trim()).collect();
    if params.len() != 2 {
      continue;
    }
    if params[0] != "processor" && params[1].parse() == Ok(cpu) {
      is_target = true;
    }
    if is_target && params[0] == "cpu MHz" {
      let freq: f64 = params[1].parse().unwrap();
      return (freq * 1000.0) as u64;
    }
  }

  0
}

pub fn check_cpus_online(avail_cpus: u32) -> bool {
  let (avail_start, avail_end) = online_cpus();
  avail_cpus > (avail_end - avail_start + 1)
}

fn online_cpus() -> (u32, u32) {
  let online = fs::read_to_string("/sys/devices/system/cpu/online").unwrap();
  let nums: Vec<&str> = online.split('-').collect();
  (nums[0].parse().unwrap(), nums[1].parse().unwrap())
}

// update information of each cpus.
pub fn update_time_and_period(cpus: &mut Vec<CPU>, aggregated: &mut CPU) {
  use crate::resource::stat::CPUID;
  let cpu_times = stat::get_cpu_time(); // index 0 is aggregated CPU

  for i in 0..cpu_times.len() {
    let cpu = if i == 0 {
      &mut *aggregated
    } else {
      &mut cpus[i - 1]
    };
    let info = &cpu_times[i];
    match info.id {
      CPUID::Average => {
        if i != 0 {
          continue;
        }
      } // XXX should panic?
      CPUID::Id(id) => {
        if cpu.id != id {
          continue;
        }
      } // XXX should panic?
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_available_cpu_num() {
    let num = num_available_cpus();
    println!("# of cpus: {}", num);
    assert_eq!(num > 0, true);
  }

  #[test]
  fn list_cpus_freq() {
    let cpus = num_available_cpus();
    let freqs = get_cpus_freq(cpus);
    println!("freqs: {:?}", freqs);
  }

  #[test]
  fn test_init_cpus() {
    let mut cpus = init_cpus();
    println!("cpus: {:?}", &cpus);
    for i in 0..cpus.len() {
      cpus[i].freq_update();
    }
    println!("cpus: {:?}", &cpus);
  }

  //#[test]
  #[allow(dead_code)]
  fn test_update_cpu_time() {
    let mut cpus = init_cpus();
    let mut aggregated = CPU {
      ..Default::default()
    };
    println!("{:?}", (cpus[0], aggregated));
    update_time_and_period(&mut cpus, &mut aggregated);
    println!("{:?}", (cpus[0], aggregated));
    let dur = std::time::Duration::from_millis(1000);
    std::thread::sleep(dur);
    update_time_and_period(&mut cpus, &mut aggregated);
    println!("{:?}", cpus[0]);
  }

  //#[test]
  #[allow(dead_code)]
  fn test_percentage() {
    let mut cpus = init_cpus();
    let mut aggregated = CPU {
      ..Default::default()
    };
    let dur = std::time::Duration::from_millis(500);

    for _ in 0..10 {
      std::thread::sleep(dur);
      println!("cpu0: {} %", cpus[0].percent());
      update_time_and_period(&mut cpus, &mut aggregated);
    }
  }
}
