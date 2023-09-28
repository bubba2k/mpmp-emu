use ratatui::prelude::{Constraint, CrosstermBackend, Direction, Layout};

use std::error::Error;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::str::FromStr;
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
    InspectTerminal,
    InspectRam,
    InspectProgram,
}

impl UiMode {
    pub fn next(&mut self) {
        match *self {
            UiMode::InspectRam => *self = UiMode::InspectTerminal,
            UiMode::InspectTerminal => *self = UiMode::InspectProgram,
            UiMode::InspectProgram => *self = UiMode::InspectRam,
        }
    }

    pub fn previous(&mut self) {
        match *self {
            UiMode::InspectTerminal => *self = UiMode::InspectRam,
            UiMode::InspectRam => *self = UiMode::InspectProgram,
            UiMode::InspectProgram => *self = UiMode::InspectTerminal,
        }
    }
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

    message_log: Log,

    should_quit: bool,
    cpu_running: bool,
    cpu_step_requested: bool,
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
            .constraints([Constraint::Max(8), Constraint::Percentage(60)].as_ref());

        let rightpanel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref());

        let mut stdout = io::stdout();
        let _ = enable_raw_mode();
        let _ = execute!(stdout, EnterAlternateScreen);
        let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

        // Log comes with help message
        let mut log = Log::default();
        log.log(Message::new(
            MessageType::Info,
            String::from("Press F1 for help!"),
        ));

        App {
            cpu: CpuState::default(),
            program: Program::default(),
            execution_timer: Timer::new(Duration::from_millis(250)),
            cpu_running: false,
            cpu_step_requested: false,

            ui_mode: UiMode::InspectTerminal,
            should_quit: false,

            toplevel_layout,
            tty_layout,
            cpustate_layout,
            ram_widget_state: RamTableState::default(),
            pmem_widget_state: PmemTableState::default(),
            registers_widget_state: RegistersDisplayState::default(),
            keybuffer_widget_state: KeybufferWidgetState { focused: true },

            terminal,
            message_log: log,
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

                // Set the right widget to focussed
                match self.ui_mode {
                    UiMode::InspectTerminal => {
                        self.keybuffer_widget_state.focused = true;
                        self.ram_widget_state.is_focussed = false;
                        self.pmem_widget_state.is_focussed = false;
                    }
                    UiMode::InspectRam => {
                        self.keybuffer_widget_state.focused = false;
                        self.ram_widget_state.is_focussed = true;
                        self.pmem_widget_state.is_focussed = false;
                    }
                    UiMode::InspectProgram => {
                        self.keybuffer_widget_state.focused = false;
                        self.ram_widget_state.is_focussed = false;
                        self.pmem_widget_state.is_focussed = true;
                    }
                }

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
            })
            .unwrap();
    }

    fn help_screen(&mut self) {
        loop {
            let help_screen = HelpScreenWidget::default();

            self.terminal
                .draw(|frame| frame.render_widget(help_screen, frame.size()))
                .unwrap();

            if event::poll(Duration::from_millis(500)).unwrap() {
                if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::F(1) => break,
                        _ => {}
                    }
                }
            }
        }
    }

    fn prompt<T: FromStr>(&mut self, prompt_text: &str) -> Option<T> {
        let mut input_buffer = String::new();

        loop {
            let prompt_widget = PromptWidget::new(prompt_text, &input_buffer);
            let vertical_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Max(4),
                    Constraint::Percentage(33),
                ])
                .split(self.terminal.get_frame().size())[1];
            let area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Max(64),
                    Constraint::Percentage(33),
                ])
                .split(vertical_area)[1];
            self.terminal
                .draw(|frame| frame.render_widget(prompt_widget, area))
                .unwrap();

            if event::poll(Duration::from_millis(500)).unwrap() {
                if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char(c) => input_buffer.push(c),
                        KeyCode::Backspace => {
                            let _ = input_buffer.pop();
                        }
                        KeyCode::Esc => {
                            return None;
                        }
                        KeyCode::Enter => match str::parse::<T>(&input_buffer) {
                            Ok(val) => return Some(val),
                            Err(_) => {
                                input_buffer.clear();
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn try_load_program(&mut self, path: String) -> bool {
        let res = backend::hex_parser::bytevec_from_hexfile(path.clone());
        match res {
            Err(_) => {
                self.message_log.log(Message::new(
                    MessageType::Error,
                    format!("Failed to load '{}'", path),
                ));
                false
            }
            Ok(bytes) => {
                self.reset_cpu();
                self.program = Program::from(bytes.as_slice());
                self.message_log.log(Message::new(
                    MessageType::Info,
                    format!("Loaded '{}'", path),
                ));
                true
            }
        }
    }

    fn reset_cpu(&mut self) {
        self.cpu_step_requested = false;
        self.cpu_running = false;
        self.cpu = CpuState::default();
    }

    fn update_cpu(&mut self) {
        // If program empty or cpu halted, skip
        if self.program.operations.len() == 0 || self.cpu.received_halt {
            return;
        }

        // Free running
        if self.cpu_running && self.execution_timer.has_elapsed() {
            self.cpu.execute_next_prog_op(&self.program);
            self.execution_timer.reset();
            if self.cpu.received_halt {
                self.message_log.log(Message::new(
                    MessageType::Info,
                    String::from("CPU received halt."),
                ))
            }
        }

        // Single step
        if self.cpu_step_requested {
            self.cpu.execute_next_prog_op(&self.program);
            if self.cpu.received_halt {
                self.message_log.log(Message::new(
                    MessageType::Info,
                    String::from("CPU received halt."),
                ))
            }
            self.cpu_step_requested = false;
        }

        // Stop if a breakpoint got reached
        if (self.cpu_running || self.cpu_step_requested)
            && self.program.breakpoints[self
                .cpu
                .pcounter
                .clamp(0, (self.program.operations.len() - 1) as u16)
                as usize]
        {
            self.cpu_running = false;
            self.message_log.log(Message::new(
                MessageType::Info,
                format!("Reached breakpoint at {:#X}", self.cpu.pcounter),
            ))
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw();

            self.update_cpu();

            self.handle_input();

            if self.should_quit {
                break;
            }
        }
    }

    // Returns true if keyevent was used/consumend
    fn handle_input_general(&mut self, key: &KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                let opt = self.prompt::<bool>("Really quit? (true/false)");
                match opt {
                    Some(b) => self.should_quit = b,
                    None => {}
                }
                true
            }
            KeyCode::Tab => {
                self.ui_mode.next();
                true
            }
            KeyCode::BackTab => {
                self.ui_mode.previous();
                true
            }
            KeyCode::F(1) => {
                self.help_screen();
                true
            }
            KeyCode::F(2) => {
                let path_opt = self.prompt::<PathBuf>("Enter file path:");

                match path_opt {
                    None => {}
                    Some(path) => match path.to_str() {
                        None => {}
                        Some(path_str) => {
                            self.try_load_program(String::from(path_str));
                        }
                    },
                }
                true
            }
            KeyCode::F(3) => {
                self.reset_cpu();
                self.message_log
                    .log(Message::new(MessageType::Info, String::from("Reset CPU")));
                true
            }
            KeyCode::F(4) => {
                let opt = self.prompt::<u32>("Set execution delay (ms)");
                match opt {
                    Some(n) => self
                        .execution_timer
                        .set_duration(Duration::from_millis(n as u64)),
                    None => {}
                }
                true
            }
            KeyCode::F(5) => {
                self.cpu_running = !self.cpu_running;
                true
            }
            KeyCode::F(6) => {
                self.cpu_step_requested = true;
                true
            }
            _ => false,
        }
    }

    fn handle_input(&mut self) {
        // Event handling
        if event::poll(Duration::from_millis(20)).unwrap() {
            if let crossterm::event::Event::Key(key) = event::read().unwrap() {
                // General input (always applicable), these are handled by the below
                // call and we return right away if the input was consumed
                if self.handle_input_general(&key) {
                    return;
                }

                // Specific input
                match self.ui_mode {
                    UiMode::InspectTerminal => self.handle_event_terminal(key),
                    UiMode::InspectRam => self.handle_event_ram(key),
                    UiMode::InspectProgram => self.handle_event_program(key),
                }
            }
        }
    }

    // The following functions are input handlers for each context
    fn handle_event_terminal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.cpu.istream.append_char(c),
            KeyCode::Enter => self.cpu.istream.append_char('\n'),
            KeyCode::Backspace => {
                self.cpu.istream.string.pop();
                ()
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
            KeyCode::Char('g') => match self.prompt::<u32>("Go to RAM address: ") {
                None => {}
                Some(n) => self.ram_widget_state.goto_address(n),
            },
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
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.cpu = CpuState::default();
        self.program = Program::default();

        self.registers_widget_state = RegistersDisplayState::default();
        self.ram_widget_state = RamTableState::default();
        self.pmem_widget_state = PmemTableState::default();
        self.keybuffer_widget_state = KeybufferWidgetState { focused: true };
        self.ui_mode = UiMode::InspectTerminal;
    }

    pub fn quit(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
        Ok(self.terminal.show_cursor()?)
    }
}
