//! Function to load a markdown theme from JSON.

use crate::markdown_widget::extensions::theme::MarkdownTheme;

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
