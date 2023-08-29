use ratatui::prelude::{Alignment, Constraint};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table, Widget};

pub struct PromptWidget<'a> {
    input_buffer: &'a str,
    prompt_text: &'a str,
}

impl<'a> Widget for PromptWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let table = Table::new([
            Row::new([Cell::from(self.prompt_text)]).light_blue(),
            Row::new([Cell::from(self.input_buffer)]),
        ])
        .column_spacing(1)
        .widths([Constraint::Percentage(100)].as_ref())
        .block(
            Block::default()
                .title(" Prompt ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        );

        Widget::render(table, area, buf)
    }
}

impl<'a> PromptWidget<'a> {
    pub fn new(prompt_text: &'a str, input_buffer: &'a str) -> Self {
        PromptWidget {
            input_buffer,
            prompt_text,
        }
    }
}
