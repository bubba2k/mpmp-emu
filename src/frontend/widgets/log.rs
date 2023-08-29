use chrono::offset::Utc;
use chrono::DateTime;
use ratatui::prelude::{Buffer, Color, Constraint, Rect};

use ratatui::style::Stylize;
use ratatui::widgets::{Cell, Row, Table, Widget};

use crate::frontend::log::{Log, Message, MessageType};

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
        let rows = self
            .messages
            .iter()
            .rev()
            .map(|msg| {
                let date_time: DateTime<Utc> = msg.timestamp.into();
                Row::new([
                    // First, the timestamp
                    Cell::from(date_time.format("%T").to_string()),
                    // Split the message into two
                    Cell::from(msg.message_string.clone()).fg(match msg.message_type {
                        MessageType::Info => Color::White,
                        MessageType::Error => Color::Red,
                        MessageType::Warning => Color::Yellow,
                    }),
                ])
            })
            .collect::<Vec<Row>>();

        let table = Table::new(rows)
            .column_spacing(1)
            .widths([Constraint::Min(10), Constraint::Percentage(80)].as_ref());

        Widget::render(table, area, buf);
    }
}
