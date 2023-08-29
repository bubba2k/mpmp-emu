use std::num::ParseIntError;

use ratatui::prelude::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, StatefulWidget, Table, Widget};

pub struct PromptState {
    prompt_text: String,
    input_buffer: String,
    active: bool,
}

pub struct PromptWidget {}

impl PromptState {
    pub fn accept_char(&mut self, c: char) {
        self.input_buffer.push(c)
    }

    pub fn new_prompt(&mut self, prompt_text: String) {
        self.active = true;
        self.input_buffer.clear();
        self.prompt_text = prompt_text;
    }

    pub fn yield_int(&mut self) -> Result<u32, ParseIntError> {
        self.active = false;
        str::parse(&self.input_buffer.trim())
    }

    pub fn yield_int_signed(&mut self) -> Result<i32, ParseIntError> {
        self.active = false;
        str::parse(&self.input_buffer.trim())
    }

    pub fn yield_string(&mut self) -> String {
        self.active = false;
        self.input_buffer.clone()
    }

    pub fn abort(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl StatefulWidget for PromptWidget {
    type State = PromptState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut PromptState,
    ) {
        let table = Table::new([
            Row::new([Cell::from(state.prompt_text.clone())]).light_blue(),
            Row::new([Cell::from(state.input_buffer.clone())]),
        ])
        .column_spacing(1)
        .widths([Constraint::Percentage(100)].as_ref())
        .block(
            Block::default()
                .title(" Prompt ")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        );

        Widget::render(table, area, buf)
    }
}

impl Default for PromptState {
    fn default() -> Self {
        PromptState {
            prompt_text: String::new(),
            input_buffer: String::new(),
            active: false,
        }
    }
}

impl PromptWidget {
    pub fn new() -> Self {
        PromptWidget {}
    }
}
