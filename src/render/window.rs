use crate::proclist::list;
use crate::render::cpumeter;
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

  pub fn update_cpu_meters(&mut self) {
    // XXX update_cpus() must be called right before recurse_proc_tree()
    self.plist.average_period = self.plist.update_cpus();
    for i in 0..self.cpumeters.len() {
      self.cpumeters[i].render(&mut self.plist.cpus[i]);
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
      self.plist.recurse_proc_tree(None, "/proc");
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
    }
  }
}

impl Drop for WinManager {
  fn drop(&mut self) {
    Self::finish();
  }
}
