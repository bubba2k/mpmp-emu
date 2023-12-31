use ratatui::prelude::{Alignment, Buffer, Rect};

use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap};

pub struct TerminalWidget<'a> {
    buffer: &'a String,
}

impl<'a> TerminalWidget<'a> {
    pub fn new(str_ref: &'a String) -> Self {
        TerminalWidget { buffer: str_ref }
    }
}

impl<'a> Widget for TerminalWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We want to explicitely display newlines as '\n' in the UI
        // The way this is implemented right now is most likely very slow

        let paragraph = Paragraph::new((*self.buffer).clone())
            .wrap(Wrap { trim: false })
            .block(
                Block::new()
                    .padding(Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    })
                    .title(" Output Buffer ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            );

        paragraph.render(area, buf);
    }
}
