use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Widget;

use crate::statusline_stacked::{
    OperationalMode, StatusLineStacked, StyledStatusLine, SLANT_BL_TR, SLANT_TL_BR,
};

impl<'a> Widget for StatusLineStacked<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut x_end = area.right();
        for (status, gap) in self.right.iter() {
            let width = status.width() as u16;
            status.render(
                Rect::new(x_end.saturating_sub(width), area.y, width, 1),
                buf,
            );
            x_end = x_end.saturating_sub(width);

            let width = gap.width() as u16;
            gap.render(
                Rect::new(x_end.saturating_sub(width), area.y, width, 1),
                buf,
            );
            x_end = x_end.saturating_sub(width);
        }

        let mut x_start = area.x;
        for (status, gap) in self.left.iter() {
            let width = status.width() as u16;
            status.render(Rect::new(x_start, area.y, width, 1), buf);
            x_start += width;

            let width = gap.width() as u16;
            gap.render(Rect::new(x_start, area.y, width, 1), buf);
            x_start += width;
        }

        buf.set_style(
            Rect::new(x_start, area.y, x_end.saturating_sub(x_start), 1),
            self.style,
        );

        let center_width = x_end
            .saturating_sub(x_start)
            .saturating_sub(self.center_margin * 2);

        self.center.render(
            Rect::new(x_start + self.center_margin, area.y, center_width, 1),
            buf,
        );
    }
}

impl<'a> StyledStatusLine<'a> {
    pub fn build(self) -> StatusLineStacked<'a> {
        use ratatui::style::Color;

        let color_title = Color::Rgb(70, 73, 77);
        let color_mode = match self.mode {
            OperationalMode::Operational => Color::Rgb(42, 193, 138),
            OperationalMode::Dire => Color::Rgb(255, 210, 88),
            OperationalMode::Evacuate => Color::Rgb(246, 90, 90),
        };
        let color_info = Color::Rgb(44, 163, 170);
        let color_dark = Color::Rgb(80, 202, 210);
        let text_black = Color::Rgb(16, 19, 23);

        let mode_str = match self.mode {
            OperationalMode::Operational => " OPERATIONAL ",
            OperationalMode::Dire => " DIRE ",
            OperationalMode::Evacuate => " EVACUATE ",
        };

        if self.use_slants {
            StatusLineStacked::new()
                .style(Style::new().fg(Color::White).bg(color_dark))
                .start(
                    Span::from(self.title).style(Style::new().fg(text_black).bg(color_title)),
                    Span::from(SLANT_TL_BR).style(Style::new().fg(color_title).bg(color_mode)),
                )
                .start(
                    Span::from(mode_str).style(Style::new().fg(text_black).bg(color_mode)),
                    Span::from(SLANT_TL_BR).style(Style::new().fg(color_mode)),
                )
                .center_margin(1)
                .center(self.center_text)
                .end(
                    Span::from(format!(
                        "R[{}][{}µs] ",
                        self.render_count, self.render_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info).bg(color_dark)),
                )
                .end(
                    "",
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_dark).bg(color_info)),
                )
                .end(
                    Span::from(format!(
                        "E[{}][{}µs] ",
                        self.event_count, self.event_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info).bg(color_dark)),
                )
                .end(
                    "",
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_dark).bg(color_info)),
                )
                .end(
                    Span::from(format!("MSG[{}] ", self.message_count))
                        .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info)),
                )
        } else {
            StatusLineStacked::new()
                .style(Style::new().fg(Color::White).bg(color_dark))
                .start_bare(
                    Span::from(self.title).style(Style::new().fg(Color::White).bg(color_title)),
                )
                .start_bare(Span::from(mode_str).style(Style::new().fg(text_black).bg(color_mode)))
                .center_margin(1)
                .center(self.center_text)
                .end_bare(
                    Span::from(format!(
                        "R[{}][{}µs] ",
                        self.render_count, self.render_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                )
                .end_bare(
                    Span::from(format!(
                        "E[{}][{}µs] ",
                        self.event_count, self.event_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                )
                .end_bare(
                    Span::from(format!(" MSG[{}] ", self.message_count))
                        .style(Style::new().fg(text_black).bg(color_info)),
                )
        }
    }
}
