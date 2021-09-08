/*****

Definition of global constants.

*******/

pub static PAGESIZE_KB: i64 = 4;

pub static UPDATE_INTERVAL: u64 = 2000;

// XXX should decide dynamically.
// threshold of CPU usage. used for colorize.
pub static CPUUSAGE_MED_DANGER: f64 = 0.5;
pub static CPUUSAGE_HIGH_DANGER: f64 = 0.8;
