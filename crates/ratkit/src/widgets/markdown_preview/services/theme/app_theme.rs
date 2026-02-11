use crate::widgets::markdown_preview::services::theme::ThemeVariant;
/// Application theme module for comprehensive TUI theming.
///
/// This module provides [`AppTheme`] which is the main theme struct that
/// contains all colors needed for a complete TUI application, including
/// UI colors, text colors, background colors, border colors, and specialized
/// color sets for diffs, markdown, and syntax highlighting.
///
/// # Color Categories
///
/// The theme is organized into logical categories:
///
/// - **UI Colors**: `primary`, `secondary`, `accent`, `error`, `warning`, `success`, `info`
/// - **Text Colors**: `text`, `text_muted`, `selected_text`
/// - **Background Colors**: `background`, `background_panel`, `background_element`, `background_menu`
/// - **Border Colors**: `border`, `border_active`, `border_subtle`
/// - **Specialized**: [`DiffColors`], [`MarkdownColors`], [`SyntaxColors`]
///
/// # Loading Themes
///
/// Themes can be loaded from JSON files in the opencode format using the
/// [`loader`](crate::widgets::markdown_preview::services::theme::loader) module.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::services::theme::{AppTheme, ThemeVariant};
///
/// // Use default theme
/// let theme = AppTheme::default();
///
/// // Access UI colors
/// let primary = theme.primary;
/// let error = theme.error;
///
/// // Access specialized colors
/// let diff_added = theme.diff.added;
/// let heading_color = theme.markdown.heading;
/// ```
use ratatui::style::Color;

use crate::widgets::markdown_preview::services::theme::diff_colors::DiffColors;
use crate::widgets::markdown_preview::services::theme::markdown_colors::MarkdownColors;
use crate::widgets::markdown_preview::services::theme::syntax_colors::SyntaxColors;

/// Comprehensive application theme with all widget colors.
///
/// This struct provides a complete color scheme for TUI applications,
/// covering all common UI elements and specialized widget colors.
///
/// # Theme Structure
///
/// The theme is organized into:
///
/// 1. **UI Colors** - Semantic colors for interactive elements
/// 2. **Text Colors** - Colors for text content
/// 3. **Background Colors** - Surface and container backgrounds
/// 4. **Border Colors** - Border and divider colors
/// 5. **Diff Colors** - Colors for diff rendering
/// 6. **Markdown Colors** - Colors for markdown content
/// 7. **Syntax Colors** - Colors for code syntax highlighting
///
/// # Loading from JSON
///
/// Use [`AppTheme::from_json`] or the loader module to load themes
/// from opencode-format JSON files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppTheme {
    // ========== UI Colors ==========
    /// Primary UI color for main interactive elements.
    ///
    /// Used for primary buttons, active selections, and key UI elements.
    pub primary: Color,

    /// Secondary UI color for supporting elements.
    ///
    /// Used for secondary buttons and less prominent interactive elements.
    pub secondary: Color,

    /// Accent color for highlighting and emphasis.
    ///
    /// Used to draw attention to specific UI elements.
    pub accent: Color,

    /// Error color for error states and messages.
    ///
    /// Used for error indicators, validation errors, and destructive actions.
    pub error: Color,

    /// Warning color for warning states and messages.
    ///
    /// Used for warnings and caution indicators.
    pub warning: Color,

    /// Success color for success states and messages.
    ///
    /// Used for success indicators and confirmation feedback.
    pub success: Color,

    /// Info color for informational elements.
    ///
    /// Used for help text, hints, and informational messages.
    pub info: Color,

    // ========== Text Colors ==========
    /// Primary text color for main content.
    ///
    /// The default color for body text and content.
    pub text: Color,

    /// Muted text color for secondary content.
    ///
    /// Used for less important text, placeholders, and hints.
    pub text_muted: Color,

    /// Text color for selected items.
    ///
    /// Used for text within selected or highlighted regions.
    pub selected_text: Color,

    // ========== Background Colors ==========
    /// Main background color.
    ///
    /// The primary application background.
    pub background: Color,

    /// Panel background color.
    ///
    /// Used for content panels and cards.
    pub background_panel: Color,

    /// Element background color.
    ///
    /// Used for interactive elements like buttons and inputs.
    pub background_element: Color,

    /// Menu background color.
    ///
    /// Used for dropdown menus and popover backgrounds.
    pub background_menu: Color,

    // ========== Border Colors ==========
    /// Default border color.
    ///
    /// Used for container borders and dividers.
    pub border: Color,

    /// Active border color.
    ///
    /// Used for focused or active element borders.
    pub border_active: Color,

    /// Subtle border color.
    ///
    /// Used for subtle dividers and less prominent borders.
    pub border_subtle: Color,

    // ========== Specialized Color Sets ==========
    /// Colors for diff rendering.
    ///
    /// Contains all colors needed for the CodeDiff widget.
    pub diff: DiffColors,

    /// Colors for markdown rendering.
    ///
    /// Contains all colors needed for the MarkdownWidget.
    pub markdown: MarkdownColors,

    /// Colors for syntax highlighting.
    ///
    /// Contains all colors needed for code syntax highlighting.
    pub syntax: SyntaxColors,
}

