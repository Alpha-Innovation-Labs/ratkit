//! Hotkey modal component
//!
//! A modal dialog for displaying hotkey bindings organized by sections.

pub mod constructors;
pub mod functions;
pub mod traits;

use ratatui::style::Color;

/// A single hotkey binding with its description
#[derive(Debug, Clone)]
pub struct Hotkey {
    pub key: String,
    pub description: String,
}

/// A category/section of hotkeys
#[derive(Debug, Clone)]
pub struct HotkeySection {
    pub title: String,
    pub hotkeys: Vec<Hotkey>,
}

/// Configuration for the hotkey modal appearance
#[derive(Debug, Clone)]
pub struct HotkeyModalConfig {
    /// Title of the modal
    pub title: String,
    /// Border color
    pub border_color: Color,
    /// Width as a percentage of screen width (0.0 - 1.0)
    pub width_percent: f32,
    /// Height as a percentage of screen height (0.0 - 1.0)
    pub height_percent: f32,
    /// Footer text (e.g., "Press any key to close")
    pub footer: Option<String>,
    /// Whether to show title inside content instead of in border
    pub title_inside: bool,
}
