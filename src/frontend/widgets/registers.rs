use ratatui::prelude::{Alignment, Buffer, Color, Constraint, Rect};

use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget};

use crate::runtime::{CpuState, Flags};

pub struct RegistersDisplayWidget<'a> {
    pcounter_ref: &'a u16,
    registers_ref: &'a [u16; 6],
    flags_ref: &'a Flags,
}

pub struct RegistersDisplayState {
    pub use_hex: bool,
}

impl Default for RegistersDisplayState {
    fn default() -> Self {
        RegistersDisplayState { use_hex: true }
    }
}

impl RegistersDisplayState {
    fn get_number_repr(&self, num: u16) -> String {
        if self.use_hex {
            format!("{:#X}", num)
        } else {
            format!("{}", num)
        }
    }
}

impl<'a> StatefulWidget for RegistersDisplayWidget<'a> {
    type State = RegistersDisplayState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut rows = Vec::new();

        // program counter
        rows.push(
            Row::new(vec![
                String::from("pc"),
                state.get_number_repr(*self.pcounter_ref),
            ])
            .fg(Color::LightMagenta),
        );

        // registers
        for i in 0..6 {
            rows.push(Row::new(vec![
                format!("reg{}", i),
                state.get_number_repr(self.registers_ref[i as usize]),
            ]));
        }

        // Flags... TODO: Make this prettier
        rows.push(Row::new(vec![
            String::from("carry"),
            format!("{}", self.flags_ref.carry),
        ]));
        rows.push(Row::new(vec![
            String::from("zero"),
            format!("{}", self.flags_ref.zero),
        ]));

        let table = Table::new(rows)
            .column_spacing(1)
            .block(
                Block::default()
                    .title(" Registers ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .widths([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref());

        Widget::render(table, area, buf)
    }
}

impl<'a> RegistersDisplayWidget<'a> {
    pub fn new(cpu: &'a CpuState) -> Self {
        RegistersDisplayWidget {
            pcounter_ref: &cpu.pcounter,
            registers_ref: &cpu.registers,
            flags_ref: &cpu.flags,
        }
    }
}
