/// Theme definitions for markdown rendering.
///
/// Supports loading themes from JSON files with named colors and light/dark mode.
///
/// This module provides:
/// - [`ColorPalette`] - Color palette mapping named colors to RGB values
/// - [`MarkdownTheme`] - Markdown theme configuration with support for light/dark modes
/// - [`MarkdownStyle`] - Configuration for markdown rendering styles
/// - [`ColorMapping`] - Color mapping for light/dark modes
/// - [`ThemeVariant`] - Theme variant selection (Dark, Light, Auto)
/// - [`SyntaxThemeVariant`] - Syntax highlighting theme variant (Dark, Light)
/// - [`SyntaxHighlighter`] - Syntax highlighting for code blocks
/// - [`palettes`] - Predefined color palettes for common themes
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{
///     ColorPalette, MarkdownTheme, ThemeVariant, palettes,
/// };
///
/// // Use a predefined dark palette
/// let palette = palettes::dark_default();
///
/// // Get a color by name
/// let blue = palette.get_or_default("blue");
/// ```
pub mod palettes;

/// Theme variant selection enum.
///
/// The [`ThemeVariant`] enum allows users to explicitly select dark or light
/// theme, or let the system auto-detect based on terminal settings.

/// Theme variant selection.
///
/// Controls which color scheme to use for rendering. The `Auto` variant
/// will attempt to detect the terminal's color scheme, falling back to
/// dark mode if detection is not available.
///
/// # Variants
///
/// * `Dark` - Use dark mode colors (light text on dark background)
/// * `Light` - Use light mode colors (dark text on light background)
/// * `Auto` - Detect from terminal settings (requires `termenv` feature)
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ThemeVariant, get_effective_theme_variant};
///
/// // Explicitly use dark mode
/// let variant = ThemeVariant::Dark;
///
/// // Auto-detect (falls back to dark if detection unavailable)
/// let auto_variant = ThemeVariant::Auto;
/// let effective = get_effective_theme_variant(auto_variant);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    /// Use dark mode colors (light text on dark background).
    #[default]
    Dark,

    /// Use light mode colors (dark text on light background).
    Light,

    /// Detect from terminal settings.
    ///
    /// When the `termenv` feature is enabled, this will attempt to detect
    /// the terminal's color scheme. Without the feature, defaults to `Dark`.
    Auto,
}

/// Theme variant for syntax highlighting.

/// Theme variant for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyntaxThemeVariant {
    /// Dark theme variant (default).
    #[default]
    Dark,
    /// Light theme variant.
    Light,
}

/// Method to get the effective color for a color scheme.
use ratatui::style::Color;

impl ColorMapping {
    /// Get color for the specified color scheme.
    ///
    /// Resolves the appropriate color name (dark or light) based on the
    /// `is_dark` parameter, then looks up that color in the provided palette.
    ///
    /// If the preferred variant (dark/light) is not set, falls back to the other variant.
    /// If neither variant is set, returns `Color::White`.
    ///
    /// # Arguments
    ///
    /// * `palette` - The [`ColorPalette`] to look up color names in
    /// * `is_dark` - Whether to use dark mode colors (`true`) or light mode colors (`false`)
    ///
    /// # Returns
    ///
    /// The resolved [`Color`] for the current color scheme.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::{ColorMapping, palettes};
    ///
    /// let mapping = ColorMapping {
    ///     dark: Some("blue".to_string()),
    ///     light: Some("oceanBlue".to_string()),
    /// };
    ///
    /// let palette = palettes::dark_default();
    ///
    /// // Get dark mode color
    /// let dark_color = mapping.get_color(&palette, true);
    ///
    /// // Get light mode color
    /// let light_color = mapping.get_color(&palette, false);
    /// ```
    pub fn get_color(&self, palette: &ColorPalette, is_dark: bool) -> Color {
        let key = if is_dark {
            self.dark.as_ref().or(self.light.as_ref())
        } else {
            self.light.as_ref().or(self.dark.as_ref())
        };
        key.map(|s| palette.get_or_default(s))
            .unwrap_or(Color::White)
    }
}

