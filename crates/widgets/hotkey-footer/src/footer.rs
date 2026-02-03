use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::Frame;

use crate::hotkey::HotkeyItem;

#[derive(Clone, Debug)]
pub struct HotkeyFooter {
    pub items: Vec<HotkeyItem>,
    pub key_color: Color,
    pub description_color: Color,
    pub background_color: Color,
}

impl HotkeyFooter {
    pub fn new(items: Vec<HotkeyItem>) -> Self {
        Self {
            items,
            key_color: Color::Cyan,
            description_color: Color::DarkGray,
            background_color: Color::Black,
        }
    }

    pub fn with_theme_colors(
        mut self,
        key_color: Color,
        description_color: Color,
        background_color: Color,
    ) -> Self {
        self.key_color = key_color;
        self.description_color = description_color;
        self.background_color = background_color;
        self
    }

    pub fn key_color(mut self, color: Color) -> Self {
        self.key_color = color;
        self
    }

    pub fn description_color(mut self, color: Color) -> Self {
        self.description_color = color;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

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
