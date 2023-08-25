use ratatui::prelude::{Alignment, Buffer, Color, Constraint, Rect};

use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap};

use crate::runtime::{CpuState, Flags};

pub struct KeybufferWidget<'a> {
    keys: &'a String,
}

impl<'a> KeybufferWidget<'a> {
    pub fn new(str_ref: &'a String) -> Self {
        KeybufferWidget { keys: str_ref }
    }
}

impl<'a> Widget for KeybufferWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We want to explicitely display newlines as '\n' in the UI
        // The way this is implemented right now is most likely very slow
        let formatted_string = self
            .keys
            .chars()
            .map(|char| match char {
                '\n' => String::from("\\n"),
                '\t' => String::from("\\t"),
                _ => {
                    let mut str = String::new();
                    str.insert(0, char);
                    str
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let paragraph = Paragraph::new(formatted_string)
            .wrap(Wrap { trim: false })
            .block(
                Block::new()
                    .title(" Input Buffer ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            );

        paragraph.render(area, buf);
    }
}
