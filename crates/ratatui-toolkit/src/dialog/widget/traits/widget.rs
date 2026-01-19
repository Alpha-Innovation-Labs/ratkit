use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::dialog::DialogWidget;

impl Widget for DialogWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dialog_width = (area.width as f32 * self.dialog.width_percent) as u16;
        let dialog_height = (area.height as f32 * self.dialog.height_percent) as u16;
        let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
        let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x: area.x + dialog_x,
            y: area.y + dialog_y,
            width: dialog_width,
            height: dialog_height,
        };

        Clear.render(dialog_area, buf);

        let block = Block::default()
            .title(self.dialog.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.dialog.get_border_color()))
            .style(self.dialog.style);

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        let message = Paragraph::new(self.dialog.message)
            .style(self.dialog.style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        message.render(chunks[0], buf);

        self.dialog.button_areas.clear();

        if !self.dialog.buttons.is_empty() {
            let total_button_width: usize = self.dialog.buttons.iter().map(|b| b.len() + 4).sum();
            let button_area_width = chunks[1].width as usize;
            let start_x = if total_button_width < button_area_width {
                chunks[1].x + ((button_area_width - total_button_width) / 2) as u16
            } else {
                chunks[1].x
            };

            let mut x = start_x;
            let y = chunks[1].y + 1;

            for (idx, button_text) in self.dialog.buttons.iter().enumerate() {
                let button_width = button_text.len() as u16 + 2;
                let style = if idx == self.dialog.selected_button {
                    self.dialog.button_selected_style
                } else {
                    self.dialog.button_style
                };

                let button_area = Rect {
                    x,
                    y,
                    width: button_width,
                    height: 1,
                };

                self.dialog.button_areas.push(button_area);

                for bx in x..x + button_width {
                    if let Some(cell) = buf.cell_mut((bx, y)) {
                        cell.set_style(style);
                    }
                }

                let button_line =
                    Line::from(vec![Span::styled(format!(" {} ", button_text), style)]);

                buf.set_line(x, y, &button_line, button_width);
                x += button_width + 2;
            }
        }
    }
}
