use crate::command::commander;
use crate::consts::*;
use crate::layout::{calc, config, config::MeterName};
use crate::proclist::list;
use crate::render::{
  color, commandbox, cpugraph, cpumanager, dmesglist, inputmeter, iometer, memmeter, meter::Meter,
  netmeter, processmeter_manager, taskmeter,
};
use crate::resource::{dmesg, mem, net};
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

  // Net graph
  pub netmeter: Option<netmeter::NetMeter>,

  // IO Meter
  pub iometer: Option<iometer::IoMeter>,

  // Memory meter
  pub memmeter: Option<memmeter::MemMeter>,

  // Input meter
  pub inputmeter: Option<inputmeter::InputMeter>,

  // Dmesg list
  pub dmesglist: Option<dmesglist::DmesgList>,

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

  pub fn init_meters(&mut self) {
    let layouts = config::read_layout_config();
    let fixed_layouts = calc::get_fixed_layouts(&layouts, self.screen_height, self.screen_width);

    for layout in &fixed_layouts {
      let height = layout.height;
      let width = layout.width;
      self.cur_y = layout.y;
      self.cur_x = layout.x;
      self.init_meter_general(layout.name.clone(), height, width);
    }

    self.layout = layouts;
  }

  fn init_meter_general(&mut self, name: MeterName, height: i32, width: i32) {
    use crate::layout::config::MeterName::*;
    match name {
      CpuMeter => {
        self.cpumanager = Some(cpumanager::CpuManager::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      CpuGraph => {
        self.cpu_graph = Some(cpugraph::CpuGraph::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      TaskMeter => {
        self.taskmeter = Some(taskmeter::TaskMeter::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      MemMeter => {
        self.memmeter = Some(memmeter::MemMeter::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      Inputs => {
        self.inputmeter = Some(inputmeter::InputMeter::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      DmesgList => {
        self.dmesglist = Some(dmesglist::DmesgList::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      ProcMeter => {
        self.processmanager = Some(processmeter_manager::ProcessMeterManager::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      NetMeter => {
        self.netmeter = Some(netmeter::NetMeter::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      IoMeter => {
        self.iometer = Some(iometer::IoMeter::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      CommandBox => {
        self.commandbox = Some(commandbox::CommandBox::init_meter(
          self.mainwin,
          self,
          height,
          width,
          self.cur_y,
          self.cur_x,
        ))
      }
      Empty => return,
    };
  }

  fn resize_meter_general(&mut self, name: MeterName, height: i32, width: i32) -> Option<()> {
    use crate::layout::config::MeterName::*;

    let meter: Box<&mut dyn Meter> = match name {
      CpuMeter => Box::new(self.cpumanager.as_mut()?),
      CpuGraph => Box::new(self.cpu_graph.as_mut()?),
      TaskMeter => Box::new(self.taskmeter.as_mut()?),
      MemMeter => Box::new(self.memmeter.as_mut()?),
      Inputs => Box::new(self.inputmeter.as_mut()?),
      DmesgList => Box::new(self.dmesglist.as_mut()?),
      ProcMeter => Box::new(self.processmanager.as_mut()?),
      NetMeter => Box::new(self.netmeter.as_mut()?),
      IoMeter => Box::new(self.iometer.as_mut()?),
      CommandBox => Box::new(self.commandbox.as_mut()?),
      Empty => return Some(()),
    };
    meter.resize(self.mainwin, height, width, self.cur_y, self.cur_x);
    Some(())
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

  fn update_netmeter(&mut self) -> Option<()> {
    let netmeter = self.netmeter.as_mut()?;
    let statistics = net::get_statistic_all();

    netmeter.set_statistics(&statistics);
    netmeter.render();
    Some(())
  }

  fn update_iometer(&mut self) -> Option<()> {
    let iometer = self.iometer.as_mut()?;
    iometer.render();
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

  fn update_dmesglist(&mut self) -> Option<()> {
    let dmesglist = self.dmesglist.as_mut()?;
    let dmesgs = dmesg::get_kmsgs();
    dmesglist.set_dmesg(dmesgs);
    dmesglist.render();
    Some(())
  }

  fn update_commandbox(&mut self) -> Option<()> {
    let commandbox = self.commandbox.as_mut()?;
    commandbox.render();
    Some(())
  }

  fn resize_meters(&mut self) {
    let layouts = config::read_layout_config();
    let fixed_layouts = calc::get_fixed_layouts(&layouts, self.screen_height, self.screen_width);

    for layout in &fixed_layouts {
      let height = layout.height;
      let width = layout.width;
      self.cur_y = layout.y;
      self.cur_x = layout.x;
      self.resize_meter_general(layout.name.clone(), height, width);
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
        self.update_netmeter();
        self.update_iometer();
        self.update_dmesglist();

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

        self.update_commandbox(); // should be at last

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
          if let Some((layout_name, (y, x))) = calc::get_layout_from_click(
            &self.layout,
            self.screen_height,
            self.screen_width,
            pos_y,
            pos_x,
          ) {
            match layout_name {
              CpuMeter => self.cpumanager.as_mut().unwrap().handle_click(y, x),
              CpuGraph => self.cpu_graph.as_mut().unwrap().handle_click(y, x),
              TaskMeter => self.taskmeter.as_mut().unwrap().handle_click(y, x),
              MemMeter => self.memmeter.as_mut().unwrap().handle_click(y, x),
              Inputs => self.inputmeter.as_mut().unwrap().handle_click(y, x),
              ProcMeter => self.processmanager.as_mut().unwrap().handle_click(y, x),
              DmesgList => self.dmesglist.as_mut().unwrap().handle_click(y, x),
              NetMeter => self.netmeter.as_mut().unwrap().handle_click(y, x),
              IoMeter => self.iometer.as_mut().unwrap().handle_click(y, x),
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
          if let Some((layout_name, (_, _))) = calc::get_layout_from_click(
            &self.layout,
            self.screen_height,
            self.screen_width,
            pos_y,
            pos_x,
          ) {
            match layout_name {
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
          let result = commander.execute(&command, self.processmanager.as_mut().unwrap());
          commandbox.set_result(&result);
        } else {
          commandbox.addstr(&c.to_string(), &mut commander);
        }
        false
      }

      CommandActivate => {
        let mut commander = self.commander.lock().unwrap();
        let commandbox = self.commandbox.as_mut().unwrap();
        commander.start_input();
        commandbox.start_input(&mut commander);

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

  fn check_validity() -> Result<(), String> {
    // check validity of layout file
    let layout = config::read_layout_config();
    match calc::check_layout_validity(&layout) {
      Ok(()) => {}
      Err(s) => return Err(s),
    }

    Ok(())
  }

  pub fn new() -> Self {
    // before initialize, check some validity
    if let Err(s) = Self::check_validity() {
      eprintln!("Error: {}", s);
      std::process::exit(1);
    }

    // create windows
    let mainwin = Self::initialize();
    let mut screen_height = 0;
    let mut screen_width = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    // init process list
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
      netmeter: None,
      iometer: None,
      inputmeter: None,
      dmesglist: None,
      commandbox: None,
      layout: vec![],
      cur_x: 0,
      cur_y: 0,
      commander: Arc::new(Mutex::new(commander::Commander::new())),
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
