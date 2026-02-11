//! Hotkey footer widget for ratatui.
//!
//! A styled hotkey footer bar component (aerospace-tui style)
//! Renders a single line with alternating hotkey/description pairs.
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::Color;
//! use ratkit_hotkey_footer::{HotkeyFooter, HotkeyItem};
//!
//! let footer = HotkeyFooter::new(vec![
//!     HotkeyItem::new("q", "quit"),
//!     HotkeyItem::new("?", "help"),
//! ])
//! .key_color(Color::Cyan)
//! .description_color(Color::DarkGray)
//! .background_color(Color::Black);
//! ```

mod footer;
mod hotkey;

pub use footer::HotkeyFooter;
pub use hotkey::HotkeyItem;
