use chrono::offset::Local;
use chrono::DateTime;
use ratatui::prelude::{Buffer, Rect};

use ratatui::widgets::{Paragraph, Widget, Wrap};

use crate::frontend::log::{Log, Message};

pub struct LogWidget<'a> {
    messages: &'a Vec<Message>,
}

impl<'a> LogWidget<'a> {
    pub fn new(log: &'a Log) -> Self {
        LogWidget {
            messages: &log.messages,
        }
    }
}

impl<'a> Widget for LogWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let log_string = self
            .messages
            .iter()
            .rev()
            .fold(String::new(), |acc_string, msg| {
                acc_string
                    + &format!(
                        "[{}] {}\n",
                        DateTime::<Local>::from(msg.timestamp).format("%T"),
                        msg.message_string
                    )
            });
        let paragraph = Paragraph::new(log_string).wrap(Wrap { trim: false });

        paragraph.render(area, buf)
    }
}
