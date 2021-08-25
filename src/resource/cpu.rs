use std::fs;

pub enum CPUFREQ {
  Valid(u64),
  Absent,
  Offline,
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
}
