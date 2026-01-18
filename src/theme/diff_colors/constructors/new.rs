//! New constructor for [`DiffColors`].

use ratatui::style::Color;

use crate::theme::DiffColors;

impl DiffColors {
    /// Creates a new [`DiffColors`] instance with the specified colors.
    ///
    /// # Arguments
    ///
    /// * `added` - Color for added line text
    /// * `removed` - Color for removed line text
    /// * `context` - Color for context line text
    /// * `hunk_header` - Color for hunk header text
    /// * `highlight_added` - Highlight color for inline additions
    /// * `highlight_removed` - Highlight color for inline removals
    /// * `added_bg` - Background color for added lines
    /// * `removed_bg` - Background color for removed lines
    /// * `context_bg` - Background color for context lines
    /// * `line_number` - Color for line numbers
    /// * `added_line_number_bg` - Background for added line numbers
    /// * `removed_line_number_bg` - Background for removed line numbers
    ///
    /// # Returns
    ///
    /// A new `DiffColors` instance with all colors configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::theme::DiffColors;
    ///
    /// let colors = DiffColors::new(
    ///     Color::Green,
    ///     Color::Red,
    ///     Color::Gray,
    ///     Color::Cyan,
    ///     Color::LightGreen,
    ///     Color::LightRed,
    ///     Color::Rgb(0, 30, 0),
    ///     Color::Rgb(30, 0, 0),
    ///     Color::Rgb(20, 20, 20),
    ///     Color::DarkGray,
    ///     Color::Rgb(0, 25, 0),
    ///     Color::Rgb(25, 0, 0),
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        added: Color,
        removed: Color,
        context: Color,
        hunk_header: Color,
        highlight_added: Color,
        highlight_removed: Color,
        added_bg: Color,
        removed_bg: Color,
        context_bg: Color,
        line_number: Color,
        added_line_number_bg: Color,
        removed_line_number_bg: Color,
    ) -> Self {
        Self {
            added,
            removed,
            context,
            hunk_header,
            highlight_added,
            highlight_removed,
            added_bg,
            removed_bg,
            context_bg,
            line_number,
            added_line_number_bg,
            removed_line_number_bg,
        }
    }
}
