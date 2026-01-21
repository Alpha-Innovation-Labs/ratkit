mod with_theme;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::Frame;

use crate::widgets::hotkey_footer::HotkeyFooter;

impl HotkeyFooter {
    fn build_line(&self) -> Line<'static> {
        let mut spans = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if i == 0 {
                spans.push(Span::raw(" "));
            }

            spans.push(Span::styled(
                item.key.clone(),
                Style::default()
                    .fg(self.key_color)
                    .add_modifier(Modifier::BOLD),
            ));

            spans.push(Span::styled(
                format!(" {}  ", item.description),
                Style::default().fg(self.description_color),
            ));
        }

        Line::from(spans)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        frame.render_widget(widget, area);
    }
}

impl Widget for HotkeyFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        widget.render(area, buf);
    }
}

impl Widget for &HotkeyFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        widget.render(area, buf);
    }
}
