//! New constructor for [`AppTheme`].

use ratatui::style::Color;

use crate::theme::app_theme::AppTheme;
use crate::theme::diff_colors::DiffColors;
use crate::theme::markdown_colors::MarkdownColors;
use crate::theme::syntax_colors::SyntaxColors;

impl AppTheme {
    /// Creates a new [`AppTheme`] with all colors specified.
    ///
    /// This is a low-level constructor that requires all colors to be provided.
    /// For most use cases, prefer [`AppTheme::default()`] or [`AppTheme::from_json()`].
    ///
    /// # Arguments
    ///
    /// * `primary` - Primary UI color
    /// * `secondary` - Secondary UI color
    /// * `accent` - Accent color
    /// * `error` - Error color
    /// * `warning` - Warning color
    /// * `success` - Success color
    /// * `info` - Info color
    /// * `text` - Primary text color
    /// * `text_muted` - Muted text color
    /// * `selected_text` - Selected text color
    /// * `background` - Main background color
    /// * `background_panel` - Panel background color
    /// * `background_element` - Element background color
    /// * `background_menu` - Menu background color
    /// * `border` - Default border color
    /// * `border_active` - Active border color
    /// * `border_subtle` - Subtle border color
    /// * `diff` - Diff colors
    /// * `markdown` - Markdown colors
    /// * `syntax` - Syntax colors
    ///
    /// # Returns
    ///
    /// A new `AppTheme` instance with all colors configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::theme::{AppTheme, DiffColors, MarkdownColors, SyntaxColors};
    ///
    /// let theme = AppTheme::new(
    ///     Color::Blue, Color::Magenta, Color::Cyan,
    ///     Color::Red, Color::Yellow, Color::Green, Color::LightBlue,
    ///     Color::White, Color::Gray, Color::White,
    ///     Color::Black, Color::DarkGray, Color::DarkGray, Color::DarkGray,
    ///     Color::Gray, Color::White, Color::DarkGray,
    ///     DiffColors::default(),
    ///     MarkdownColors::default(),
    ///     SyntaxColors::default(),
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        primary: Color,
        secondary: Color,
        accent: Color,
        error: Color,
        warning: Color,
        success: Color,
        info: Color,
        text: Color,
        text_muted: Color,
        selected_text: Color,
        background: Color,
        background_panel: Color,
        background_element: Color,
        background_menu: Color,
        border: Color,
        border_active: Color,
        border_subtle: Color,
        diff: DiffColors,
        markdown: MarkdownColors,
        syntax: SyntaxColors,
    ) -> Self {
        Self {
            primary,
            secondary,
            accent,
            error,
            warning,
            success,
            info,
            text,
            text_muted,
            selected_text,
            background,
            background_panel,
            background_element,
            background_menu,
            border,
            border_active,
            border_subtle,
            diff,
            markdown,
            syntax,
        }
    }
}
