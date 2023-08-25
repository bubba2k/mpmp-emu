use std::env;
use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use ratatui::prelude::{
    Alignment, Color, Constraint, CrosstermBackend, Direction, Layout, Rect, Style,
};
use ratatui::widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, Wrap};
use ratatui::Terminal;

use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use self::program::Program;
use self::runtime::CpuState;

mod decoder;
mod frontend;
mod hex_parser;
mod ir;
mod program;
mod runtime;

extern crate num;
#[macro_use]
extern crate num_derive;

extern crate crossterm;
extern crate ratatui;

use crate::frontend::App;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = "helpers/hex/tty_test.hex";

    let mut app = App::new();
    if args.len() < 2 {
        app.try_load_program(String::from(default_path));
    } else {
        app.try_load_program(args[1].clone());
    }
    app.run();
    app.quit().expect("Quitting should work");
}
