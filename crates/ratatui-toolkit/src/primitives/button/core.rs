//! Button component
//!
//! Provides clickable button widgets for UI interactions.
//!
//! # Structure
//!
//! - [`Button`] - The button widget struct
//! - [`core`] - Core functionality and constructors
//! - [`render`] - Rendering utilities
//! - [`interact`] - Interaction handling
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::{Color, Style};
//! use ratatui_toolkit::Button;
//!
//! let button = Button::new("Click Me")
//!     .normal_style(Style::default().fg(Color::White))
//!     .hover_style(Style::default().fg(Color::Yellow));
//! ```
//!
//! # Click Detection
//!
//! Buttons track their own area for click detection. Use [`Button::is_clicked`] to
//! check if a click occurred within the button's bounds after rendering.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};

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
}

impl Default for Button {
    fn default() -> Self {
        Self::new("Button")
    }
}
