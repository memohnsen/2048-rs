use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
};

pub struct Grid {
    pub cells: [[u32; 4]; 4],
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..4).map(|_| Constraint::Length(15));
        let row_constraints = (0..4).map(|_| Constraint::Length(5));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let row = i / 4;
            let col = i % 4;
            let value = self.cells[row][col];

            let text = if value == 0 {
                String::new()
            } else {
                value.to_string()
            };

            Paragraph::new(text.bold())
                .block(Block::bordered())
                .centered()
                .render(cell, buf);
        }
    }
}
