//! Button component
//!
//! Provides clickable button widgets for UI interactions.
//!
//! # Structure
//!
//! - [`Button`] - The button widget struct
//! - [`constructors`] - Constructor functions (`new`, builder methods)
//! - [`methods`] - Instance methods (`render`, `is_clicked`, etc.)
//! - [`traits`] - Trait implementations (`Default`)
//! - [`render_title_with_buttons`] - Standalone function for rendering title with buttons
//!
//! # Example
//!
//! ```rust
//! use crate::button::Button;
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

pub mod constructors;
pub mod methods;
pub mod render_title_with_buttons;
pub mod traits;

use ratatui::style::{Color, Modifier, Style};

/// A clickable button widget for the UI
#[derive(Debug, Clone)]
pub struct Button {
    text: String,
    area: Option<ratatui::layout::Rect>,
    hovered: bool,
    normal_style: Style,
    hover_style: Style,
}
