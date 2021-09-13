use super::manager::WinManager;
use crate::layout::config::MeterName;
use crate::layout::{calc, config};
use crate::render::meter::*;

pub fn resize_meter_general(
  wm: &mut WinManager,
  name: MeterName,
  height: i32,
  width: i32,
) -> Option<()> {
  use crate::layout::config::MeterName::*;

  let meter: Box<&mut dyn Meter> = match name {
    CpuMeter => Box::new(wm.cpumanager.as_mut()?),
    CpuGraph => Box::new(wm.cpu_graph.as_mut()?),
    TaskMeter => Box::new(wm.taskmeter.as_mut()?),
    MemMeter => Box::new(wm.memmeter.as_mut()?),
    Inputs => Box::new(wm.inputmeter.as_mut()?),
    DmesgList => Box::new(wm.dmesglist.as_mut()?),
    ProcMeter => Box::new(wm.processmanager.as_mut()?),
    NetMeter => Box::new(wm.netmeter.as_mut()?),
    IoMeter => Box::new(wm.iometer.as_mut()?),
    CommandBox => Box::new(wm.commandbox.as_mut()?),
    Empty => return Some(()),
  };
  meter.resize(wm.mainwin, height, width, wm.cur_y, wm.cur_x);
  Some(())
}

pub fn resize_meters(wm: &mut WinManager) {
  let layouts = config::read_layout_config();
  let fixed_layouts = calc::get_fixed_layouts(&layouts, wm.screen_height, wm.screen_width);

  for layout in &fixed_layouts {
    let height = layout.height;
    let width = layout.width;
    wm.cur_y = layout.y;
    wm.cur_x = layout.x;
    resize_meter_general(wm, layout.name.clone(), height, width);
  }
}
