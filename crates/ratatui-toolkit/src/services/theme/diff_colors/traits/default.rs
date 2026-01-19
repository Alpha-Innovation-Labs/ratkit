//! Default trait implementation for [`DiffColors`].

use ratatui::style::Color;

use crate::services::theme::DiffColors;

impl Default for DiffColors {
    /// Creates a default diff color scheme based on the Gruvbox dark theme.
    ///
    /// This provides a reasonable default that works well on dark terminal
    /// backgrounds with good contrast for additions and removals.
    ///
    /// # Returns
    ///
    /// A `DiffColors` instance with Gruvbox-inspired colors.
    fn default() -> Self {
        Self {
            added: Color::Rgb(152, 151, 26),                // gruvbox green
            removed: Color::Rgb(204, 36, 29),               // gruvbox red
            context: Color::Rgb(146, 131, 116),             // gruvbox gray
            hunk_header: Color::Rgb(104, 157, 106),         // gruvbox aqua
            highlight_added: Color::Rgb(184, 187, 38),      // gruvbox bright green
            highlight_removed: Color::Rgb(251, 73, 52),     // gruvbox bright red
            added_bg: Color::Rgb(50, 48, 47),               // dark green tint
            removed_bg: Color::Rgb(50, 41, 41),             // dark red tint
            context_bg: Color::Rgb(60, 56, 54),             // gruvbox bg1
            line_number: Color::Rgb(102, 92, 84),           // gruvbox bg3
            added_line_number_bg: Color::Rgb(42, 40, 39),   // subtle green tint
            removed_line_number_bg: Color::Rgb(42, 34, 34), // subtle red tint
        }
    }
}
