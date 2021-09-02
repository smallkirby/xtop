/*****

Implementation of MoraGraph.
MoraGraph shows the transition of moratorium.

*******/

use crate::render::meter::*;
use ncurses::*;

static MORA: &str = "      j@`  ``.JgHH=?\"\"W%`     TB\"7!(\"TY` db.\n    `-#!     _ue    ...      `.de.......  .M;\n   ` dD      .dMY9_T\"TMb      ?MY?!_??T#_  dR`\n     W]    `  j#      dD      .Wl     j@`` J@ \n     W]  `    .N+   .(#!       ?N.. .(H'  .W%\n     db. +#WNx _TBD`?=     `     ?!(\"=   .d$\n     .Me Wm(d9       . .. .-. .,        .dD\n      .T\\ _!`       .TH#WHB7HMYWH%     .\"!\n";

pub struct MoraGraph {
  pub height: i32,
  pub width: i32,
  pub win: WINDOW,
  pub morastring: Vec<String>,
}

impl Meter for MoraGraph {
  fn render(&mut self) {
    let win = self.win;
    // erase and draw box
    werase(win);
    box_(win, 0, 0);

    // draw picture
    let mut y = 1;
    let x = 10;
    let width = self.width - 1 - x;
    for _s in &self.morastring {
      let s = if _s.len() > width as usize {
        _s[0..width as usize].to_string()
      } else {
        _s.clone()
      };
      mvwaddstr(win, y as i32, x, &s);
      y += 1;
    }

    // draw header
    mvwaddstr(win, 0, 1, " Mora ");

    wrefresh(win);
  }

  fn init_meter(
    _parent: ncurses::WINDOW,
    _wm: &mut super::window::WinManager,
    height: Option<i32>,
    width: Option<i32>,
    y: i32,
    x: i32,
  ) -> Self {
    if height.is_none() || width.is_none() {
      panic!("height and width must be specified for MoraGraph::init_meter().");
    }
    let height = height.unwrap();
    let width = width.unwrap();
    let win = newwin(height, width, y, x);
    box_(win, 0, 0);
    wrefresh(win);

    let morastring: Vec<String> = String::from(MORA)
      .split("\n")
      .map(|s| s.to_string())
      .collect();

    MoraGraph {
      width,
      height,
      win,
      morastring,
    }
  }

  fn resize(&mut self) {
    todo!()
  }
}
