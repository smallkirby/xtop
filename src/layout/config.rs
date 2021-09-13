/********

Implementation of Layout structure and layout config fle.

********/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeterName {
  CpuMeter,
  NetMeter,
  CpuGraph,
  TaskMeter,
  MemMeter,
  Inputs,
  DmesgList,
  ProcMeter,
  CommandBox,
  IoMeter,
  Empty,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Height {
  Rest,
  Line(u64),
  Minus(u64),
}

// size of width.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Size {
  Ratio(f64), // specify size of area in ratio. [0.0, 1.0]
  Rest,       // use all remained area.
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Layout {
  pub name: MeterName, // name of component.
  pub height: Height,  // height of component.
  pub ratio: Size,     // ratio of width.
}

use Height::*;
use Size::*;

static DEFAULT_LAYOUT: [Layout; 12] = [
  Layout {
    name: MeterName::CpuMeter,
    height: Line(4),
    ratio: Ratio(0.75),
  },
  Layout {
    name: MeterName::IoMeter,
    height: Line(4),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::Empty,
    height: Line(1),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::TaskMeter,
    height: Line(3),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::Empty,
    height: Line(1),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::CpuGraph,
    height: Line(15),
    ratio: Ratio(0.28),
  },
  Layout {
    name: MeterName::MemMeter,
    height: Line(15),
    ratio: Ratio(0.16667),
  },
  Layout {
    name: MeterName::NetMeter,
    height: Line(15),
    ratio: Ratio(0.22),
  },
  Layout {
    name: MeterName::Inputs,
    height: Line(15),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::ProcMeter,
    height: Height::Minus(1),
    ratio: Ratio(0.75),
  },
  Layout {
    name: MeterName::DmesgList,
    height: Height::Minus(1),
    ratio: Size::Rest,
  },
  Layout {
    name: MeterName::CommandBox,
    height: Height::Rest,
    ratio: Size::Rest,
  },
];

pub fn read_layout_config() -> Vec<Layout> {
  match std::fs::read_to_string("layout.json") {
    Ok(config_str) => serde_json::from_str(&config_str).unwrap(),
    Err(_) => DEFAULT_LAYOUT.to_vec(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reading_json() {
    let layout_from_file = read_layout_config();
    let layout_default = DEFAULT_LAYOUT.to_vec();
    assert_eq!(layout_from_file, layout_default);
  }
}
