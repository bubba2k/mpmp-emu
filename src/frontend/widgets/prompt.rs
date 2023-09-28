use ratatui::prelude::{Alignment};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap};

pub struct PromptWidget<'a> {
    input_buffer: &'a str,
    prompt_text: &'a str,
}

impl<'a> Widget for PromptWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let text = vec![
            Line::styled(self.prompt_text, Style::default().light_blue()),
            Line::styled(self.input_buffer, Style::default()),
        ];

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .title(" Prompt ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        );

        Widget::render(paragraph, area, buf)
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
