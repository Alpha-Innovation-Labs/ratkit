//! Configuration for minimap appearance.

use ratatui::style::Style;

/// Configuration for minimap appearance.
#[derive(Debug, Clone)]
pub struct MinimapConfig {
    /// Width of the minimap in characters.
    pub width: u16,
    /// Height of the minimap in characters (for corner overlay mode).
    pub height: u16,
    /// Style for the minimap text (Braille characters).
    pub text_style: Style,
    /// Style for the viewport indicator.
    pub viewport_style: Style,
    /// Style for the minimap border/background.
    pub background_style: Style,
    /// Whether to show line density or just structure.
    pub show_density: bool,
}
