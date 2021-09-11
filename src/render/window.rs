use crate::command::commander;
use crate::consts::*;
use crate::layout::{calc, config};
use crate::proclist::list;
use crate::render::{
  color, commandbox, cpugraph, cpumanager, inputmeter, memmeter, meter::Meter,
  processmeter_manager, taskmeter,
};
use crate::resource::mem;
use ncurses::*;
use signal_hook::{consts::*, iterator::Signals};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
enum ThreadSignal {
  DoUpdate,
  Resize,
  Mouse(MEVENT),
  Command(char),
  CommandActivate,
  Quit,
}

pub struct WinManager {
  pub mainwin: WINDOW,
  pub screen_height: i32,
  pub screen_width: i32,
  pub plist: list::ProcList,

  // CPU meters
  cpumanager: Option<cpumanager::CpuManager>,

  // Task Meter
  taskmeter: Option<taskmeter::TaskMeter>,

  // Process meters
  processmanager: Option<processmeter_manager::ProcessMeterManager>,

  // CPU graph
  pub cpu_graph: Option<cpugraph::CpuGraph>,

  // Memory meter
  pub memmeter: Option<memmeter::MemMeter>,

  // Input meter
  pub inputmeter: Option<inputmeter::InputMeter>,

  // Layout of components
  layout: Vec<config::Layout>,

  // CommandBox
  pub commandbox: Option<commandbox::CommandBox>,
  pub commander: Arc<Mutex<commander::Commander>>,

  // cursor
  pub cur_x: i32,
  pub cur_y: i32,
}

impl WinManager {
  fn initialize() -> WINDOW {
    setlocale(LcCategory::all, "");
    let mainwin = initscr();
    cbreak();
    intrflush(mainwin, true);
    keypad(stdscr(), true);
    noecho();
    color::initialize_color();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    mousemask((ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION) as u32, None);
    refresh();
    mainwin
  }

  // XXX check of layout file should occur before init of screen.
  // or, should terminate windows before panic to show appropriate message.
  pub fn init_meters(&mut self) {
    use config::{Height, MeterName::*, Size};

    self.cur_x = 0;
    self.cur_y = 0;
    let mut max_height_in_line = 0;

    let layouts = config::read_layout_config();
    for layout in &layouts {
      let mut go_newline = false;
      let width = match layout.ratio {
        Size::Ratio(r) => (self.screen_width as f64 * r) as i32,
        Size::Rest => {
          go_newline = true;
          self.screen_width - self.cur_x
        }
      };
      let height = match layout.height {
        Height::Line(l) => l as i32,
        Height::Rest => self.screen_height - self.cur_y,
        Height::Minus(l) => (self.screen_height - self.cur_y) - l as i32,
      };
      max_height_in_line = std::cmp::max(max_height_in_line, height);

      match layout.name {
        CpuMeter => self.init_cpumanager(height, width),
        CpuGraph => self.init_cpugraph(height, width),
        TaskMeter => self.init_taskmeter(height, width),
        MemMeter => self.init_memmeter(height, width),
        Inputs => self.init_inputmeter(height, width),
        ProcMeter => self.init_process_meters(height, width),
        CommandBox => self.init_commandbox(height, width),
        Empty => {}
      }

      self.cur_x += width;
      if go_newline {
        self.cur_y += max_height_in_line;
        max_height_in_line = 0;
        self.cur_x = 0;
      }
    }

    self.layout = layouts;
  }

