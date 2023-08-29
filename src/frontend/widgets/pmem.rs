use crate::backend::program::Program;
use crate::backend::runtime::CpuState;

use ratatui::prelude::{Alignment, Buffer, Color, Constraint, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, StatefulWidget, Table, Widget};

pub struct PmemTableWidget<'a> {
    program: &'a Program,
    program_counter: &'a u16,
}

pub struct PmemTableState {
    pub selected: u32,
    pub viewport_begin: u32,
    pub viewport_end: u32,
    pub max_visible_lines: u32,
    pub focus_executing: bool, // True if table should center around executing instruction
    pub is_focussed: bool,
}

impl Default for PmemTableState {
    fn default() -> Self {
        PmemTableState {
            selected: 10,
            viewport_begin: 0,
            viewport_end: 24,
            max_visible_lines: 24,
            focus_executing: true,
            is_focussed: false,
        }
    }
}

impl PmemTableState {
    pub fn scroll(&mut self, offset: i32) {
        let viewport_begin_i32 = self.viewport_begin as i32;
        self.viewport_begin = viewport_begin_i32.saturating_add(offset).clamp(0, i32::MAX) as u32;
        self.viewport_end = self.viewport_begin + self.max_visible_lines;

        let selected_i32 = self.selected as i32;
        self.selected = selected_i32.saturating_add(offset).clamp(0, i32::MAX) as u32;
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
        // Clamp the selection index
        state.selected = state
            .selected
            .clamp(0, self.program.operations.len() as u32 - 1);

        // Focus the executing instruction if desired
        if state.focus_executing {
            // Let the selected be equal to the executing instruction
            state.selected = *self.program_counter as u32;

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
        } else {
            // Center the viewport around selected

            // Program counter outside viewport - this can happen during jumps
            if !(state.viewport_begin..state.viewport_end).contains(&state.selected) {
                state.viewport_begin = state.selected.saturating_sub(4);
            }
            // Program counter near top
            else if state.selected <= state.viewport_begin + 10 {
                state.viewport_begin = state.viewport_begin.saturating_sub(1);
            }
            // Program counter near bottom
            else if state.selected >= state.viewport_end - 10 {
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
            let address_str = format!("{:04x}", i);
            cells.push(
                Cell::from(address_str).fg(match self.program.breakpoints[i as usize] {
                    // Mark breakpoints
                    true => Color::Red,
                    false => Color::LightMagenta,
                }),
            );
            // Now the instruction hex code
            cells.push(
                Cell::from(format!(
                    "{:05x}",
                    self.program.instruction_words[i as usize].buffer
                ))
                .fg(Color::Green),
            );
            // Finally the disassembled representation
            cells.push(Cell::from(
                self.program.operations[i as usize].get_assembly_string(),
            ));

            let mut row = Row::new(cells);

            // Highlight the currently executing instruction

            if i == *self.program_counter as u32 {
                row = row.bg(Color::DarkGray);
            }
            if i == state.selected {
                row = row.bold();
            }

            // Push the row
            rows.push(row);
        }

        let table = Table::new(rows)
            .block(
                Block::default()
                    .title(" Program Memory ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(match state.is_focussed {
                        true => BorderType::Thick,
                        false => BorderType::Plain,
                    }),
            )
            .column_spacing(1)
            .widths(
                [
                    Constraint::Min(5),
                    Constraint::Min(6),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            );

        Widget::render(table, area, buf)
    }
}
