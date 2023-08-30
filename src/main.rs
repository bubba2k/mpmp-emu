// Remove this later
#![allow(dead_code, unused_variables)]

extern crate chrono;
extern crate num;
#[macro_use]
extern crate num_derive;

extern crate crossterm;
extern crate ratatui;

mod backend;
mod frontend;
mod util;

use frontend::App;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut app = App::new();

    if args.len() == 2 {
        app.try_load_program(args[1].clone());
    }

    app.run();
    app.quit().expect("Quitting should work");
}
