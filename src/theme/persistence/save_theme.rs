//! Save theme to config file.

use std::fs;
use std::io;
use std::path::PathBuf;

use super::default_config_path::default_config_path;
use super::theme_config::ThemeConfig;

/// Saves the selected theme to a config file.
///
/// Creates the config directory if it doesn't exist and writes the theme
/// configuration as JSON.
///
/// # Arguments
///
/// * `theme_name` - The name of the theme to save
/// * `config_path` - Optional custom path. If None, uses the default config location.
///
/// # Returns
///
/// `Ok(())` on success, or an error if the file couldn't be written.
///
/// # Errors
///
/// Returns an error if:
/// - The config directory cannot be determined (when using default path)
/// - The parent directory cannot be created
/// - The config file cannot be written
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::theme::persistence::save_theme;
///
/// // Save to default location
/// save_theme("dracula", None).expect("Failed to save theme");
///
/// // Save to custom location
/// save_theme("nord", Some("/path/to/config.json".into())).ok();
/// ```
pub fn save_theme(theme_name: &str, config_path: Option<PathBuf>) -> io::Result<()> {
    let path = config_path.or_else(default_config_path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        )
    })?;

    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config = ThemeConfig {
        theme_name: theme_name.to_string(),
    };

    let json = serde_json::to_string_pretty(&config).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("JSON error: {}", e))
    })?;

    fs::write(&path, json)?;

    Ok(())
}