/// Color mapping for light/dark mode support.
///
/// The [`ColorMapping`] struct provides a way to define different colors for
/// light and dark color schemes, allowing themes to adapt to the user's
/// terminal or application theme.

#[cfg(feature = "markdown")]
use serde::Deserialize;

/// Color mapping for light/dark modes.
///
/// This struct holds color names (not actual colors) that map to entries
/// in a [`ColorPalette`]. When resolving the actual color, the appropriate
/// variant (dark or light) is selected based on the current color scheme.
///
/// # Fields
///
/// * `dark` - Color name to use in dark mode
/// * `light` - Color name to use in light mode
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ColorMapping, ColorPalette, palettes};
///
/// // ColorMapping references color names, not actual RGB values
/// let mapping = ColorMapping {
///     dark: Some("blue".to_string()),
///     light: Some("oceanBlue".to_string()),
/// };
///
/// let palette = palettes::dark_default();
/// let color = mapping.get_color(&palette, true); // true = dark mode
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ColorMapping {
    /// Color name to use in dark mode.
    #[serde(default)]
    pub dark: Option<String>,

    /// Color name to use in light mode.
    #[serde(default)]
    pub light: Option<String>,
}

/// Constructor for creating a new empty [`ColorPalette`].
use std::collections::HashMap;

impl ColorPalette {
    /// Create a new empty palette.
    ///
    /// # Returns
    ///
    /// A new [`ColorPalette`] with no colors defined.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    ///
    /// let palette = ColorPalette::new();
    /// assert!(palette.get("any_color").is_none());
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// Method to get a color by name from the palette.

impl ColorPalette {
    /// Get a color by name, with fallback.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Color)` if the color exists in the palette, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let mut palette = ColorPalette::new();
    /// palette.add_color("blue", Color::Rgb(97, 175, 239));
    ///
    /// assert!(palette.get("blue").is_some());
    /// assert!(palette.get("nonexistent").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<Color> {
        self.0.get(name).copied()
    }
}

/// Method to get a color by name with a default fallback.

impl ColorPalette {
    /// Get a color by name with default fallback.
    ///
    /// If the color is not found in the palette, returns `Color::White` as the default.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the color to retrieve
    ///
    /// # Returns
    ///
    /// The [`Color`] associated with the name, or `Color::White` if not found.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let palette = ColorPalette::new();
    /// // Returns Color::White since "nonexistent" is not in the palette
    /// let color = palette.get_or_default("nonexistent");
    /// ```
    pub fn get_or_default(&self, name: &str) -> Color {
        self.get(name).unwrap_or(Color::White)
    }
}

/// Method to add a color to the palette.

impl ColorPalette {
    /// Add a color to the palette.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the color (e.g., "primary", "error")
    /// * `color` - The ratatui [`Color`] value
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
    /// use ratatui::style::Color;
    ///
    /// let mut palette = ColorPalette::new();
    /// palette.add_color("blue", Color::Rgb(97, 175, 239));
    /// ```
    pub fn add_color(&mut self, name: &str, color: Color) {
        self.0.insert(name.to_string(), color);
    }
}

/// Color palette for mapping named colors to RGB values.
///
/// The [`ColorPalette`] struct provides a way to define and lookup colors by name,
/// which is useful for theming systems where colors are referenced by semantic names
/// rather than direct RGB values.

/// Color palette mapping named colors to RGB values.
///
/// This struct wraps a `HashMap` to provide named color lookups. Colors can be
/// added with semantic names (like "primary", "error", "success") and retrieved
/// by those names, with optional fallback to default colors.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::ColorPalette;
/// use ratatui::style::Color;
///
/// let mut palette = ColorPalette::new();
/// palette.add_color("primary", Color::Rgb(97, 175, 239));
///
/// let color = palette.get_or_default("primary");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ColorPalette(pub(crate) HashMap<String, Color>);

