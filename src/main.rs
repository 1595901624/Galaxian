use std::{thread::sleep, time::Duration};

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


mod element;
mod util;
mod constant;

fn main() {
    enable_raw_mode().unwrap();

    loop {
        // 1. clear screen
        util::clear_screen();

        // 2. init our plane
        // util::keyboard_hit();

        sleep(Duration::from_millis(1000));
    }

    disable_raw_mode().unwrap();
}
