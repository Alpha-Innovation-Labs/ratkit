use ratatui::style::Color;

use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::enums::DiffStyle;

impl DiffConfig {
    /// Creates a new diff configuration with default values.
    ///
    /// Default colors are chosen to work well on both light and dark terminals:
    /// - Added lines: dark green background, bright green foreground
    /// - Removed lines: dark red background, bright red foreground
    /// - Hunk headers: gray background, cyan foreground
    ///
    /// # Returns
    ///
    /// A new `DiffConfig` with default settings
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffConfig;
    ///
    /// let config = DiffConfig::new();
    /// assert!(config.show_line_numbers);
    /// ```
    pub fn new() -> Self {
        Self {
            added_bg: Color::Rgb(0, 60, 0),
            added_fg: Color::Rgb(80, 200, 80),
            removed_bg: Color::Rgb(60, 0, 0),
            removed_fg: Color::Rgb(200, 80, 80),
            hunk_header_bg: Color::Rgb(40, 40, 40),
            hunk_header_fg: Color::Cyan,
            line_number_fg: Color::DarkGray,
            show_line_numbers: true,
            context_lines: 3,
            style: DiffStyle::SideBySide,
            gutter_width: 2,
            line_number_width: 4,
            sidebar_enabled: false,
            sidebar_default_width: 25,
            sidebar_min_width: 10,
            sidebar_max_width: 50,
        }
    }
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self::new()
    }
}
