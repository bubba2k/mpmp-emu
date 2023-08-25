use crate::hex_parser;
use ratatui::prelude::{
    Alignment, Color, Constraint, CrosstermBackend, Direction, Layout, Rect, Style,
};
use ratatui::widgets::{Cell, Row, Table};
use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use ratatui::Terminal;

use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::frontend::widgets::*;
use crate::program::Program;
use crate::runtime::CpuState;

pub struct App {
    cpu: CpuState,
    program: Program,

    terminal: Terminal<CrosstermBackend<Stdout>>,

    // Layout
    toplevel_layout: Layout,
    tty_layout: Layout,
    cpustate_layout: Layout,
    rightpanel_layout: Layout,
    // Component states
    ram_widget_state: RamTableState,
    pmem_widget_state: PmemTableState,
    registers_widget_state: RegistersDisplayState,

    log_messages: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        // Setup layout
        let toplevel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            );
        let tty_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref());

        let cpustate_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref());

        let rightpanel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref());

        let mut stdout = io::stdout();
        enable_raw_mode();
        execute!(stdout, EnterAlternateScreen);
        let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

        App {
            cpu: CpuState::default(),
            program: Program::default(),
            toplevel_layout,
            tty_layout,
            cpustate_layout,
            ram_widget_state: RamTableState::default(),
            pmem_widget_state: PmemTableState::default(),
            registers_widget_state: RegistersDisplayState::default(),
            terminal,
            log_messages: Vec::new(),
            rightpanel_layout,
        }
    }

    fn draw(&mut self) {
        self.terminal
            .draw(|frame| {
                // Resolve layout
                let toplevel_chunks = self.toplevel_layout.split(frame.size());
                let tty_chunks = self.tty_layout.split(toplevel_chunks[1]);
                let cpustate_chunks = self.cpustate_layout.split(toplevel_chunks[0]);
                let rightpanel_chunks = self.rightpanel_layout.split(toplevel_chunks[2]);

                let ram_table = RamTableWidget::new(&self.cpu);
                let pmem_table = PmemTableWidget::new(&self.cpu, &self.program);
                let register_display = RegistersDisplayWidget::new(&self.cpu);
                let keybuffer_widget = KeybufferWidget::new(&self.cpu.istream.string);
                let terminal_widget = TerminalWidget::new(&self.cpu.ostream.string);

                let message_table = Table::new(
                    self.log_messages
                        .iter()
                        .map(|msg| Row::new([Cell::from(msg.clone())]))
                        .collect::<Vec<Row>>(),
                )
                .column_spacing(1)
                .widths([Constraint::Percentage(100)].as_ref());

                frame.render_stateful_widget(
                    ram_table,
                    cpustate_chunks[1],
                    &mut self.ram_widget_state,
                );
                frame.render_stateful_widget(
                    pmem_table,
                    rightpanel_chunks[0],
                    &mut self.pmem_widget_state,
                );
                frame.render_stateful_widget(
                    register_display,
                    cpustate_chunks[0],
                    &mut self.registers_widget_state,
                );
                frame.render_widget(message_table, rightpanel_chunks[1]);
                frame.render_widget(keybuffer_widget, tty_chunks[1]);
                frame.render_widget(terminal_widget, tty_chunks[0]);
            })
            .unwrap();
    }

    pub fn try_load_program(&mut self, path: String) -> bool {
        let res = hex_parser::bytevec_from_hexfile(path.clone());
        match res {
            Err(_) => {
                self.log_messages
                    .push(format!("Failed to load file '{}'", path));
                false
            }
            Ok(bytes) => {
                self.program = Program::from(bytes.as_slice());
                self.log_messages
                    .push(format!("Successfully loaded '{}'", path));
                true
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.cpu.received_halt {
                self.cpu.execute_next_prog_op(&self.program);
                if self.cpu.received_halt {
                    self.log_messages.push(String::from("CPU recieved halt"));
                }
            }
            self.draw();
            // Event handling
            if event::poll(Duration::from_millis(50)).expect("Should work") {
                if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                    if KeyCode::Char('q') == key.code {
                        break;
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.cpu = CpuState::default();
        self.program = Program::default();

        self.registers_widget_state = RegistersDisplayState::default();
        self.ram_widget_state = RamTableState::default();
        self.pmem_widget_state = PmemTableState::default();
    }

    pub fn quit(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
        Ok(self.terminal.show_cursor()?)
    }
}
