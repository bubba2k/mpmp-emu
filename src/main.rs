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

use crate::frontend::app::App;

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let bytevec = hex_parser::bytevec_from_hexfile(&args[1]).unwrap();
    let program = Program::from(bytevec.as_slice());
    let mut cpu = CpuState::default();
    */

    let mut app = App::new();
    app.init();
    app.run();
    app.quit();
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    cpu: &mut CpuState,
    program: &Program,
) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        // Drawing
        terminal.draw(|frame| {
            let toplevel_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let tty_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(toplevel_chunks[1]);

            let cpustate_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(toplevel_chunks[0]);

            let stdout_block = Block::default()
                .title(" Output ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1));
            let keyboard_buf_block = Block::default()
                .title(" Keyboard Buffer ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let register_block = Block::default()
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let ram_block = Block::default()
                .title(" RAM ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1));
            let pmem_block = Block::default()
                .title(" Program Memory ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .padding(Padding::new(1, 1, 1, 1));

            let stdout_pgraph = Paragraph::new(cpu.ostream.string.clone())
                .block(stdout_block)
                .wrap(Wrap { trim: false });
            let pmem_pgraph = Paragraph::new("Program memory")
                .block(pmem_block)
                .wrap(Wrap { trim: true });
            let register_table = Table::new(vec![
                Row::new(vec![
                    Cell::from("%reg0"),
                    Cell::from(format!("{:#4X}", cpu.registers[0])),
                ]),
                Row::new(vec![
                    Cell::from("%reg1"),
                    Cell::from(format!("{:#4X}", cpu.registers[1])),
                ]),
                Row::new(vec![
                    Cell::from("%reg2"),
                    Cell::from(format!("{:#4X}", cpu.registers[2])),
                ]),
                Row::new(vec![
                    Cell::from("%reg3"),
                    Cell::from(format!("{:#4X}", cpu.registers[3])),
                ]),
                Row::new(vec![
                    Cell::from("%reg4"),
                    Cell::from(format!("{:#4X}", cpu.registers[4])),
                ]),
                Row::new(vec![
                    Cell::from("%reg5"),
                    Cell::from(format!("{:#4X}", cpu.registers[5])),
                ]),
                Row::new(vec![
                    Cell::from("PC"),
                    Cell::from(format!("{:#4X}", cpu.pcounter)),
                ]),
            ])
            .block(register_block)
            .column_spacing(1)
            .widths([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());

            frame.render_widget(keyboard_buf_block, tty_chunks[1]);
            frame.render_widget(stdout_pgraph, tty_chunks[0]);
            frame.render_widget(register_table, cpustate_chunks[0]);
            frame.render_widget(ram_block, cpustate_chunks[1]);
            frame.render_widget(pmem_pgraph, toplevel_chunks[2]);
        })?;

        // CPU update
        if !cpu.received_halt {
            cpu.execute_next_prog_op(program);
        }

        // Event handling
        if event::poll(Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}
