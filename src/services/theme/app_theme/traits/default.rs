//! Default trait implementation for [`AppTheme`].

use ratatui::style::Color;

use crate::services::theme::app_theme::AppTheme;
use crate::services::theme::diff_colors::DiffColors;
use crate::services::theme::markdown_colors::MarkdownColors;
use crate::services::theme::syntax_colors::SyntaxColors;

impl Default for AppTheme {
    /// Creates a default application theme based on the Gruvbox dark theme.
    ///
    /// This provides a cohesive dark theme that works well in most terminals
    /// and provides good contrast and readability.
    ///
    /// # Returns
    ///
    /// An `AppTheme` instance with Gruvbox dark colors.
    fn default() -> Self {
        Self {
            // UI colors
            primary: Color::Rgb(131, 165, 152), // gruvbox bright blue
            secondary: Color::Rgb(211, 134, 155), // gruvbox bright purple
            accent: Color::Rgb(142, 192, 124),  // gruvbox bright aqua
            error: Color::Rgb(251, 73, 52),     // gruvbox bright red
            warning: Color::Rgb(254, 128, 25),  // gruvbox bright orange
            success: Color::Rgb(184, 187, 38),  // gruvbox bright green
            info: Color::Rgb(250, 189, 47),     // gruvbox bright yellow

            // Text colors
            text: Color::Rgb(235, 219, 178),          // gruvbox fg1
            text_muted: Color::Rgb(146, 131, 116),    // gruvbox gray
            selected_text: Color::Rgb(251, 241, 199), // gruvbox fg0

            // Background colors
            background: Color::Rgb(40, 40, 40), // gruvbox bg0
            background_panel: Color::Rgb(60, 56, 54), // gruvbox bg1
            background_element: Color::Rgb(80, 73, 69), // gruvbox bg2
            background_menu: Color::Rgb(60, 56, 54), // gruvbox bg1

            // Border colors
            border: Color::Rgb(102, 92, 84),          // gruvbox bg3
            border_active: Color::Rgb(235, 219, 178), // gruvbox fg1
            border_subtle: Color::Rgb(80, 73, 69),    // gruvbox bg2

            // Specialized colors
            diff: DiffColors::default(),
            markdown: MarkdownColors::default(),
            syntax: SyntaxColors::default(),
        }
    }
}