/// Default trait implementation for MarkdownStyle.

impl Default for MarkdownStyle {
    fn default() -> Self {
        Self {
            // Heading icons (matching render-markdown.nvim defaults)
            h1_icon: "# ",
            h2_icon: "## ",
            h3_icon: "### ",
            h4_icon: "#### ",
            h5_icon: "##### ",
            h6_icon: "###### ",

            // Heading colors matching render-markdown.nvim with typical colorscheme
            // H1 = Deep blue
            h1_fg: Color::Rgb(255, 255, 255),
            h1_bg: Color::Rgb(30, 58, 138),
            // H2 = Cyan/teal
            h2_fg: Color::Rgb(255, 255, 255),
            h2_bg: Color::Rgb(8, 145, 178),
            // H3 = Purple/magenta
            h3_fg: Color::Rgb(255, 255, 255),
            h3_bg: Color::Rgb(168, 85, 247),
            // H4 = Orange/amber
            h4_fg: Color::Rgb(255, 255, 255),
            h4_bg: Color::Rgb(217, 119, 6),
            // H5 = Gray
            h5_fg: Color::Rgb(255, 255, 255),
            h5_bg: Color::Rgb(107, 114, 128),
            // H6 = Dark gray
            h6_fg: Color::Rgb(255, 255, 255),
            h6_bg: Color::Rgb(75, 85, 99),

            // Bullet points
            bullet_l1: "* ",
            bullet_l2: "- ",
            bullet_l3: "+ ",

            // Code blocks
            code_block_border: true,
            code_block_bg: Color::Rgb(40, 42, 54),
            inline_code_bg: Color::Rgb(68, 71, 90),
            inline_code_fg: Color::Rgb(139, 233, 253),

            // Block quotes
            quote_icon: "| ",
            quote_fg: Color::Rgb(139, 233, 253),
            quote_bg: Color::Rgb(40, 42, 54),

            // Callouts
            callout_note_icon: "! ",
            callout_tip_icon: "! ",
            callout_warning_icon: "! ",
            callout_caution_icon: "! ",

            // Text colors
            text_fg: Color::Rgb(220, 220, 220),
            text_bg: Color::Reset,
            link_fg: Color::Rgb(97, 175, 239),
            emph_fg: Color::Rgb(198, 120, 221),
            strong_fg: Color::Rgb(191, 97, 106),
            hr_fg: Color::Rgb(144, 145, 156),
            table_border_fg: Color::Rgb(96, 98, 109),
        }
    }
}

/// Configuration for markdown rendering styles.
///
/// This module provides the `MarkdownStyle` struct which configures
/// the visual appearance of markdown elements including headings,
/// code blocks, lists, and more.

/// Configuration for markdown rendering styles.
///
/// This struct allows customization of all visual aspects of markdown
/// rendering, including heading icons and colors, bullet point styles,
/// code block appearance, and text colors.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::MarkdownStyle;
///
/// // Use default styling
/// let style = MarkdownStyle::default();
/// ```
#[derive(Clone)]
pub struct MarkdownStyle {
    /// Icon displayed before H1 headings (e.g., "* ").
    pub h1_icon: &'static str,
    /// Icon displayed before H2 headings (e.g., "* ").
    pub h2_icon: &'static str,
    /// Icon displayed before H3 headings (e.g., "* ").
    pub h3_icon: &'static str,
    /// Icon displayed before H4 headings (e.g., "* ").
    pub h4_icon: &'static str,
    /// Icon displayed before H5 headings (e.g., "* ").
    pub h5_icon: &'static str,
    /// Icon displayed before H6 headings (e.g., "* ").
    pub h6_icon: &'static str,

