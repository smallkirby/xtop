use crate::proclist::list;
use crate::render::window;
use ncurses::*;

use super::window::WinManager;

static HEIGHT: i32 = 3;

#[derive(Debug)]
pub struct TaskMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
}

impl TaskMeter {
  pub fn render(&mut self, plist: &list::ProcList) {
    let win = self.win;
    let tasks = plist.total_tasks - plist.kernel_threads - plist.userland_threads;
    let uthr = plist.userland_threads;
    let kthr = plist.kernel_threads;
    mvwprintw(
      win,
      0,
      0,
      &format!("Tasks: {}, {} thr; {} kthr", tasks, uthr, kthr),
    );
    mvwprintw(win, 1, 0, &format!("Load Average: "));
    mvwprintw(win, 2, 0, &format!("Uptime: "));
    wrefresh(win);
  }
}

pub fn winsize_require(wm: &WinManager) -> (i32, i32) {
  let width = wm.screen_width / 2;
  let height = HEIGHT;

  (width, height)
}

pub fn init_meter(wm: &mut window::WinManager, y: i32, x: i32) -> TaskMeter {
  let (width, height) = winsize_require(wm);
  let win = create_meter_win(height, width, y, x);
  let mut meter = TaskMeter { height, width, win };
  meter.render(&mut wm.plist);

  meter
}

fn create_meter_win(height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = newwin(height, width, y, x);
  wrefresh(win);
  win
}