/// JSON constructor for [`AppTheme`].
use std::path::Path;

use crate::widgets::markdown_preview::services::theme::loader::{load_theme_file, load_theme_str};

impl AppTheme {
    /// Creates an [`AppTheme`] from a JSON string in opencode format.
    ///
    /// This parses the JSON, resolves all color references from the `defs`
    /// section, and constructs a complete theme.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON string in opencode theme format
    /// * `variant` - Which theme variant (dark/light) to use
    ///
    /// # Returns
    ///
    /// `Ok(AppTheme)` if parsing succeeds, `Err` with a description otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The JSON is malformed
    /// - Color values cannot be resolved
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::services::theme::{AppTheme, ThemeVariant};
    ///
    /// let json = r#"{
    ///   "defs": { "myBlue": "#0066ff" },
    ///   "theme": { "primary": { "dark": "myBlue", "light": "myBlue" } }
    /// }"#;
    ///
    /// let theme = AppTheme::from_json(json, ThemeVariant::Dark)
    ///     .expect("Failed to parse theme");
    /// ```
    pub fn from_json(json: &str, variant: ThemeVariant) -> Result<Self, String> {
        load_theme_str(json, variant)
    }

    /// Creates an [`AppTheme`] from a JSON file path.
    ///
    /// Reads the file and parses it as an opencode theme.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON theme file
    /// * `variant` - Which theme variant (dark/light) to use
    ///
    /// # Returns
    ///
    /// `Ok(AppTheme)` if the file can be read and parsed,
    /// `Err` with a description otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The JSON is malformed
    /// - Color values cannot be resolved
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use ratatui_toolkit::services::theme::{AppTheme, ThemeVariant};
    ///
    /// let theme = AppTheme::from_json_file("themes/gruvbox.json", ThemeVariant::Dark)
    ///     .expect("Failed to load theme");
    /// ```
    pub fn from_json_file<P: AsRef<Path>>(path: P, variant: ThemeVariant) -> Result<Self, String> {
        load_theme_file(path, variant)
    }
}

/// New constructor for [`AppTheme`].

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
    /// ```rust,ignore
    /// use ratatui::style::Color;
    /// use ratatui_toolkit::services::theme::{AppTheme, DiffColors, MarkdownColors, SyntaxColors};
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

/// Color resolution methods for [`AppTheme`].

impl AppTheme {
    /// Gets a semantic color by name.
    ///
    /// This method allows looking up theme colors by their semantic name,
    /// which is useful for dynamic color resolution from configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The semantic name of the color (e.g., "primary", "error")
    ///
    /// # Returns
    ///
    /// `Some(Color)` if the name matches a known semantic color,
    /// `None` otherwise.
    ///
    /// # Supported Names
    ///
    /// - UI: `primary`, `secondary`, `accent`, `error`, `warning`, `success`, `info`
    /// - Text: `text`, `text_muted`, `selected_text`
    /// - Background: `background`, `background_panel`, `background_element`, `background_menu`
    /// - Border: `border`, `border_active`, `border_subtle`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::services::theme::AppTheme;
    ///
    /// let theme = AppTheme::default();
    /// let primary = theme.get_color("primary");
    /// assert!(primary.is_some());
    /// ```
    pub fn get_color(&self, name: &str) -> Option<Color> {
        match name {
            // UI colors
            "primary" => Some(self.primary),
            "secondary" => Some(self.secondary),
            "accent" => Some(self.accent),
            "error" => Some(self.error),
            "warning" => Some(self.warning),
            "success" => Some(self.success),
            "info" => Some(self.info),
            // Text colors
            "text" => Some(self.text),
            "text_muted" | "textMuted" => Some(self.text_muted),
            "selected_text" | "selectedText" => Some(self.selected_text),
            // Background colors
            "background" => Some(self.background),
            "background_panel" | "backgroundPanel" => Some(self.background_panel),
            "background_element" | "backgroundElement" => Some(self.background_element),
            "background_menu" | "backgroundMenu" => Some(self.background_menu),
            // Border colors
            "border" => Some(self.border),
            "border_active" | "borderActive" => Some(self.border_active),
            "border_subtle" | "borderSubtle" => Some(self.border_subtle),
            _ => None,
        }
    }
}

/// Default trait implementation for [`AppTheme`].

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
