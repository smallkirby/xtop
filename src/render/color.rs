use ncurses::*;

mod COLORS {
  pub static BROWN_BLACK: i16 = 16;
  pub static WHITE: i16 = 17;
}

pub mod CPAIR {
  pub static DEFAULT: i16 = 1;
}

pub fn initialize_color() {
  use CPAIR::*;

  start_color();
  define_colors();

  bkgd(' ' as chtype | COLOR_PAIR(DEFAULT) as chtype);
}

pub fn define_colors() {
  use COLORS::*;
  use CPAIR::*;

  // init colors
  init_color(BROWN_BLACK, 0x32 * 4, 0x30 * 4, 0x2F * 4);
  init_color(WHITE, 0xEB * 4, 0xDB * 4, 0xB2 * 4);

  // init pairs
  init_pair(DEFAULT, WHITE, BROWN_BLACK);
}

pub fn mvwaddstr_fgcolor(win: WINDOW, y: i32, x: i32, s: &str, cpair: i16) {
  wattron(win, COLOR_PAIR(cpair));
  mvwaddstr(win, y, x, s);
  wattron(win, COLOR_PAIR(cpair));
}