    /// Foreground color for H1 headings.
    pub h1_fg: Color,
    /// Background color for H1 headings.
    pub h1_bg: Color,
    /// Foreground color for H2 headings.
    pub h2_fg: Color,
    /// Background color for H2 headings.
    pub h2_bg: Color,
    /// Foreground color for H3 headings.
    pub h3_fg: Color,
    /// Background color for H3 headings.
    pub h3_bg: Color,
    /// Foreground color for H4 headings.
    pub h4_fg: Color,
    /// Background color for H4 headings.
    pub h4_bg: Color,
    /// Foreground color for H5 headings.
    pub h5_fg: Color,
    /// Background color for H5 headings.
    pub h5_bg: Color,
    /// Foreground color for H6 headings.
    pub h6_fg: Color,
    /// Background color for H6 headings.
    pub h6_bg: Color,

    /// Bullet character for level 1 list items (e.g., "* ").
    pub bullet_l1: &'static str,
    /// Bullet character for level 2 list items (e.g., "* ").
    pub bullet_l2: &'static str,
    /// Bullet character for level 3 list items (e.g., "* ").
    pub bullet_l3: &'static str,

    /// Whether to show a border around code blocks.
    pub code_block_border: bool,
    /// Background color for code blocks.
    pub code_block_bg: Color,
    /// Background color for inline code spans.
    pub inline_code_bg: Color,
    /// Foreground color for inline code spans.
    pub inline_code_fg: Color,

    /// Icon displayed at the start of blockquotes (e.g., "| ").
    pub quote_icon: &'static str,
    /// Foreground color for blockquote text.
    pub quote_fg: Color,
    /// Background color for blockquotes.
    pub quote_bg: Color,

    /// Icon for note callouts (e.g., "! ").
    pub callout_note_icon: &'static str,
    /// Icon for tip callouts (e.g., "! ").
    pub callout_tip_icon: &'static str,
    /// Icon for warning callouts (e.g., "! ").
    pub callout_warning_icon: &'static str,
    /// Icon for caution callouts (e.g., "! ").
    pub callout_caution_icon: &'static str,

    /// Default foreground color for body text.
    pub text_fg: Color,
    /// Default background color for body text.
    pub text_bg: Color,
    /// Foreground color for hyperlinks.
    pub link_fg: Color,
    /// Foreground color for emphasized (italic) text.
    pub emph_fg: Color,
    /// Foreground color for strong (bold) text.
    pub strong_fg: Color,
    /// Foreground color for horizontal rules.
    pub hr_fg: Color,
    /// Foreground color for table borders.
    pub table_border_fg: Color,
}

/// Constructor functions for [`MarkdownTheme`].

/// Methods for [`MarkdownTheme`].

/// Markdown theme configuration struct.
///
/// The [`MarkdownTheme`] struct defines the color scheme for different markdown
/// elements, supporting both light and dark mode variants through [`ColorMapping`].

#[cfg(feature = "markdown")]

/// Markdown theme configuration.
///
/// This struct holds color mappings for various markdown elements. Each field
/// is optional, allowing themes to only override specific elements while
/// inheriting defaults for others.
///
/// # Fields
///
/// * `name` - Optional theme name for identification
/// * `markdown_text` - Color for regular text
/// * `markdown_heading` - Color for headings (h1-h6)
/// * `markdown_code` - Color for inline code and code blocks
/// * `markdown_block_quote` - Color for block quotes
/// * `markdown_emph` - Color for emphasized (italic) text
/// * `markdown_strong` - Color for strong (bold) text
/// * `markdown_link` - Color for links
/// * `markdown_hr` - Color for horizontal rules
/// * `markdown_table` - Color for tables
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{MarkdownTheme, load_theme_from_json};
///
/// let json = r#"{
///     "name": "my-theme",
///     "markdown_heading": { "dark": "blue", "light": "oceanBlue" }
/// }"#;
///
/// let theme = load_theme_from_json(json).unwrap();
/// assert_eq!(theme.name, Some("my-theme".to_string()));
/// ```
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MarkdownTheme {
    /// Optional theme name for identification.
    #[serde(default)]
    pub name: Option<String>,

    /// Color for regular text.
    #[serde(default)]
    pub markdown_text: Option<ColorMapping>,

    /// Color for headings (h1-h6).
    #[serde(default)]
    pub markdown_heading: Option<ColorMapping>,

    /// Color for inline code and code blocks.
    #[serde(default)]
    pub markdown_code: Option<ColorMapping>,

    /// Color for block quotes.
    #[serde(default)]
    pub markdown_block_quote: Option<ColorMapping>,

    /// Color for emphasized (italic) text.
    #[serde(default)]
    pub markdown_emph: Option<ColorMapping>,

    /// Color for strong (bold) text.
    #[serde(default)]
    pub markdown_strong: Option<ColorMapping>,

    /// Color for links.
    #[serde(default)]
    pub markdown_link: Option<ColorMapping>,

    /// Color for horizontal rules.
    #[serde(default)]
    pub markdown_hr: Option<ColorMapping>,

    /// Color for tables.
    #[serde(default)]
    pub markdown_table: Option<ColorMapping>,
}

