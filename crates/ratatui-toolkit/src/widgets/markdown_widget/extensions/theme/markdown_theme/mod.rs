//! Markdown theme configuration struct.
//!
//! The [`MarkdownTheme`] struct defines the color scheme for different markdown
//! elements, supporting both light and dark mode variants through [`ColorMapping`].

mod constructors;
mod methods;

use crate::widgets::markdown_widget::extensions::theme::ColorMapping;

#[cfg(feature = "markdown")]
use serde::Deserialize;

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
