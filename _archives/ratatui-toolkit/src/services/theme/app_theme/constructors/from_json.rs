//! JSON constructor for [`AppTheme`].

use std::path::Path;

use crate::services::theme::app_theme::AppTheme;
use crate::services::theme::loader::{load_theme_file, load_theme_str};
use crate::services::theme::ThemeVariant;

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
    /// ```rust
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
    /// ```rust,no_run
    /// use ratatui_toolkit::services::theme::{AppTheme, ThemeVariant};
    ///
    /// let theme = AppTheme::from_json_file("themes/gruvbox.json", ThemeVariant::Dark)
    ///     .expect("Failed to load theme");
    /// ```
    pub fn from_json_file<P: AsRef<Path>>(path: P, variant: ThemeVariant) -> Result<Self, String> {
        load_theme_file(path, variant)
    }
}
