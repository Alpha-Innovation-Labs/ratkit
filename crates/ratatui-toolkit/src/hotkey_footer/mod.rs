//! Hotkey footer component
//!
//! A styled hotkey footer bar component (aerospace-tui style)
//! Renders a single line with alternating hotkey/description pairs.

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::style::Color;

/// A single hotkey with its description
#[derive(Clone, Debug)]
pub struct HotkeyItem {
    /// The key or key combination (e.g., "j/k", "Enter", "?")
    pub key: String,
    /// The description of what the key does (e.g., "scroll", "navigate", "help")
    pub description: String,
}

/// A styled hotkey footer bar component (aerospace-tui style)
#[derive(Clone, Debug)]
pub struct HotkeyFooter {
    /// List of hotkey items to display
    pub items: Vec<HotkeyItem>,
    /// Color for the hotkeys
    pub key_color: Color,
    /// Color for the descriptions
    pub description_color: Color,
    /// Background color for the footer
    pub background_color: Color,
}

/// Builder pattern for creating hotkey footers
pub struct HotkeyFooterBuilder {
    items: Vec<HotkeyItem>,
}
