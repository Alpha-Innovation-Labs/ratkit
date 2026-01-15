//! Button component
//!
//! Provides clickable button widgets for UI interactions.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// A clickable button widget for the UI
#[derive(Debug, Clone)]
pub struct Button {
    /// The text displayed on the button
    pub text: String,
    /// The area where the button is rendered (for click detection)
    pub area: Option<Rect>,
    /// Whether the button is currently hovered
    pub hovered: bool,
    /// Normal style (not hovered)
    pub normal_style: Style,
    /// Hover style
    pub hover_style: Style,
}

impl Button {
    /// Create a new button with default styling
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

    /// Set custom normal style
    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    /// Set custom hover style
    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    /// Check if a mouse click at (column, row) is within the button area
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

    /// Update hover state based on mouse position
    pub fn update_hover(&mut self, column: u16, row: u16) {
        self.hovered = self.is_clicked(column, row);
    }

    /// Render the button as a styled span
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

    /// Create a complete title line with the button on the right
    pub fn render_with_title(&mut self, panel_area: Rect, title: &str) -> Line<'static> {
        let (button_span, area) = self.render(panel_area, title);
        self.area = Some(area);
        let title_line = Line::from(vec![Span::raw(title.to_string()), button_span]);
        title_line
    }

    /// Render button at a specific position (for multiple buttons)
    pub fn render_at_offset(
        &self,
        panel_area: Rect,
        offset_from_right: u16,
    ) -> (Span<'static>, Rect) {
        let button_text = format!(" [{}] ", self.text);
        let button_width = button_text.len() as u16;
        let button_x = panel_area.x
            + panel_area
                .width
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
}

impl Default for Button {
    fn default() -> Self {
        Self::new("Button")
    }
}

/// Helper function to render multiple buttons in a title
pub fn render_title_with_buttons(
    panel_area: Rect,
    title: &str,
    buttons: &mut [&mut Button],
) -> Line<'static> {
    let mut spans = vec![Span::raw(title.to_string())];

    let mut offset = 0u16;

    for button in buttons.iter_mut().rev() {
        let (button_span, area) = button.render_at_offset(panel_area, offset);
        button.area = Some(area);

        let button_width = format!(" [{}] ", button.text).len() as u16;
        offset += button_width;

        spans.insert(1, button_span);
    }

    Line::from(spans)
}
