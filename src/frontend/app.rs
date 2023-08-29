use ratatui::prelude::{Constraint, CrosstermBackend, Direction, Layout};

use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use ratatui::Terminal;

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::backend;
use crate::backend::program::Program;
use crate::backend::runtime::CpuState;
use crate::frontend::widgets::*;
use crate::util::Timer;

use super::log::*;

enum UiMode {
    Normal,
    InspectRam,
    InspectProgram,
}

pub struct App {
    cpu: CpuState,
    program: Program,
    execution_timer: Timer,

    terminal: Terminal<CrosstermBackend<Stdout>>,

    // Keep track of UI mode
    ui_mode: UiMode,

    // Layout
    toplevel_layout: Layout,
    tty_layout: Layout,
    cpustate_layout: Layout,
    rightpanel_layout: Layout,
    // Component states
    ram_widget_state: RamTableState,
    pmem_widget_state: PmemTableState,
    registers_widget_state: RegistersDisplayState,
    keybuffer_widget_state: KeybufferWidgetState,

    prompt_state: PromptState,

    message_log: Log,

    should_quit: bool,
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
            execution_timer: Timer::new(Duration::from_millis(250)),

            ui_mode: UiMode::Normal,
            should_quit: false,

            toplevel_layout,
            tty_layout,
            cpustate_layout,
            ram_widget_state: RamTableState::default(),
            pmem_widget_state: PmemTableState::default(),
            registers_widget_state: RegistersDisplayState::default(),
            prompt_state: PromptState::default(),
            keybuffer_widget_state: KeybufferWidgetState { focused: true },

            terminal,
            message_log: Log::default(),
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
                let log_widget = LogWidget::new(&self.message_log);

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
                frame.render_stateful_widget(
                    keybuffer_widget,
                    tty_chunks[1],
                    &mut self.keybuffer_widget_state,
                );
                frame.render_widget(terminal_widget, tty_chunks[0]);
                frame.render_widget(log_widget, rightpanel_chunks[1]);

                if self.prompt_state.is_active() {
                    let prompt_chunk = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(100)])
                        .margin(10)
                        .split(frame.size())[0];
                    frame.render_stateful_widget(
                        PromptWidget::new(),
                        prompt_chunk,
                        &mut self.prompt_state,
                    )
                }
            })
            .unwrap();
    }

    pub fn try_load_program(&mut self, path: String) -> bool {
        let res = backend::hex_parser::bytevec_from_hexfile(path.clone());
        match res {
            Err(_) => {
                self.message_log.log(Message::new(
                    MessageType::Error,
                    format!("Failed to load file {}.", path),
                ));
                false
            }
            Ok(bytes) => {
                self.program = Program::from(bytes.as_slice());
                self.message_log.log(Message::new(
                    MessageType::Info,
                    format!("Successfully loaded file {}.", path),
                ));
                true
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw();

            if self.execution_timer.has_elapsed() {
                self.execution_timer.reset();
                if !self.cpu.received_halt && self.program.operations.len() > 0 {
                    self.cpu.execute_next_prog_op(&self.program);
                    if self.cpu.received_halt {
                        self.message_log.log(Message::new(
                            MessageType::Info,
                            String::from("CPU received halt"),
                        ));
                    }
                }
            }

            self.handle_input();

            if self.should_quit {
                break;
            }
        }
    }

    fn handle_input(&mut self) {
        // Event handling
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                match self.ui_mode {
                    UiMode::Normal => self.handle_event_normal(key),
                    UiMode::InspectRam => self.handle_event_ram(key),
                    UiMode::InspectProgram => self.handle_event_program(key),
                }
            }
        }
    }

    // The following functions are input handlers for each context
    fn handle_event_normal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Char(c) => self.cpu.istream.append_char(c),
            KeyCode::Enter => self.cpu.istream.append_char('\n'),
            KeyCode::Backspace => {
                self.cpu.istream.string.pop();
                ()
            }
            KeyCode::Tab => {
                self.ui_mode = UiMode::InspectProgram;
                self.keybuffer_widget_state.focused = false;
                self.pmem_widget_state.is_focussed = true;
            }
            _ => {}
        }
    }
    fn handle_event_ram(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.ram_widget_state.scroll(-1),
            KeyCode::Down | KeyCode::Char('j') => self.ram_widget_state.scroll(1),
            KeyCode::PageUp | KeyCode::Char('K') => self.ram_widget_state.scroll(-16),
            KeyCode::PageDown | KeyCode::Char('J') => self.ram_widget_state.scroll(16),
            KeyCode::Char('G') => self.prompt_state.new_prompt(String::from("Goto line:")),
            KeyCode::Tab => {
                self.ui_mode = UiMode::Normal;
                self.ram_widget_state.is_focussed = false;
                self.keybuffer_widget_state.focused = true;
            }
            _ => {}
        }
    }
    fn handle_event_program(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.pmem_widget_state.scroll(-1),
            KeyCode::Down | KeyCode::Char('j') => self.pmem_widget_state.scroll(1),
            KeyCode::PageUp | KeyCode::Char('K') => self.pmem_widget_state.scroll(-16),
            KeyCode::PageDown | KeyCode::Char('J') => self.pmem_widget_state.scroll(16),
            KeyCode::Char('f') => {
                self.pmem_widget_state.focus_executing = !self.pmem_widget_state.focus_executing
            }
            KeyCode::Char('b') => {
                // Set a new breakpoint
                let selected_is_breakpoint =
                    self.program.breakpoints[self.pmem_widget_state.selected as usize];

                self.program.breakpoints[self.pmem_widget_state.selected as usize] =
                    !selected_is_breakpoint;
            }
            KeyCode::Tab => {
                self.ui_mode = UiMode::InspectRam;
                self.pmem_widget_state.is_focussed = false;
                self.ram_widget_state.is_focussed = true;
            }
            _ => {}
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
