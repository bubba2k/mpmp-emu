use crate::runtime::CpuState;
use crate::runtime::RAM_SIZE;

use ratatui::prelude::{Buffer, Color, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget};

const LINE_MAX: u32 = (u16::MAX / 4) as u32;

pub struct RamTableWidget<'a> {
    ram_ref: &'a [u16; RAM_SIZE],
}

pub struct RamTableState {
    pub starting_row: u32,
    pub n_rows: u32,
}

impl<'a> RamTableWidget<'a> {
    pub fn new(cpu: &'a CpuState) -> Self {
        RamTableWidget { ram_ref: &cpu.ram }
    }
}

impl<'a> StatefulWidget for RamTableWidget<'a> {
    type State = RamTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Build the table
        let mut rows: Vec<Row> = Vec::new();

        let ending_row = state.starting_row + state.n_rows;
        for row in state.starting_row..ending_row {
            let row_mem_address = row * 4;

            // Push row address
            let mut cells =
                vec![Cell::from(format!("{:#X}", row_mem_address)).fg(Color::LightMagenta)];

            // Push the row values
            for address in row_mem_address..(row_mem_address + 4) {
                cells.push(Cell::from(format!("{:#X}", self.ram_ref[address as usize])));
            }

            rows.push(Row::new(cells));
        }

        let table = Table::new(rows)
            .block(Block::default().title(" RAM ").borders(Borders::ALL))
            .column_spacing(1);

        Widget::render(table, area, buf)
    }
}

impl Default for RamTableState {
    fn default() -> Self {
        RamTableState {
            starting_row: 0,
            n_rows: 16,
        }
    }
}

impl RamTableState {
    pub fn scroll(&mut self, offset: i32) {
        let tmp = self.starting_row as i32 + offset;

        self.starting_row = tmp.rem_euclid(LINE_MAX as i32) as u32;
    }
}