  fn init_cpumanager(&mut self, height: i32, width: i32) {
    self.cpumanager = Some(cpumanager::CpuManager::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  fn init_taskmeter(&mut self, height: i32, width: i32) {
    self.taskmeter = Some(taskmeter::TaskMeter::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  fn init_process_meters(&mut self, height: i32, width: i32) {
    self.processmanager = Some(processmeter_manager::ProcessMeterManager::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  fn init_cpugraph(&mut self, height: i32, width: i32) {
    self.cpu_graph = Some(cpugraph::CpuGraph::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  fn init_memmeter(&mut self, height: i32, width: i32) {
    self.memmeter = Some(memmeter::MemMeter::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  pub fn init_inputmeter(&mut self, height: i32, width: i32) {
    self.inputmeter = Some(inputmeter::InputMeter::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  pub fn init_commandbox(&mut self, height: i32, width: i32) {
    self.commandbox = Some(commandbox::CommandBox::init_meter(
      self.mainwin,
      self,
      height,
      width,
      self.cur_y,
      self.cur_x,
    ));
  }

  fn update_cpu_meters(&mut self) -> Option<()> {
    let cpumanager = self.cpumanager.as_mut()?;
    self.plist.update_cpus();
    cpumanager.set_cpus(&self.plist.cpus);
    cpumanager.render();
    Some(())
  }

  fn update_task_meter(&mut self) -> Option<()> {
    let taskmeter = self.taskmeter.as_mut()?;
    self.plist.loadaverage.update();
    self.plist.uptime.update();

    taskmeter.set_values(&self.plist);
    taskmeter.render();
    Some(())
  }

  fn update_process_meters(&mut self) -> Option<()> {
    let processmanager = self.processmanager.as_mut()?;
    let sorted_procs = self.plist.get_sorted_by_cpu();
    processmanager.set_sorted_procs(sorted_procs);
    processmanager.render();
    Some(())
  }

  fn update_cpugraph(&mut self) -> Option<()> {
    let cpu_graph = self.cpu_graph.as_mut()?;
    let ave_cpu = &self.plist.aggregated_cpu;

    cpu_graph.set_cpu(ave_cpu);
    cpu_graph.render();
    Some(())
  }

  fn update_memmeter(&mut self) -> Option<()> {
    let memmeter = self.memmeter.as_mut()?;
    let usage = mem::MemInfo::new();
    memmeter.set_usage(&usage);
    memmeter.render();
    Some(())
  }

  fn update_inputmeter(&mut self) -> Option<()> {
    let inputmeter = self.inputmeter.as_mut()?;
    inputmeter.update_inputs();
    inputmeter.render();
    Some(())
  }

  fn update_commandbox(&mut self) -> Option<()> {
    let commandbox = self.commandbox.as_mut()?;
    commandbox.render();
    Some(())
  }

  fn resize_cpumanager(&mut self, height: i32, width: i32) -> Option<()> {
    let cpumanager = self.cpumanager.as_mut()?;
    cpumanager.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_taskmeter(&mut self, height: i32, width: i32) -> Option<()> {
    let taskmeter = self.taskmeter.as_mut()?;
    taskmeter.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_cpugraph(&mut self, height: i32, width: i32) -> Option<()> {
    let cpugraph = self.cpu_graph.as_mut()?;
    cpugraph.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_memmeter(&mut self, height: i32, width: i32) -> Option<()> {
    let memmeter = self.memmeter.as_mut()?;
    memmeter.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_inputmeter(&mut self, height: i32, width: i32) -> Option<()> {
    let inputmeter = self.inputmeter.as_mut()?;
    inputmeter.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_process_meters(&mut self, height: i32, width: i32) -> Option<()> {
    let processmanager = self.processmanager.as_mut()?;
    processmanager.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_commandbox(&mut self, height: i32, width: i32) -> Option<()> {
    let commandbox = self.commandbox.as_mut()?;
    commandbox.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
  }

  fn resize_meters(&mut self) {
    use config::{Height, MeterName::*, Size};

    self.cur_x = 0;
    self.cur_y = 0;
    let layouts = &self.layout.clone();
    let mut max_height_in_line = 0;

    for layout in layouts {
      let mut go_newline = false;
      let width = match layout.ratio {
        Size::Ratio(r) => (self.screen_width as f64 * r) as i32,
        Size::Rest => {
          go_newline = true;
          self.screen_width - self.cur_x
        }
      };
      let height = match layout.height {
        Height::Line(l) => l as i32,
        Height::Rest => self.screen_height - self.cur_y,
        Height::Minus(l) => (self.screen_height - self.cur_y) - l as i32,
      };
      max_height_in_line = std::cmp::max(max_height_in_line, height);

      match layout.name {
        CpuMeter => self.resize_cpumanager(height, width),
        CpuGraph => self.resize_cpugraph(height, width),
        TaskMeter => self.resize_taskmeter(height, width),
        MemMeter => self.resize_memmeter(height, width),
        Inputs => self.resize_inputmeter(height, width),
        ProcMeter => self.resize_process_meters(height, width),
        CommandBox => self.resize_commandbox(height, width),
        Empty => None,
      };

      self.cur_x += width;
      if go_newline {
        self.cur_y += max_height_in_line;
        max_height_in_line = 0;
        self.cur_x = 0;
      }
    }
  }

  fn finish() {
    endwin();
  }

  // Handle all the signal from threads.
  // if true is returned, main thread should exit immediately.
  fn handle_thread_signal(&mut self, sig: &ThreadSignal) -> bool {
    use ThreadSignal::*;
    match sig {
      DoUpdate => {
        self.update_cpu_meters();
        self.update_cpugraph();
        self.update_inputmeter();
        self.update_memmeter();
        self.update_commandbox(); // XXX

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
          if !proc.is_updated {
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

      Quit => true,

      Resize => {
        flushinp();
        // get new term size
        endwin();
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

      Mouse(mevent) => {
        use config::MeterName::*;
        let bstate = mevent.bstate;
        let pos_x = mevent.x;
        let pos_y = mevent.y;
        let mut scroll = 0;

        if (bstate & BUTTON1_CLICKED as u32) != 0 {
          if let Some((layout, (y, x))) = calc::get_layout_from_click(
            &self.layout,
            self.screen_height,
            self.screen_width,
            pos_y,
            pos_x,
          ) {
            match layout.name {
              CpuMeter => self.cpumanager.as_mut().unwrap().handle_click(y, x),
              CpuGraph => self.cpu_graph.as_mut().unwrap().handle_click(y, x),
              TaskMeter => self.taskmeter.as_mut().unwrap().handle_click(y, x),
              MemMeter => self.memmeter.as_mut().unwrap().handle_click(y, x),
              Inputs => self.inputmeter.as_mut().unwrap().handle_click(y, x),
              ProcMeter => self.processmanager.as_mut().unwrap().handle_click(y, x),
              CommandBox => {}
              Empty => {}
            };
          }
        } else if (bstate & BUTTON4_PRESSED as u32) != 0 {
          // wheel up
          scroll = -1;
        } else if (bstate & BUTTON5_PRESSED as u32) != 0 {
          // wheel down
          scroll = 1;
        }

        // handle scroll
        if scroll != 0 {
          if let Some((layout, (_, _))) = calc::get_layout_from_click(
            &self.layout,
            self.screen_height,
            self.screen_width,
            pos_y,
            pos_x,
          ) {
            match layout.name {
              ProcMeter => self.processmanager.as_mut().unwrap().handle_scroll(scroll),
              _ => {}
            };
          }
        }

        false
      }

      Command(c) => {
        let mut commander = self.commander.lock().unwrap();
        let commandbox = self.commandbox.as_mut().unwrap();
        if *c == '\n' {
          let command = commandbox.do_enter();
          let result = commander.execute(&command);
          commandbox.set_result(&result);
        } else {
          commandbox.addstr(&c.to_string());
        }
        false
      }

      CommandActivate => {
        let mut commander = self.commander.lock().unwrap();
        let commandbox = self.commandbox.as_mut().unwrap();
        commander.start_input();
        commandbox.start_input();

        false
      }
    }
  }

  pub fn qloop(&mut self) {
    use ThreadSignal::*;
    // channel to send signal from children.
    let (tx, rx) = mpsc::channel();

    let update_timer_tx = tx.clone();
    let _update_timer = thread::spawn(move || loop {
      update_timer_tx.send(DoUpdate).unwrap();
      thread::sleep(Duration::from_millis(UPDATE_INTERVAL));
    });

    let input_sender_tx = tx.clone();
    let input_commander = self.commander.clone();
    let _input_sender = thread::spawn(move || loop {
      let ch = getch();
      match ch {
        // special inputs
        KEY_MOUSE => {
          let mut mevent: MEVENT = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
          getmouse(&mut mevent);
          input_sender_tx.send(ThreadSignal::Mouse(mevent)).unwrap();
        }
        KEY_BACKSPACE => {
          if input_commander.lock().unwrap().is_active() {
            input_sender_tx.send(Command('\x08')).unwrap();
            continue;
          }
        }

        // normal key input
        _ => {
          let c = match std::char::from_u32(ch as u32) {
            Some(_c) => _c,
            None => continue,
          };
          // if commander is active, send all normal key as Command signal.
          if input_commander.lock().unwrap().is_active() {
            input_sender_tx.send(Command(c)).unwrap();
            continue;
          }
          // otherwise, check the key and send appropriate signal.
          match c {
            'q' => {
              input_sender_tx.send(Quit).unwrap();
              break;
            }
            'U' => {
              input_sender_tx.send(DoUpdate).unwrap();
            }
            ';' => {
              input_sender_tx.send(CommandActivate).unwrap();
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
          SIGWINCH => sigwinch_tx.send(Resize).unwrap(),
          SIGKILL | SIGTERM | SIGSTOP | SIGINT => {
            sigwinch_tx.send(Quit).unwrap();
            return;
          }
          _ => {}
        }
      }
    });

    drop(tx);

    // main handler
    loop {
      let sig = rx.recv().unwrap();
      if self.handle_thread_signal(&sig) {
        endwin();
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
      memmeter: None,
      taskmeter: None,
      processmanager: None,
      cpu_graph: None,
      inputmeter: None,
      commandbox: None,
      commander: Arc::new(Mutex::new(commander::Commander::new())),
      layout: vec![],
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

impl Default for WinManager {
  fn default() -> Self {
    Self::new()
  }
}
