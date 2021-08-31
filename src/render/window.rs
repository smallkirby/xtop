use crate::proclist::list;
use crate::render::{cpumeter, taskmeter, processmeter};
use ncurses::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct WinManager {
  pub screen_height: i32,
  pub screen_width: i32,
  pub plist: list::ProcList,

  // CPU meters
  pub cpumeter_win: Option<WINDOW>,
  cpumeters: Vec<cpumeter::CPUMeter>,

  // Task Meter
  taskmeter: Option<taskmeter::TaskMeter>,

  // Process meters
  pub processmeter_win: Option<WINDOW>,
  processmeters: Vec<processmeter::ProcessMeter>,
}

impl WinManager {
  fn initialize() {
    initscr();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
  }

  pub fn init_cpu_meters(&mut self) {
    // init entire window for cpumeters.
    let (width, height) = cpumeter::winsize_require(&self);
    self.cpumeter_win = Some(newwin(height, width, 0, 0));
    wrefresh(self.cpumeter_win.unwrap());

    // init each windows of cpumeter inside parent window.
    self.cpumeters = cpumeter::init_meters(self);
    refresh();
  }

  pub fn init_taskmeter(&mut self) {
    self.taskmeter = Some(taskmeter::init_meter(self, 4, 0)); // XXX y must be calculated
    wrefresh(self.taskmeter.as_ref().unwrap().win);
  }

  pub fn init_process_meters(&mut self) {
    // init entire window for cpumeters.
    let width = self.screen_width; 
    let height = 30; // XXX
    self.processmeter_win = Some(newwin(height, width, 10, 0)); // XXX
    wrefresh(self.processmeter_win.unwrap());

    // init each windows of cpumeter inside parent window.
    self.processmeters = processmeter::init_meters(self, height);
    refresh();
  }

  pub fn update_cpu_meters(&mut self) {
    // XXX update_cpus() must be called right before recurse_proc_tree()
    self.plist.average_period = self.plist.update_cpus();
    for i in 0..self.cpumeters.len() {
      self.cpumeters[i].render(&mut self.plist.cpus[i]);
    }
  }

  pub fn update_task_meter(&mut self) {
    let taskmeter = self.taskmeter.as_mut().unwrap();
    self.plist.loadaverage.update();
    self.plist.uptime.update();
    taskmeter.render(&self.plist);
  }

  pub fn update_process_meters(&mut self) {
    let sorted_proc = self.plist.get_sorted_by_cpu(30); // XXX
    for (i, proc) in sorted_proc.into_iter().enumerate() {
      self.processmeters[i].render(&proc);
    }
  }

  fn finish() {
    endwin();
  }

  // just test func. should create thread pools.
  pub fn qloop(&mut self) {
    let (tx, rx) = mpsc::channel();

    let input_handler = thread::spawn(move || loop {
      let ch = getch() as u32;
      if std::char::from_u32(ch).unwrap() == 'q' {
        tx.send(true).unwrap();
        break;
      }
      refresh();
    });

    loop {
      thread::sleep(Duration::from_millis(1000));

      self.update_cpu_meters();

      // update values
      self.plist.total_tasks = 0;
      self.plist.userland_threads = 0;
      self.plist.kernel_threads = 0;
      for proc in self.plist.plist.values_mut() {
        proc.is_updated = false;
      } 
      self.plist.recurse_proc_tree(None, "/proc", self.plist.average_period);
      let mut deleted_pids = vec!();
      for proc in self.plist.plist.values_mut() {
        if proc.is_updated == false {
          deleted_pids.push(proc.pid);
        }
      }
      for pid in deleted_pids {
        self.plist.plist.remove(&pid);
      }

      self.update_task_meter();
      self.update_process_meters();

      refresh();
      if rx.try_recv().is_ok() {
        input_handler.join().unwrap();
        break;
      }
    }
  }

  pub fn new() -> Self {
    Self::initialize();
    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    let plist = list::ProcList::new();

    Self {
      plist,
      screen_height,
      screen_width,
      cpumeter_win: None,
      cpumeters: vec![],
      taskmeter: None,
      processmeter_win: None,
      processmeters: vec![],
    }
  }
}

impl Drop for WinManager {
  fn drop(&mut self) {
    Self::finish();
  }
}
