use crate::render::cpumeter;
use crate::resource::cpu;
use ncurses::*;

pub struct WinManager {
  pub screen_height: i32,
  pub screen_width: i32,

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

  pub fn init_cpu_meters(&mut self, cpus: &Vec<cpu::CPU>) {
    // init entire window for cpumeters.
    let (width, height) = cpumeter::winsize_require(&self, cpus);
    self.cpumeter_win = Some(newwin(height, width, 0, 0));
    wrefresh(self.cpumeter_win.unwrap());

    // init each windows of cpumeter inside parent window.
    self.cpumeters = cpumeter::init_meters(self, &cpus);
    refresh();
  }

  fn finish() {
    endwin();
  }

  pub fn qloop(&self) {
    loop {
      let ch = getch() as u32;
      if std::char::from_u32(ch).unwrap() == 'q' {
        break;
      }
    }
  }

  pub fn new() -> Self {
    Self::initialize();
    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    Self {
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

pub fn test_just_window(s: &str) {
  let mut cur_x = 0;
  let mut cur_y = 0;
  getyx(stdscr(), &mut cur_y, &mut cur_x);
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  let win = newwin(max_y - 2, max_x - 2, 1, 1);
  box_(win, 0, 0);
  wrefresh(win);

  wmove(win, 1, 1);
  refresh();
  wprintw(win, s);
  wrefresh(win);

  loop {
    let ch = getch() as u32;
    if std::char::from_u32(ch).unwrap() == 'q' {
      break;
    }
  }
  endwin();
}
