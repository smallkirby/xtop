use crate::proclist::list;
use crate::render::{cpumanager, meter::Meter, processmeter, taskmeter};
use ncurses::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct WinManager {
  pub mainwin: WINDOW,
  pub screen_height: i32,
  pub screen_width: i32,
  pub plist: list::ProcList,

  // CPU meters
  cpumanager: Option<cpumanager::CPUManager>,

  // Task Meter
  taskmeter: Option<taskmeter::TaskMeter>,

  // Process meters
  pub processmeter_win: Option<WINDOW>,
  processmeters: Vec<processmeter::ProcessMeter>,

  // cursor
  pub cur_x: i32,
  pub cur_y: i32,
}

impl WinManager {
  fn initialize() -> WINDOW {
    let mainwin = initscr();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
    mainwin
  }

  pub fn init_cpumanager(&mut self) {
    let width = self.screen_width;
    self.cpumanager = Some(cpumanager::CPUManager::init_meter(self.mainwin, self, None, Some(width), self.cur_y, self.cur_x));
    self.cur_y += self.cpumanager.as_mut().unwrap().height;
  }

  pub fn init_taskmeter(&mut self) {
    let height = 4;
    let width = self.screen_width;
    self.taskmeter = Some(taskmeter::TaskMeter::init_meter(
      self.mainwin, self, Some(height), Some(width), self.cur_y, self.cur_x,
    ));
    self.cur_y += height;
    wrefresh(self.taskmeter.as_ref().unwrap().win);
  }

  pub fn init_process_meters(&mut self) {
    // init entire window for cpumeters.
    let width = self.screen_width;
    let height = self.screen_height - self.cur_y;
    self.processmeter_win = Some(newwin(height, width, self.cur_y, 0));
    wrefresh(self.processmeter_win.unwrap());

    // init each windows of cpumeter inside parent window.
    self.processmeters = processmeter::init_meters(self.mainwin, self, height);
    self.cur_y += self.processmeters[0].height * self.processmeters.len() as i32;
    refresh();
  }

  pub fn update_cpu_meters(&mut self) {
    let cpumanager = self.cpumanager.as_mut().unwrap();
    self.plist.update_cpus();
    cpumanager.set_cpus(&self.plist.cpus);
    cpumanager.render();
  }

  pub fn update_task_meter(&mut self) {
    let taskmeter = self.taskmeter.as_mut().unwrap();
    self.plist.loadaverage.update();
    self.plist.uptime.update();

    taskmeter.set_values(&self.plist);
    taskmeter.render();
  }

  pub fn update_process_meters(&mut self) {
    let num = self.processmeters.len();
    let sorted_proc = self.plist.get_sorted_by_cpu(num);
    for (i, proc) in sorted_proc.into_iter().enumerate() {
      self.processmeters[i].set_proc(proc.clone());
      self.processmeters[i].render();
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
      self.plist.recurse_proc_tree(
        None,
        "/proc",
        self.plist.aggregated_cpu.totaltime_period as f64 / self.plist.cpus.len() as f64,
      );

      // delete tombed procs
      let mut deleted_pids = vec![];
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
    let mainwin = Self::initialize();
    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    let plist = list::ProcList::new();

    Self {
      mainwin,
      plist,
      screen_height,
      screen_width,
      cpumanager: None,
      taskmeter: None,
      processmeter_win: None,
      processmeters: vec![],
      cur_x: 0,
      cur_y: 0,
    }
  }
}

impl Drop for WinManager {
  fn drop(&mut self) {
    Self::finish();
  }
}
