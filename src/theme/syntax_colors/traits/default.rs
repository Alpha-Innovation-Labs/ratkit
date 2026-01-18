//! Default trait implementation for [`SyntaxColors`].

use ratatui::style::Color;

use crate::theme::SyntaxColors;

impl Default for SyntaxColors {
    /// Creates a default syntax color scheme based on the Gruvbox dark theme.
    ///
    /// This provides a reasonable default that works well on dark terminal
    /// backgrounds with good contrast for code syntax highlighting.
    ///
    /// # Returns
    ///
    /// A `SyntaxColors` instance with Gruvbox-inspired colors.
    fn default() -> Self {
        Self {
            comment: Color::Rgb(146, 131, 116),     // gruvbox gray
            keyword: Color::Rgb(251, 73, 52),       // gruvbox bright red
            function: Color::Rgb(184, 187, 38),     // gruvbox bright green
            variable: Color::Rgb(131, 165, 152),    // gruvbox bright blue
            string: Color::Rgb(250, 189, 47),       // gruvbox bright yellow
            number: Color::Rgb(211, 134, 155),      // gruvbox bright purple
            type_: Color::Rgb(142, 192, 124),       // gruvbox bright aqua
            operator: Color::Rgb(254, 128, 25),     // gruvbox bright orange
            punctuation: Color::Rgb(235, 219, 178), // gruvbox fg1
        }
    }
}
