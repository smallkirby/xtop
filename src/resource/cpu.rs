use super::stat::*;
use std::{
  fmt::{self},
  fs,
};

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

#[derive(Debug)]
pub struct CPU {
  pub freq: CPUFREQ,
  pub id: u32,
  pub usertime: u64,
  pub nicetime: u64,
  pub systemtime: u64,
  pub idletime: u64,
  pub iowait: u64,
  pub irq: u64,
  pub softirq: u64,
  pub steal: u64,
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
}

pub fn init_cpus() -> Vec<CPU> {
  let mut cpus = vec![];
  let avail_cpus = num_available_cpus();
  if is_scaling_cur_freq_supported() {
    for i in 0..avail_cpus {
      cpus.push(CPU::new(i));
    }
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

fn get_cpu_freq(cpu: u32) -> u64 {
  let scaling_freq = fs::read_to_string(format!(
    "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
    cpu
  ))
  .unwrap();
  scaling_freq.trim().parse().unwrap()
}

#[allow(dead_code)]
fn get_cpu_freq_fallback(cpu: u32) -> u64 {
  unimplemented!(
    "cpu{}: reading cpu info via cpuinfo is now not supported.",
    cpu
  );
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::resource::stat::*;

  #[test]
  fn check_available_cpu_num() {
    let num = num_available_cpus();
    println!("# of cpus: {}", num);
    assert_eq!(num > 0, true);
  }

  #[test]
  fn list_cpus_freq() {
    let cpus = num_available_cpus();
    if is_scaling_cur_freq_supported() {
      let freqs = get_cpus_freq(cpus);
      println!("freqs: {:?}", freqs);
    }
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

  #[test]
  fn test_update_cpu_time() {
    let mut cpus = init_cpus();
    println!("{:?}", cpus[0]);
    scan_cpu_time(&mut cpus);
    println!("{:?}", cpus[0]);
    let dur = std::time::Duration::from_millis(1000);
    std::thread::sleep(dur);
    scan_cpu_time(&mut cpus);
    println!("{:?}", cpus[0]);
  }
}
