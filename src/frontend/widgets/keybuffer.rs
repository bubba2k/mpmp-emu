use ratatui::prelude::{Alignment, Buffer, Rect};

use ratatui::widgets::{
    Block, BorderType, Borders, Padding, Paragraph, StatefulWidget, Widget, Wrap,
};

pub struct KeybufferWidget<'a> {
    keys: &'a String,
}

pub struct KeybufferWidgetState {
    pub focused: bool,
}

impl<'a> KeybufferWidget<'a> {
    pub fn new(str_ref: &'a String) -> Self {
        KeybufferWidget { keys: str_ref }
    }
}

impl<'a> StatefulWidget for KeybufferWidget<'a> {
    type State = KeybufferWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut KeybufferWidgetState) {
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
                    .padding(Padding {
                        left: 1,
                        right: 1,
                        bottom: 0,
                        top: 0,
                    })
                    .borders(Borders::ALL)
                    .border_type(match state.focused {
                        true => BorderType::Thick,
                        false => BorderType::Plain,
                    }),
            );

        paragraph.render(area, buf);
    }
}
