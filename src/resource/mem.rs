/*****

/proc/meminfo related funcs.

*******/

use std::fs;

// unit is [kB] for all.
#[derive(Default, Debug, Clone)]
pub struct MemInfo {
  pub total: u64,      // total usable. physical RAM minus some reserved/ktext area.
  pub cached: u64,     // dirty page cache
  pub shared: u64,     // amount of memory consumed in tmpfs.
  pub used: u64,       // total used mem.
  pub buffers: u64,    // temporary small buffer. ??
  pub avail: u64,      // free/available memory
  pub total_swap: u64, // amount of swap.
  pub used_swap: u64,  // used about of swap. (total - (free swap + cached swap))
  pub cached_swap: u64, // cached swap
                       // `cached swap` means data in swap even after the data is written back to memory.
}

impl MemInfo {
  pub fn new() -> Self {
    let s_meminfo = match get_string_meminfo() {
      Ok(_s) => _s,
      Err(_) => {
        return Self {
          ..Default::default()
        }
      }
    };

    let lines: Vec<&str> = s_meminfo.split("\n").collect();
    let mut mem_total = 0;
    let mut mem_free = 0;
    let mut mem_available = 0;
    let mut buffers = 0;
    let mut cached = 0;
    let mut swap_cached = 0;
    let mut swap_total = 0;
    let mut swap_free = 0;
    let mut shmem = 0;
    let mut s_reclaimable = 0; // reclaimable slab

    for line in lines {
      // XXX match is faster?
      if line.starts_with("MemTotal") {
        mem_total = parse_line(line);
      } else if line.starts_with("MemFree") {
        mem_free = parse_line(line);
      } else if line.starts_with("MemAvailable") {
        mem_available = parse_line(line);
      } else if line.starts_with("Buffers") {
        buffers = parse_line(line);
      } else if line.starts_with("Cached") {
        cached = parse_line(line);
      } else if line.starts_with("SwapCached") {
        swap_cached = parse_line(line);
      } else if line.starts_with("SwapTotal") {
        swap_total = parse_line(line);
      } else if line.starts_with("SwapFree") {
        swap_free = parse_line(line);
      } else if line.starts_with("Shmem") {
        shmem = parse_line(line);
      } else if line.starts_with("SReclaimable") {
        s_reclaimable = parse_line(line);
      }
    }

    let used_diff = mem_free + cached + s_reclaimable + buffers;
    let used = if mem_total >= used_diff {
      mem_total - used_diff
    } else {
      mem_total - mem_free
    };
    let avail = if mem_available != 0 {
      std::cmp::min(mem_available, mem_total)
    } else {
      mem_free
    };

    Self {
      total: mem_total,
      cached: cached + s_reclaimable - shmem,
      shared: shmem,
      used,
      buffers,
      avail,
      total_swap: swap_total,
      used_swap: swap_total - swap_free - swap_cached,
      cached_swap: swap_cached,
    }
  }
}

fn get_string_meminfo() -> Result<String, String> {
  match fs::read_to_string("/proc/meminfo") {
    Ok(_s) => Ok(_s.trim().into()),
    Err(e) => Err(e.to_string()),
  }
}

fn parse_line(line: &str) -> u64 {
  let tokens: Vec<&str> = line.split_whitespace().collect();
  tokens[1].parse().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  //#[test]
  #[allow(dead_code)]
  fn test_new_meminfo() {
    let meminfo = MemInfo::new();
    println!("{:?}", meminfo);
  }
}