/// Default constructor for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default dark theme.
    pub fn new() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Dark,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter (no-op when markdown feature is disabled).
    pub fn new() -> Self {
        Self
    }
}

/// Custom theme constructor for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with custom theme.
    pub fn with_custom_theme(theme: syntect::highlighting::Theme) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_variant = if theme.name.as_deref().unwrap_or("").contains("light") {
            SyntaxThemeVariant::Light
        } else {
            SyntaxThemeVariant::Dark
        };

        Self {
            syntax_set,
            theme,
            theme_variant,
        }
    }
}

/// Dark theme constructor for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with dark theme.
    pub fn with_dark_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Dark,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with dark theme (no-op when markdown feature is disabled).
    pub fn with_dark_theme() -> Self {
        Self
    }
}

/// Light theme constructor for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with GitHub Light theme.
    pub fn with_light_theme() -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());

        Self {
            syntax_set,
            theme,
            theme_variant: SyntaxThemeVariant::Light,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with light theme (no-op when markdown feature is disabled).
    pub fn with_light_theme() -> Self {
        Self
    }
}

/// Named theme constructor for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with a specific theme name.
    ///
    /// # Arguments
    ///
    /// * `theme_name` - Name of the theme (e.g., "base16-ocean.dark", "github-dark", "github-light")
    pub fn with_named_theme(theme_name: &str) -> Self {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(theme_name)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to dark theme
                theme_set.themes["base16-ocean.dark"].clone()
            });

        let theme_variant = if theme_name.contains("light") {
            SyntaxThemeVariant::Light
        } else {
            SyntaxThemeVariant::Dark
        };

        Self {
            syntax_set,
            theme,
            theme_variant,
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Create a new syntax highlighter with a named theme (no-op when markdown feature is disabled).
    pub fn with_named_theme(_theme_name: &str) -> Self {
        Self
    }
}

/// Find syntax method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Find a syntax definition for given language identifier.
    pub(crate) fn find_syntax(&self, language: &str) -> Option<syntect::parsing::SyntaxReference> {
        if language.is_empty() {
            return None;
        }

        self.syntax_set
            .find_syntax_by_token(language)
            .or_else(|| self.syntax_set.find_syntax_by_name(language))
            .or_else(|| self.syntax_set.find_syntax_by_extension(language))
            .cloned()
    }
}

