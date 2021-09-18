/*****

Implementation of DockerMeter.
DockerMeter shows the simplified list of running containers.

*******/

use crate::render::{color::*, executer::manager::WinManager, meter::*};
use crate::resource::docker::DockerExtInfo;

use ncurses::*;

static NAME_MAXLEN: usize = 20;
static CPU_MAXLEN: usize = 7;

pub struct DockerMeter {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  containers: Vec<DockerExtInfo>,
}

impl DockerMeter {
  pub fn set_containers(&mut self, containers: Vec<DockerExtInfo>) {
    let mut new_containers = vec![];
    for container in containers.iter() {
      if !self.containers.contains(container) {
        new_containers.push(container);
      }
    }

    self
      .containers
      .append(&mut new_containers.into_iter().map(|c| c.clone()).collect());
    for container in &mut self.containers {
      container.update();
    }
  }
}

impl Meter for DockerMeter {
  fn render(&mut self) {
    use crate::util::firstn;

    let win = self.win;
    werase(win);

    // draw each entries
    let mut cy = 1;
    for container in &self.containers {
      let mut cx = 1;

      let name = firstn(&container.psinfo.name, NAME_MAXLEN);
      mvwaddstr(win, cy, cx, &name);
      cx += NAME_MAXLEN as i32 + 1;

      let id = &container.psinfo.short_id;
      mvwaddstr(win, cy, cx, id);
      cx += id.len() as i32 + 1;

      let uptime = container.psinfo.uptime.to_string();
      mvwaddstr(win, cy, cx, &uptime);
      cx += uptime.len() as i32 + 1;

      let cpuusage = &format!("{:>3.2}%", &container.cpuusage * 100.0);
      mvwaddstr(win, cy, cx, &cpuusage);
      cx += CPU_MAXLEN as i32 + 1;

      cx = 1 + NAME_MAXLEN as i32 + 1;
      cy += 1;

      let ports = &container.psinfo.ports;
      for port in ports {
        mvwaddstr(win, cy, cx, &format!("{} ", port));
        cx += port.len() as i32 + 1;
      }

      cy += 1;
    }

    // draw header
    box_(win, 0, 0);
    mvwaddstr_color(win, 0, 1, " Container ", cpair::PAIR_HEAD);

    wrefresh(win);
  }

  fn init_meter(
    _parent: WINDOW,
    _wm: &mut WinManager,
    height: i32,
    width: i32,
    y: i32,
    x: i32,
  ) -> Self
  where
    Self: Sized,
  {
    let win = newwin(height, width, y, x);
    wattron(win, COLOR_PAIR(cpair::DEFAULT));
    wbkgd(win, ' ' as chtype | COLOR_PAIR(cpair::DEFAULT) as chtype);
    box_(win, 0, 0);
    wrefresh(win);

    DockerMeter {
      width,
      height,
      win,
      containers: vec![],
    }
  }

  fn resize(&mut self, _parent: WINDOW, height: i32, width: i32, y: i32, x: i32) {
    self.height = height;
    self.width = width;
    wresize(self.win, height, width);
    werase(self.win);
    mvwin(self.win, y, x);

    self.render();
    wrefresh(self.win);
  }
}
