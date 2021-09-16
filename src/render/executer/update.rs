/**************************

functions which update components.

**************************/

use super::manager::WinManager;
use crate::proclist::list::ProcList;
use crate::render::meter::*;
use crate::resource::{disk, dmesg, mem, net};

// update uptime and return interval.
pub fn update_uptime(plist: &mut ProcList) -> f64 {
  let prev_uptime = plist.uptime.clone();
  plist.uptime.update();
  let current_uptime = &plist.uptime;

  let interval = current_uptime.uptime - prev_uptime.uptime;
  if interval <= 0.0 {
    1.0
  } else {
    interval
  }
}

pub fn update_cpu_meters(wm: &mut WinManager) -> Option<()> {
  let cpumanager = wm.cpumanager.as_mut()?;
  cpumanager.set_cpus(&wm.plist.cpus);
  cpumanager.render();
  Some(())
}

pub fn update_task_meter(wm: &mut WinManager) -> Option<()> {
  let taskmeter = wm.taskmeter.as_mut()?;
  wm.plist.loadaverage.update();
  wm.plist.uptime.update();

  taskmeter.set_values(&wm.plist);
  taskmeter.render();
  Some(())
}

pub fn update_process_meters(wm: &mut WinManager) -> Option<()> {
  let processmanager = wm.processmanager.as_mut()?;
  let sorted_procs = wm.plist.get_sorted_by_cpu();
  processmanager.set_sorted_procs(sorted_procs);
  processmanager.render();
  Some(())
}

pub fn update_cpugraph(wm: &mut WinManager) -> Option<()> {
  let cpu_graph = wm.cpu_graph.as_mut()?;
  let ave_cpu = &wm.plist.aggregated_cpu;

  cpu_graph.set_cpu(ave_cpu);
  cpu_graph.render();
  Some(())
}

pub fn update_netmeter(wm: &mut WinManager) -> Option<()> {
  let netmeter = wm.netmeter.as_mut()?;
  let statistics = net::get_statistic_all();

  netmeter.set_statistics(&statistics);
  netmeter.render();
  Some(())
}

pub fn update_iometer(wm: &mut WinManager) -> Option<()> {
  let iometer = wm.iometer.as_mut()?;
  let statistics = disk::get_diskstats();
  iometer.set_statistics(statistics, wm.update_interval);
  iometer.render();
  Some(())
}

pub fn update_memmeter(wm: &mut WinManager) -> Option<()> {
  let memmeter = wm.memmeter.as_mut()?;
  let usage = mem::MemInfo::new();
  memmeter.set_usage(&usage);
  memmeter.render();
  Some(())
}

pub fn update_inputmeter(wm: &mut WinManager) -> Option<()> {
  let inputmeter = wm.inputmeter.as_mut()?;
  inputmeter.update_inputs();
  inputmeter.render();
  Some(())
}

pub fn update_dmesglist(wm: &mut WinManager) -> Option<()> {
  let dmesglist = wm.dmesglist.as_mut()?;
  let dmesgs = dmesg::get_kmsgs();
  dmesglist.set_dmesg(dmesgs);
  dmesglist.render();
  Some(())
}

pub fn update_commandbox(wm: &mut WinManager) -> Option<()> {
  let commandbox = wm.commandbox.as_mut()?;
  commandbox.render();
  Some(())
}
