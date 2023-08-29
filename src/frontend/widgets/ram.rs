use crate::backend::runtime::{CpuState, RAM_SIZE};

use ratatui::prelude::Constraint;
use ratatui::prelude::{Alignment, Buffer, Color, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Padding, Row, StatefulWidget, Table, Widget,
};

const RAM_MAX_LINES: u32 = (RAM_SIZE / 4) as u32;

pub struct RamTableWidget<'a> {
    ram_ref: &'a [u16; RAM_SIZE],
}

pub struct RamTableState {
    pub starting_row: u32,
    pub is_focussed: bool,
}

impl<'a> RamTableWidget<'a> {
    pub fn new(cpu: &'a CpuState) -> Self {
        RamTableWidget { ram_ref: &cpu.ram }
    }
}

impl<'a> RamTableWidget<'a> {
    fn construct_row(&self, memory_row_index: u32) -> Row {
        let row_mem_address = memory_row_index * 4;

        // Push row address
        let mut cells =
            vec![Cell::from(format!("{:04X}", row_mem_address)).fg(Color::LightMagenta)];

        // Push the row values
        for address in row_mem_address..(row_mem_address + 4) {
            cells.push(Cell::from(format!(
                "{:04X}",
                self.ram_ref[address as usize]
            )));
        }

        Row::new(cells)
    }
}

impl<'a> StatefulWidget for RamTableWidget<'a> {
    type State = RamTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Check how many lines we can render
        let n_rows = (area.height - 2) as u32; // -2 because of borders
        let end_row = state.starting_row + n_rows;

        // Build a list of indices... remember, we need to wrap around LINE_MAX
        let indices: Vec<u32> = (state.starting_row..end_row)
            .map(|i| i % RAM_MAX_LINES)
            .collect();
        // Construct the rows
        let rows = indices
            .iter()
            .map(|row_index| self.construct_row(*row_index));

        // Construct the table
        let table = Table::new(rows)
            .block(
                Block::default()
                    .title(" RAM ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(match state.is_focussed {
                        true => BorderType::Thick,
                        false => BorderType::Plain,
                    })
                    .padding(Padding::new(1, 1, 0, 0)),
            )
            .column_spacing(0)
            .widths(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            );

        Widget::render(table, area, buf)
    }
}

impl Default for RamTableState {
    fn default() -> Self {
        RamTableState {
            starting_row: 0,
            is_focussed: false,
        }
    }
}

impl RamTableState {
    pub fn scroll(&mut self, offset: i32) {
        let tmp = self.starting_row as i32 + offset;

        if tmp < 0 {
            self.starting_row = ((RAM_MAX_LINES as i32) + tmp) as u32;
        } else {
            self.starting_row = (tmp as u32) % RAM_MAX_LINES;
        }
    }

    pub fn goto_address(&mut self, address: u32) {
        self.starting_row = address.clamp(0, RAM_SIZE as u32) / 4;
    }
}
