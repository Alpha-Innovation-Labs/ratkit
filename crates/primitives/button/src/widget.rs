//! Button widget for terminal UI applications.
//! # Example
//!
//! ```rust
//! use ratatui::style::{Color, Style};
//! use ratkit_button::Button;
//!
//! let button = Button::new("Click Me")
//!     .normal_style(Style::default().fg(Color::White))
//!     .hover_style(Style::default().fg(Color::Yellow));
//! ```

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::text::Span;

#[derive(Debug, Clone)]
pub struct Button {
    pub(crate) text: String,
    pub(crate) area: Option<Rect>,
    pub(crate) hovered: bool,
    pub(crate) normal_style: Style,
    pub(crate) hover_style: Style,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            area: None,
            hovered: false,
            normal_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            hover_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn area(&self) -> Option<Rect> {
        self.area
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn hover(&self) -> Style {
        self.hover_style
    }

    pub fn normal(&self) -> Style {
        self.normal_style
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }

    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    pub fn is_clicked(&self, column: u16, row: u16) -> bool {
        if let Some(area) = self.area {
            column >= area.x
                && column < area.x + area.width
                && row >= area.y
                && row < area.y + area.height
        } else {
            false
        }
    }

    pub fn update_hover(&mut self, column: u16, row: u16) {
        self.hovered = self.is_clicked(column, row);
    }

    pub fn render(&self, panel_area: Rect, _title_prefix: &str) -> (Span<'static>, Rect) {
        let button_text = format!(" [{}] ", self.text);
        let button_width = button_text.len() as u16;
        let button_x = panel_area.x + panel_area.width.saturating_sub(button_width + 2);
        let button_y = panel_area.y;

        let area = Rect {
            x: button_x,
            y: button_y,
            width: button_width,
            height: 1,
        };

        let style = if self.hovered {
            self.hover_style
        } else {
            self.normal_style
        };

        (Span::styled(button_text, style), area)
    }

    pub fn render_at_offset(
        &self,
        panel_area: Rect,
        offset_from_right: u16,
    ) -> (Span<'static>, Rect) {
        let button_text = format!(" [{}] ", self.text);
        let button_width = button_text.len() as u16;
        let button_x = panel_area
            .x
            .saturating_sub(offset_from_right + button_width + 2);
        let button_y = panel_area.y;

        let area = Rect {
            x: button_x,
            y: button_y,
            width: button_width,
            height: 1,
        };

        let style = if self.hovered {
            self.hover_style
        } else {
            self.normal_style
        };

        (Span::styled(button_text, style), area)
    }

    pub fn render_with_title(&mut self, panel_area: Rect, title: &str) -> Line<'static> {
        let (button_span, area) = self.render(panel_area, title);
        self.area = Some(area);
        let title_line = Line::from(vec![Span::raw(title.to_string()), button_span]);
        title_line
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("Button")
    }
}
