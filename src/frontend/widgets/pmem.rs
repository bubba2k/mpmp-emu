use crate::backend::program::Program;
use crate::backend::runtime::CpuState;

use num_traits::{clamp_max, clamp_min};
use ratatui::prelude::{Buffer, Color, Constraint, Rect};
use ratatui::style::{Styled, Stylize};
use ratatui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget};

pub struct PmemTableWidget<'a> {
    program: &'a Program,
    program_counter: &'a u16,
}

pub struct PmemTableState {
    pub viewport_begin: u32,
    pub viewport_end: u32,
    pub max_visible_lines: u32,
    pub focus_executing: bool, // True if table should center around executing instruction
}

impl Default for PmemTableState {
    fn default() -> Self {
        PmemTableState {
            viewport_begin: 0,
            viewport_end: 24,
            max_visible_lines: 24,
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
        // Focus the executing instruction if desired
        if state.focus_executing {
            let program_counter_u32 = *self.program_counter as u32;
            // Program counter outside viewport - this can happen during jumps
            if !(state.viewport_begin..state.viewport_end).contains(&program_counter_u32) {
                state.viewport_begin = program_counter_u32.saturating_sub(4);
            }
            // Program counter near top
            else if program_counter_u32 <= state.viewport_begin + 10 {
                state.viewport_begin = state.viewport_begin.saturating_sub(1);
            }
            // Program counter near bottom
            else if program_counter_u32 >= state.viewport_end - 10 {
                state.viewport_begin = state.viewport_begin.saturating_add(1);
            }

            state.viewport_end = state.viewport_begin.saturating_add(state.max_visible_lines);
        }

        // Create the empty row vector to be populated
        let mut rows = Vec::new();

        // Build the rows, make sure to clamp the viewport to progam begin/end
        for i in state
            .viewport_begin
            .clamp(0, self.program.operations.len() as u32)
            ..state
                .viewport_end
                .clamp(0, self.program.operations.len() as u32)
        {
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
            .column_spacing(1)
            .widths(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            );

        Widget::render(table, area, buf)
    }
}
