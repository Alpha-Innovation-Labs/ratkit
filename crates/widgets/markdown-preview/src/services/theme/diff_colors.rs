/// Diff colors module for CodeDiff widget theming.
///
/// This module provides [`DiffColors`] which contains all the colors needed
/// for rendering unified diff views with syntax-highlighted additions,
/// removals, and context lines.
///
/// # Color Categories
///
/// The diff color scheme includes:
/// - **Line colors**: Colors for added, removed, and context line text
/// - **Background colors**: Background colors for different line types
/// - **Highlight colors**: Emphasized colors for inline changes
/// - **Line number colors**: Colors for the gutter line numbers
///
/// # Example
///
/// ```rust,ignore
/// use ratatui::style::Color;
/// use ratatui_toolkit::services::theme::DiffColors;
///
/// let colors = DiffColors::default();
/// // Use colors.added for added line text color
/// ```

use ratatui::style::Color;

/// Colors for rendering unified diff views.
///
/// This struct contains all the colors needed for the [`CodeDiff`](crate::CodeDiff)
/// widget to render diff output with proper syntax highlighting for additions,
/// removals, context lines, and hunk headers.
///
/// # Fields
///
/// The color scheme is organized into logical groups:
///
/// - **Line text colors**: `added`, `removed`, `context`, `hunk_header`
/// - **Highlight colors**: `highlight_added`, `highlight_removed` for inline changes
/// - **Background colors**: `added_bg`, `removed_bg`, `context_bg` for line backgrounds
/// - **Line number colors**: `line_number`, `added_line_number_bg`, `removed_line_number_bg`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffColors {
    /// Color for added line text (e.g., green).
    ///
    /// Used for lines prefixed with `+` in unified diff format.
    pub added: Color,

    /// Color for removed line text (e.g., red).
    ///
    /// Used for lines prefixed with `-` in unified diff format.
    pub removed: Color,

    /// Color for context line text (e.g., gray).
    ///
    /// Used for unchanged lines that provide context around changes.
    pub context: Color,

    /// Color for hunk header text (e.g., cyan).
    ///
    /// Used for lines like `@@ -1,3 +1,4 @@` that mark diff sections.
    pub hunk_header: Color,

    /// Highlight color for added text within a line.
    ///
    /// Used to emphasize specific added characters or words
    /// when showing inline changes.
    pub highlight_added: Color,

    /// Highlight color for removed text within a line.
    ///
    /// Used to emphasize specific removed characters or words
    /// when showing inline changes.
    pub highlight_removed: Color,

    /// Background color for added lines.
    ///
    /// A subtle green tint to distinguish added lines.
    pub added_bg: Color,

    /// Background color for removed lines.
    ///
    /// A subtle red tint to distinguish removed lines.
    pub removed_bg: Color,

    /// Background color for context lines.
    ///
    /// Usually the same as the panel background or slightly different.
    pub context_bg: Color,

    /// Color for line numbers in the gutter.
    ///
    /// Usually a muted color that doesn't distract from the diff content.
    pub line_number: Color,

    /// Background color for line numbers on added lines.
    ///
    /// Slightly tinted to match the added line background.
    pub added_line_number_bg: Color,

    /// Background color for line numbers on removed lines.
    ///
    /// Slightly tinted to match the removed line background.
    pub removed_line_number_bg: Color,
}


/// New constructor for [`DiffColors`].



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
    /// ```rust,ignore
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::services::theme::DiffColors;
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


/// Default trait implementation for [`DiffColors`].



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

