use ratatui::prelude::{
    Alignment, Color, Constraint, CrosstermBackend, Direction, Layout, Rect, Style,
};
use std::env;
use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use ratatui::Terminal;

use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::frontend::pmem::{PmemTableState, PmemTableWidget};
use crate::frontend::ram::{RamTableState, RamTableWidget};
use crate::frontend::registers::{RegistersDisplayState, RegistersDisplayWidget};
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
    // Component states
    ram_widget_state: RamTableState,
    pmem_widget_state: PmemTableState,
    registers_widget_state: RegistersDisplayState,
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
        }
    }

    fn draw(&mut self) {
        self.terminal
            .draw(|frame| {
                // Resolve layout
                let toplevel_chunks = self.toplevel_layout.split(frame.size());
                let tty_chunks = self.tty_layout.split(toplevel_chunks[1]);
                let cpustate_chunks = self.cpustate_layout.split(toplevel_chunks[0]);

                let ram_table = RamTableWidget::new(&self.cpu);
                let pmem_table = PmemTableWidget::new(&self.cpu, &self.program);
                let register_display = RegistersDisplayWidget::new(&self.cpu);

                frame.render_stateful_widget(
                    ram_table,
                    cpustate_chunks[1],
                    &mut self.ram_widget_state,
                );
                frame.render_stateful_widget(
                    pmem_table,
                    toplevel_chunks[2],
                    &mut self.pmem_widget_state,
                );
                frame.render_stateful_widget(
                    register_display,
                    cpustate_chunks[0],
                    &mut self.registers_widget_state,
                );
            })
            .unwrap();
    }

    pub fn run(&mut self) {
        loop {
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

    pub fn init(&mut self) -> bool {
        // Setup layout
        self.toplevel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            );
        self.tty_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref());

        self.cpustate_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref());

        true
    }

    pub fn quit(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
        Ok(self.terminal.show_cursor()?)
    }
}
