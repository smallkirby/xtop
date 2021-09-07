use crate::consts::*;
use crate::proclist::list;
use crate::render::{
  cpugraph, cpumanager, meter::Meter, moragraph, processmeter_manager, taskmeter,
};
use crate::resource::process;
use ncurses::*;
use signal_hook::{consts::SIGWINCH, iterator::Signals};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
enum ThreadSignal {
  DOUPDATE,
  RESIZE,
  QUIT,
}

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
  processmanager: Option<processmeter_manager::ProcessMeterManager>,

  // CPU graph
  pub cpu_graph: Option<cpugraph::CPUGraph>,

  // Mora graph
  pub mora_graph: Option<moragraph::MoraGraph>,

  // cursor
  pub cur_x: i32,
  pub cur_y: i32,
}

impl WinManager {
  fn initialize() -> WINDOW {
    setlocale(LcCategory::all, "");
    let mainwin = initscr();
    cbreak();
    noecho();
    intrflush(mainwin, true);
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();
    mainwin
  }

  pub fn init_meters(&mut self) {
    self.cur_x = 0;
    self.cur_y = 0;
    self.init_cpumanager();
    self.cur_y += 1;
    self.init_taskmeter();
    self.cur_y += 1;
    self.init_cpugraph();
    self.init_moragraph();
    self.init_process_meters();
  }

  fn init_cpumanager(&mut self) {
    let width = self.screen_width;
    self.cpumanager = Some(cpumanager::CPUManager::init_meter(
      self.mainwin,
      self,
      None,
      Some(width),
      self.cur_y,
      self.cur_x,
    ));
    self.cur_y += self.cpumanager.as_mut().unwrap().height;
  }

  fn init_taskmeter(&mut self) {
    let height = 3;
    let width = self.screen_width;
    self.taskmeter = Some(taskmeter::TaskMeter::init_meter(
      self.mainwin,
      self,
      Some(height),
      Some(width),
      self.cur_y,
      self.cur_x,
    ));
    self.cur_y += height;
  }

  fn init_process_meters(&mut self) {
    // init entire window for cpumeters.
    let width = self.screen_width;
    let height = std::cmp::max(self.screen_height - self.cur_y, 1);
    let y = self.cur_y;
    let x = 0;
    self.processmanager = Some(processmeter_manager::ProcessMeterManager::init_meter(
      self.mainwin,
      self,
      Some(height),
      Some(width),
      y,
      x,
    ));
    wrefresh(self.processmanager.as_mut().unwrap().win);
    self.cur_y += self.processmanager.as_mut().unwrap().height;
  }

  fn init_cpugraph(&mut self) {
    let x = 0;
    let height = 10;
    let width = self.screen_width / 3 * 2;
    self.cpu_graph = Some(cpugraph::CPUGraph::init_meter(
      self.mainwin,
      self,
      Some(height),
      Some(width),
      self.cur_y,
      x,
    ));
  }

  pub fn init_moragraph(&mut self) {
    let x = self.screen_width / 3 * 2;
    let height = 10;
    let width = self.screen_width / 3 * 1;
    self.mora_graph = Some(moragraph::MoraGraph::init_meter(
      self.mainwin,
      self,
      Some(height),
      Some(width),
      self.cur_y,
      x,
    ));
    self.cur_y += self.mora_graph.as_ref().unwrap().height;
  }

  fn update_cpu_meters(&mut self) {
    let cpumanager = self.cpumanager.as_mut().unwrap();
    self.plist.update_cpus();
    cpumanager.set_cpus(&self.plist.cpus);
    cpumanager.render();
  }

  fn update_task_meter(&mut self) {
    let taskmeter = self.taskmeter.as_mut().unwrap();
    self.plist.loadaverage.update();
    self.plist.uptime.update();

    taskmeter.set_values(&self.plist);
    taskmeter.render();
  }

  fn update_process_meters(&mut self) {
    let processmanager = self.processmanager.as_mut().unwrap();
    let sorted_procs = self.plist.get_sorted_by_cpu();
    processmanager.set_sorted_procs(sorted_procs);
    processmanager.render();
  }

  fn update_cpugraph(&mut self) {
    let cpu_graph = self.cpu_graph.as_mut().unwrap();
    let ave_cpu = &self.plist.aggregated_cpu;

    cpu_graph.set_cpu(ave_cpu);
    cpu_graph.render();
  }

