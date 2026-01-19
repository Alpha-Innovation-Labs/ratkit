//! Load saved theme from config file.

use std::fs;
use std::path::PathBuf;

use super::default_config_path::default_config_path;
use super::theme_config::ThemeConfig;

/// Loads the saved theme name from the config file.
///
/// Attempts to read and parse the theme configuration from the specified
/// or default location.
///
/// # Arguments
///
/// * `config_path` - Optional custom path. If None, uses the default config location.
///
/// # Returns
///
/// `Some(theme_name)` if a saved theme was found and successfully parsed,
/// `None` otherwise (including if the file doesn't exist).
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::services::theme::persistence::load_saved_theme;
///
/// if let Some(theme_name) = load_saved_theme(None) {
///     println!("Using saved theme: {}", theme_name);
/// } else {
///     println!("No saved theme, using default");
/// }
/// ```
#[must_use]
pub fn load_saved_theme(config_path: Option<PathBuf>) -> Option<String> {
    let path = config_path.or_else(default_config_path)?;

    let content = fs::read_to_string(&path).ok()?;
    let config: ThemeConfig = serde_json::from_str(&content).ok()?;

    Some(config.theme_name)
}