/// Highlight method for SyntaxHighlighter.
use ratatui::text::{Line, Text};

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Highlight code content for a given language.
    ///
    /// # Arguments
    ///
    /// * `content` - The code content to highlight
    /// * `language` - The language identifier (e.g., "rust", "python", "javascript")
    ///
    /// # Returns
    ///
    /// Syntax highlighted text, or `None` if language is not recognized
    pub fn highlight(&self, content: &str, language: &str) -> Option<Text<'static>> {
        let syntax = self.find_syntax(language)?;

        let mut highlighter = syntect::easy::HighlightLines::new(&syntax, &self.theme);

        let mut lines = Vec::new();

        for line in content.lines() {
            if let Ok(highlighted) = highlighter.highlight_line(line, &self.syntax_set) {
                let spans: Vec<ratatui::text::Span<'static>> = highlighted
                    .into_iter()
                    .map(|(style, text)| {
                        let ratatui_style = syntect_tui::translate_style(style)
                            .unwrap_or_else(|_| ratatui::style::Style::default());
                        ratatui::text::Span::styled(text.to_string(), ratatui_style)
                    })
                    .collect();

                lines.push(Line::from(spans));
            } else {
                lines.push(Line::from(line.to_string()));
            }
        }

        Some(Text::from(lines))
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Highlight code content (always returns None when markdown feature is disabled).
    pub fn highlight(&self, _content: &str, _language: &str) -> Option<Text<'static>> {
        None
    }
}

/// Highlight with line numbers method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Highlight multiple lines of code with line numbers.
    ///
    /// # Arguments
    ///
    /// * `content` - The code content to highlight
    /// * `language` - The language identifier
    /// * `start_line` - Starting line number for display
    ///
    /// # Returns
    ///
    /// Syntax highlighted text with line numbers
    pub fn highlight_with_line_numbers(
        &self,
        content: &str,
        language: &str,
        start_line: usize,
    ) -> Option<Text<'static>> {
        let syntax = self.find_syntax(language)?;

        let mut highlighter = syntect::easy::HighlightLines::new(&syntax, &self.theme);

        let mut lines = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let line_num = start_line + i;
            let line_num_str = format!("{:4} ", line_num);

            if let Ok(highlighted) = highlighter.highlight_line(line, &self.syntax_set) {
                let num_style =
                    ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(100, 100, 100));
                let num_span = ratatui::text::Span::styled(line_num_str, num_style);

                let content_spans: Vec<ratatui::text::Span<'static>> = highlighted
                    .into_iter()
                    .map(|(style, text)| {
                        let ratatui_style = syntect_tui::translate_style(style)
                            .unwrap_or_else(|_| ratatui::style::Style::default());
                        ratatui::text::Span::styled(text.to_string(), ratatui_style)
                    })
                    .collect();

                let mut all_spans = vec![num_span];
                all_spans.extend(content_spans);
                lines.push(Line::from(all_spans));
            } else {
                let num_style =
                    ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(100, 100, 100));
                let span =
                    ratatui::text::Span::styled(format!("{}{}", line_num_str, line), num_style);
                lines.push(Line::from(span));
            }
        }

        Some(Text::from(lines))
    }
}

/// Set dark theme method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Set the theme to dark mode.
    pub fn set_dark_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set.themes["base16-ocean.dark"].clone();
        self.theme_variant = SyntaxThemeVariant::Dark;
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Set dark theme (no-op when markdown feature is disabled).
    pub fn set_dark_theme(&mut self) {}
}

/// Set light theme method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Set the theme to light mode (GitHub Light).
    pub fn set_light_theme(&mut self) {
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        self.theme = theme_set
            .themes
            .get("github-light")
            .cloned()
            .unwrap_or_else(|| theme_set.themes["base16-ocean.light"].clone());
        self.theme_variant = SyntaxThemeVariant::Light;
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Set light theme (no-op when markdown feature is disabled).
    pub fn set_light_theme(&mut self) {}
}

/// Theme variant getter method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Get the current theme variant.
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        self.theme_variant
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Get current theme variant (always returns Dark when markdown feature is disabled).
    pub fn theme_variant(&self) -> SyntaxThemeVariant {
        SyntaxThemeVariant::Dark
    }
}

/// Toggle theme method for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl SyntaxHighlighter {
    /// Switch between light and dark themes.
    pub fn toggle_theme(&mut self) {
        match self.theme_variant {
            SyntaxThemeVariant::Dark => self.set_light_theme(),
            SyntaxThemeVariant::Light => self.set_dark_theme(),
        }
    }
}

