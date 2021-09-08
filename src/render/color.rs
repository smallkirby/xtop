use ncurses::*;

mod colors {
  pub static BROWN_BLACK: i16 = 16;
  pub static WHITE: i16 = 17;
  pub static LIGHT_BLUE: i16 = 18;
  pub static LIGHT_GREEN: i16 = 19;
  pub static RED: i16 = 20;
  pub static ORANGE: i16 = 21;
}

pub mod cpair {
  pub static DEFAULT: i16 = 1;
  pub static PAIR_COMM: i16 = 2;
  pub static PAIR_HEAD: i16 = 3;
  pub static PAIR_DANGER: i16 = 4;
  pub static PAIR_MED_DANGER: i16 = 5;
}

pub fn initialize_color() {
  use cpair::*;

  start_color();
  define_colors();

  bkgd(' ' as chtype | COLOR_PAIR(DEFAULT) as chtype);
}

pub fn define_colors() {
  use colors::*;
  use cpair::*;

  // init colors
  init_color_rgb_m(BROWN_BLACK, 0x32302F, 4);
  init_color_rgb_m(WHITE, 0xEBD8B2, 4);
  init_color_rgb_m(LIGHT_BLUE, 0x84A87F, 4);
  init_color_rgb_m(LIGHT_GREEN, 0x4E9A06, 4);
  init_color_rgb_m(RED, 0xCC241D, 4);
  init_color_rgb_m(ORANGE, 0xFE8019, 4);

  // init pairs
  init_pair(DEFAULT, WHITE, BROWN_BLACK);
  init_pair(PAIR_COMM, LIGHT_BLUE, BROWN_BLACK);
  init_pair(PAIR_HEAD, LIGHT_GREEN, BROWN_BLACK);
  init_pair(PAIR_DANGER, RED, BROWN_BLACK);
  init_pair(PAIR_MED_DANGER, ORANGE, BROWN_BLACK);
}

pub fn mvwaddstr_color(win: WINDOW, y: i32, x: i32, s: &str, cpair: i16) {
  wattron(win, COLOR_PAIR(cpair));
  mvwaddstr(win, y, x, s);
  wattroff(win, COLOR_PAIR(cpair));
}

fn init_color_rgb_m(color: i16, rgb: u32, m: i16) {
  let b = (rgb & 0xFF) as i16;
  let g = ((rgb >> 8) & 0xFF) as i16;
  let r = ((rgb >> 16) & 0xFF) as i16;
  init_color(color, r * m, g * m, b * m);
}
