use ncurses::*;

pub fn test_just_window(s: &str) {
  // XXX
  // test of ncurses
  initscr();
  raw();
  keypad(stdscr(), true);
  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  refresh();

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
