use crate::proclist::list;
use crate::render::meter;
use crate::resource::{loadavg, uptime};
use ncurses::*;

use super::window::WinManager;

static HEIGHT: i32 = 3;

struct TaskValues {
  pub tasks: u32,
  pub uthr: u32,
  pub kthr: u32,
  pub loadaverage: loadavg::LoadAvg,
  pub uptime: uptime::Uptime,
}

pub struct TaskMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  vals: Option<TaskValues>,
}

impl TaskMeter {
  pub fn set_values(&mut self, plist: &list::ProcList) {
    let tasks = plist.total_tasks - plist.kernel_threads - plist.userland_threads;
    let uthr = plist.userland_threads;
    let kthr = plist.kernel_threads;
    let loadaverage = plist.loadaverage.clone();
    let uptime = plist.uptime.clone();
    self.vals = Some(TaskValues {
      tasks,
      uthr,
      kthr,
      loadaverage,
      uptime,
    });
  }
}

impl meter::Meter for TaskMeter {
  fn render(&mut self) {
    let win = self.win;
    wclear(win);

    let vals = match self.vals.as_ref() {
      Some(_vals) => _vals,
      None => {
        mvwprintw(win, 0, 0, &format!("[ERROR] task vals not initialized."));
        wrefresh(win);
        return;
      }
    };
    let ave = &vals.loadaverage;
    mvwprintw(
      win,
      0,
      0,
      &format!(
        "Tasks: {}, {} thr; {} kthr",
        vals.tasks, vals.uthr, vals.kthr
      ),
    );
    mvwprintw(
      win,
      1,
      0,
      &format!("Load Average: {} {} {}", ave.one, ave.five, ave.fifteen),
    );
    mvwprintw(
      win,
      2,
      0,
      &format!("Uptime: {}", vals.uptime.into_readable_string()),
    );
    wrefresh(win);
  }

  fn init_meter(parent: WINDOW, wm: &mut WinManager, height: Option<i32>, width: Option<i32>, y: i32, x: i32) -> Self {
    let (width, height) = winsize_require(wm);
    let win = create_meter_win(height, width, y, x);
    let mut meter = TaskMeter {
      height,
      width,
      win,
      vals: None,
    };
    meter.render();

    meter
  }
}

pub fn winsize_require(wm: &WinManager) -> (i32, i32) {
  let width = wm.screen_width / 2;
  let height = HEIGHT;

  (width, height)
}

fn create_meter_win(height: i32, width: i32, y: i32, x: i32) -> WINDOW {
  let win = newwin(height, width, y, x);
  wrefresh(win);
  win
}
