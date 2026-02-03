//! Button component for terminal UI applications.
//!
//! # Module Structure
//!
//! - [`core`] - Button struct, constructors, and accessor methods
//! - [`render`] - Rendering utilities and composition
//! - [`interact`] - Interaction handling (click detection, hover state)
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

pub mod core;
pub mod interact;
pub mod render;

pub use core::Button;
pub use render::with_title::render_title_with_buttons;
