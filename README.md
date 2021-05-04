# Simple program for fetching system info

![Screenshot]("https://raw.githubusercontent.com/icebox246/rfetch/master/screen.png")

Written in Rust, shows basic info

- user and host name
- distro name (if compatible with systemd or freedesktop.org)
- kernel version
- current shell
- darker half of color palette

Cute penguin ascii art supplied and batteries included (most of the time).

## Compilation

1. Install Rust if you haven't already done this
2. Run `cargo build --release` and needed crates should get downloaded and after compilation program should run

## Installation

1. After compiling make `install.sh` executable and run it as a super user

## Deinstalation

1. Make `uninstall.sh` executable and run it as a super user
