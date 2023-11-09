use std::{thread::sleep, time::Duration};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod constant;
mod game;
mod util;

fn main() {
    // enable_raw_mode().unwrap();

    // loop {
    //     // 1. clear screen
    //     util::clear_screen();

    //     // 2. init our plane
    //     // util::keyboard_hit();

    //     sleep(Duration::from_millis(1000));
    // }

    // disable_raw_mode().unwrap();

    let map = game::create_map();
    game::print_map(map);
}
