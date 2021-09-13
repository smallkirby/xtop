/**************************

functions which initialize components.

**************************/

use super::manager::WinManager;
use crate::layout::{calc, config::*};
use crate::render::component::{
  commandbox, cpugraph, cpumanager, dmesglist, inputmeter, iometer, memmeter, netmeter,
  processmeter_manager, taskmeter,
};
use crate::render::meter::Meter;

pub fn init_meters(wm: &mut WinManager) {
  let layouts = read_layout_config();
  let fixed_layouts = calc::get_fixed_layouts(&layouts, wm.screen_height, wm.screen_width);

  for layout in &fixed_layouts {
    let height = layout.height;
    let width = layout.width;
    wm.cur_y = layout.y;
    wm.cur_x = layout.x;
    init_meter_general(wm, layout.name.clone(), height, width);
  }

  wm.layout = layouts;
}

pub fn init_meter_general(wm: &mut WinManager, name: MeterName, height: i32, width: i32) {
  use crate::layout::config::MeterName::*;
  match name {
    CpuMeter => {
      wm.cpumanager = Some(cpumanager::CpuManager::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    CpuGraph => {
      wm.cpu_graph = Some(cpugraph::CpuGraph::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    TaskMeter => {
      wm.taskmeter = Some(taskmeter::TaskMeter::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    MemMeter => {
      wm.memmeter = Some(memmeter::MemMeter::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    Inputs => {
      wm.inputmeter = Some(inputmeter::InputMeter::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    DmesgList => {
      wm.dmesglist = Some(dmesglist::DmesgList::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    ProcMeter => {
      wm.processmanager = Some(processmeter_manager::ProcessMeterManager::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    NetMeter => {
      wm.netmeter = Some(netmeter::NetMeter::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    IoMeter => {
      wm.iometer = Some(iometer::IoMeter::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    CommandBox => {
      wm.commandbox = Some(commandbox::CommandBox::init_meter(
        wm.mainwin, wm, height, width, wm.cur_y, wm.cur_x,
      ))
    }
    Empty => {}
  };
}
