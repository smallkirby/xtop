# 🚧 UNDER CONSTRUCTION 🚧

# xtop 

extremely-simplified top

![log4](./images/log4.png)


## Depends

`xtop` depends on below relatively primitive crates:
- [`ncurses`](https://github.com/jeaye/ncurses-rs): TUI
- [`sysconf`](https://github.com/zerocostgoods/sysconf.rs): only to get a jiffy.
- [`signal-hook`](https://github.com/vorner/signal-hook): to handle `SIGWINCH`.
- [`serde`](https://github.com/serde-rs/serde): to read layout config from file.

## Env

Intended only on Linux(Ubuntu).

## Progress

| Status | Functionality |
| ------------- | ------------- |
| 🌤 | CPU Graph |
| 🌤 | CPU Meter |
| 🌤 | XInput list |
| 🌤 | process list |
| ☀ | task list |
| ☁ | command window |
| ⛈ | search process |
| ⛈ | scrollable process list |
| ⛈ | modest colorize |
| ⛈ | configurable layout |
| ⛈ | network usage |
| ⛈ | memory usage |
| ⛈ | kernel config list |


### legend

- ☀️: completed
- 🌤: almost done, still needs more impls 
- ☁️: work in progress
- ⛈: totally untouched
