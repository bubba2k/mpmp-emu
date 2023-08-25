use crate::program::Program;
use crate::runtime::CpuState;

use num_traits::{clamp_max, clamp_min};
use ratatui::prelude::{Buffer, Color, Rect};
use ratatui::style::{Styled, Stylize};
use ratatui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget};

pub struct PmemTableWidget<'a> {
    program: &'a Program,
    program_counter: &'a u16,
}

pub struct PmemTableState {
    pub starting_row: u32,
    pub n_rows: u32,
    pub focus_executing: bool, // True if table should center around executing instruction
}

impl Default for PmemTableState {
    fn default() -> Self {
        PmemTableState {
            starting_row: 0,
            n_rows: 24,
            focus_executing: true,
        }
    }
}

impl<'a> PmemTableWidget<'a> {
    pub fn new(cpu: &'a CpuState, program: &'a Program) -> Self {
        PmemTableWidget {
            program_counter: &cpu.pcounter,
            program,
        }
    }
}

impl<'a> StatefulWidget for PmemTableWidget<'a> {
    type State = PmemTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Make sure nothing bad happens
        state.n_rows = clamp_max(state.n_rows, self.program.operations.len() as u32);

        // Focus on the executing instruction if required
        if state.focus_executing {
            state.starting_row = *self.program_counter as u32 - (state.n_rows / 2);
        }

        // Create the empty row vector to be populated
        let mut rows = Vec::new();

        for i in state.starting_row..(state.starting_row + state.n_rows) {
            let mut cells = Vec::new();

            // Build the cells, address first
            let address_str = format!("{:#X}", i);
            cells.push(Cell::from(address_str).fg(Color::LightMagenta));
            // Now the instruction hex code
            cells.push(
                Cell::from(format!(
                    "{:X}",
                    self.program.instruction_words[i as usize].buffer
                ))
                .fg(Color::LightBlue),
            );
            // Finally the disassembled representation
            cells.push(Cell::from(
                self.program.operations[i as usize].get_assembly_string(),
            ));

            let mut row = Row::new(cells);

            // Highlight the currently executing instruction
            if i == *self.program_counter as u32 {
                row = row.bg(Color::LightRed);
            }

            // Push the row
            rows.push(row);
        }

        let table = Table::new(rows)
            .block(
                Block::default()
                    .title(" Program Memory ")
                    .borders(Borders::ALL),
            )
            .column_spacing(1);

        Widget::render(table, area, buf)
    }
}