  fn update_moragraph(&mut self) {
    let mora_graph = self.mora_graph.as_mut().unwrap();
    mora_graph.render();
  }

  fn resize_cpumanager(&mut self) {
    let cpumanager = self.cpumanager.as_mut().unwrap();
    cpumanager.resize(self.mainwin, None, Some(self.screen_width), 0, 0);
    self.cur_y += cpumanager.height;
  }

  fn resize_taskmeter(&mut self) {
    let taskmeter = self.taskmeter.as_mut().unwrap();
    let width = self.screen_width;
    taskmeter.resize(self.mainwin, None, Some(width), self.cur_y, 0);
    self.cur_y += taskmeter.height;
  }

  fn resize_cpugraph(&mut self) {
    let cpugraph = self.cpu_graph.as_mut().unwrap();
    let width = self.screen_width / 3 * 2;

    cpugraph.resize(self.mainwin, None, Some(width), self.cur_y, 0);
  }

  fn resize_moragraph(&mut self) {
    let x = self.screen_width / 3 * 2;
    let moragraph = self.mora_graph.as_mut().unwrap();
    let width = self.screen_width / 3 * 1;
    moragraph.resize(self.mainwin, None, Some(width), self.cur_y, x);
    self.cur_y += moragraph.height;
  }

  fn resize_process_meters(&mut self) {
    let processmanager = self.processmanager.as_mut().unwrap();
    let mut x = 0;
    let mut y = 0;

    getbegyx(processmanager.win, &mut y, &mut x);
    let start_y = y;
    let height = std::cmp::max(self.screen_height - start_y as i32, 2);
    processmanager.resize(
      self.mainwin,
      Some(height),
      Some(self.screen_width),
      start_y,
      0,
    );
  }

  fn resize_meters(&mut self) {
    self.cur_x = 0;
    self.cur_y = 0;
    werase(self.mainwin);
    self.resize_cpumanager();
    self.cur_y += 1;
    self.resize_taskmeter();
    self.cur_y += 1;
    self.resize_cpugraph();
    self.resize_moragraph();
    self.resize_process_meters();
  }

  fn finish() {
    endwin();
  }

  // Handle all the signal from threads.
  // if true is returned, main thread should exit immediately.
  fn handle_thread_signal(&mut self, sig: &ThreadSignal) -> bool {
    use ThreadSignal::*;
    match sig {
      DOUPDATE => {
        self.update_cpu_meters();
        self.update_cpugraph();
        self.update_moragraph();

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

        false
      }

      QUIT => true,

      RESIZE => {
        flushinp();
        // get new term size
        refresh();
        getmaxyx(
          self.mainwin,
          &mut self.screen_height,
          &mut self.screen_width,
        );
        wresize(self.mainwin, self.screen_height, self.screen_width);
        // resize/redraw
        self.resize_meters();
        flushinp();
        false
      }
    }
  }

  pub fn qloop(&mut self) {
    use ThreadSignal::*;
    let (tx, rx) = mpsc::channel();

    let update_timer_tx = tx.clone();
    let _update_timer = thread::spawn(move || loop {
      update_timer_tx.send(DOUPDATE).unwrap();
      thread::sleep(Duration::from_millis(UPDATE_INTERVAL));
    });

    let input_sender_tx = tx.clone();
    let _input_sender = thread::spawn(move || loop {
      let ch = getch();
      match ch {
        // special inputs

        // normal key input
        _ => {
          let c = match std::char::from_u32(ch as u32) {
            Some(_c) => _c,
            None => continue,
          };
          match c {
            'q' => {
              input_sender_tx.send(QUIT).unwrap();
              break;
            }
            'U' => {
              input_sender_tx.send(DOUPDATE).unwrap();
            }
            _ => {}
          }
        }
      };
    });

    let sigwinch_tx = tx.clone();
    let mut signals = Signals::new(&[SIGWINCH]).unwrap();
    let _sigwinch_notifier = thread::spawn(move || loop {
      for sig in signals.forever() {
        match sig {
          SIGWINCH => sigwinch_tx.send(RESIZE).unwrap(),
          _ => {}
        }
      }
    });

    // main handler
    loop {
      let sig = rx.recv().unwrap();
      if self.handle_thread_signal(&sig) {
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
      processmanager: None,
      cpu_graph: None,
      mora_graph: None,
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
