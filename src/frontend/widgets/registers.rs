use ratatui::prelude::{Alignment, Color, Constraint};

use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget};

use crate::backend::runtime::{CpuState, Flags};

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

        // Build the rows
        rows.push(Row::new(vec![
            Cell::from("%reg0").blue(),
            Cell::from(state.get_number_repr(self.registers_ref[0])),
            Cell::from("pc").magenta(),
            Cell::from(state.get_number_repr(*self.pcounter_ref)),
        ]));

        rows.push(Row::new(vec![
            Cell::from("%reg1").blue(),
            Cell::from(state.get_number_repr(self.registers_ref[1])),
            Cell::from("zero").green(),
            Cell::from(format!("{}", self.flags_ref.zero)),
        ]));

        rows.push(Row::new(vec![
            Cell::from("%reg2").blue(),
            Cell::from(state.get_number_repr(self.registers_ref[2])),
            Cell::from("carry").green(),
            Cell::from(format!("{}", self.flags_ref.carry)),
        ]));

        let mut remaining_rows = (3..6)
            .into_iter()
            .map(|i| {
                Row::new(vec![
                    Cell::from(format!("%reg{}", i)).blue(),
                    Cell::from(state.get_number_repr(self.registers_ref[i])),
                ])
            })
            .collect();

        rows.append(&mut remaining_rows);

        let table = Table::new(rows)
            .column_spacing(1)
            .block(
                Block::default()
                    .title(" Registers / Flags")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .widths(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            );

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