#[cfg(not(feature = "markdown"))]
impl SyntaxHighlighter {
    /// Toggle theme (no-op when markdown feature is disabled).
    pub fn toggle_theme(&mut self) {}
}

/// Default trait implementation for SyntaxHighlighter.

#[cfg(feature = "markdown")]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "markdown"))]
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Syntax highlighting for code blocks using syntect.
///
/// This module provides syntax highlighting functionality for code blocks
/// in markdown documents using the syntect library.

/// Highlighter for code blocks using syntect.
pub struct SyntaxHighlighter {
    #[cfg(feature = "markdown")]
    pub(crate) syntax_set: syntect::parsing::SyntaxSet,
    #[cfg(feature = "markdown")]
    pub(crate) theme: syntect::highlighting::Theme,
    #[cfg(feature = "markdown")]
    pub(crate) theme_variant: SyntaxThemeVariant,
}

/// Function to resolve the effective theme variant.

/// Get the effective color scheme based on variant and terminal detection.
///
/// Resolves a [`ThemeVariant`] to its effective value. For `Dark` and `Light`
/// variants, returns them unchanged. For `Auto`, attempts to detect the
/// terminal's color scheme.
///
/// # Arguments
///
/// * `variant` - The [`ThemeVariant`] to resolve
///
/// # Returns
///
/// The effective [`ThemeVariant`] after resolution. Note that `Auto` will
/// be resolved to either `Dark` or `Light`.
///
/// # Features
///
/// When the `termenv` feature is enabled, auto-detection will use the
/// terminal environment to determine the color scheme. Without this feature,
/// `Auto` defaults to `Dark`.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ThemeVariant, get_effective_theme_variant};
///
/// // Explicit variant is returned unchanged
/// assert_eq!(
///     get_effective_theme_variant(ThemeVariant::Dark),
///     ThemeVariant::Dark
/// );
///
/// // Auto resolves to Dark or Light based on terminal
/// let effective = get_effective_theme_variant(ThemeVariant::Auto);
/// assert!(matches!(effective, ThemeVariant::Dark | ThemeVariant::Light));
/// ```
#[allow(unexpected_cfgs)]
pub fn get_effective_theme_variant(variant: ThemeVariant) -> ThemeVariant {
    match variant {
        ThemeVariant::Auto => {
            // Simple terminal detection: check for dark terminal indicators
            // This is a basic implementation that can be enhanced
            #[cfg(feature = "termenv")]
            {
                use termenv::Config;
                let config = Config::default();
                if config.profile() == Some(termenv::Profile::Dark) {
                    ThemeVariant::Dark
                } else {
                    ThemeVariant::Light
                }
            }
            #[cfg(not(feature = "termenv"))]
            {
                // Default to dark mode if termenv is not available
                ThemeVariant::Dark
            }
        }
        _ => variant,
    }
}

/// Function to load a markdown theme from JSON.

/// Load a markdown theme from JSON string.
///
/// Parses a JSON string into a [`MarkdownTheme`] struct. The JSON should
/// contain optional color mappings for markdown elements.
///
/// # Arguments
///
/// * `json` - A JSON string containing theme configuration
///
/// # Returns
///
/// A `Result` containing the parsed [`MarkdownTheme`] on success, or a
/// `serde_json::Error` if parsing fails.
///
/// # Errors
///
/// Returns an error if the JSON is malformed or contains invalid fields.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::load_theme_from_json;
///
/// let json = r#"{
///     "name": "custom-theme",
///     "markdown_heading": { "dark": "blue", "light": "oceanBlue" },
///     "markdown_code": { "dark": "green" }
/// }"#;
///
/// let theme = load_theme_from_json(json).expect("valid JSON");
/// assert_eq!(theme.name, Some("custom-theme".to_string()));
/// ```
pub fn load_theme_from_json(json: &str) -> Result<MarkdownTheme, serde_json::Error> {
    serde_json::from_str(json)
}
