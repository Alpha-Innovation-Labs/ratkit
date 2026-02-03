use crate::types::Dialog;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

pub struct DialogWidget<'a> {
    pub dialog: &'a mut Dialog<'a>,
}

impl<'a> DialogWidget<'a> {
    pub fn new(dialog: &'a mut Dialog<'a>) -> Self {
        Self { dialog }
    }
}

impl Widget for DialogWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.dialog.overlay {
            let overlay = Paragraph::new("").style(self.dialog.overlay_style);
            overlay.render(area, buf);
        }

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

        let block = if self.dialog.title_inside {
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.dialog.get_border_color()))
                .style(self.dialog.style)
        } else {
            Block::default()
                .title(self.dialog.title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.dialog.get_border_color()))
                .style(self.dialog.style)
        };

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        let mut lines: Vec<Line> = Vec::new();

        if self.dialog.title_inside {
            lines.push(Line::from(vec![Span::styled(
                self.dialog.title,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(self.dialog.get_border_color()),
            )]));
            lines.push(Line::from(""));
        }

        for line in self.dialog.message.lines() {
            lines.push(Line::from(line));
        }

        if let Some(footer) = self.dialog.footer {
            if !lines.is_empty() {
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![Span::styled(
                footer,
                self.dialog.footer_style,
            )]));
        }

        let message = Paragraph::new(lines)
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
