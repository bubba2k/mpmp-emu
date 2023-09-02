use ratatui::prelude::Alignment;
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap};

pub struct HelpScreenWidget {}

impl Default for HelpScreenWidget {
    fn default() -> Self {
        HelpScreenWidget {}
    }
}

impl Widget for HelpScreenWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let text = vec![
            Line::styled("General", Style::default().bold()),
            Line::from("Esc: Exit / Go back"),
            Line::from("F1: Display this screen"),
            Line::from("F2: Load filepath"),
            Line::from("F3: Reset CPU"),
            Line::from("F4: Set per-instruction execution delay"),
            Line::from("F5: Start/stop CPU"),
            Line::from("F6: Perform a single step"),
            Line::from("Tab: Switch input context [Terminal Input Buffer / Program Memory / RAM]. The active context is highlighted."),
            Line::from(""),
            Line::styled("In Input Buffer context", Style::default().bold()),
            Line::from("All keystrokes are captured by the terminal."),
            Line::from(""),
            Line::styled("In RAM context", Style::default().bold()),
            Line::from("Down/Up or j/k: Navigate RAM table"),
            Line::from("PGDOWN/PGUP or J/K: Navigate RAM table (16 steps)"),
            Line::from("g: Go to specified memory address"),
            Line::from(""),
            Line::styled("In Program Memory context", Style::default().bold()),
            Line::from("f: Follow currently executing instruction (toggle)"),
            Line::from("Down/Up or j/k: Navigate program memory (when not following currently executing instruction)"),
            Line::from("b: Toggle breakpoint at selected instruction"),
        ];

        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .title(" Help ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .padding(Padding {
                    left: 3,
                    right: 3,
                    top: 1,
                    bottom: 1,
                }),
        );

        paragraph.render(area, buf)
    }
}
