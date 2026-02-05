//! Load theme from a JSON file path.

use std::fs;
use crate::services::theme::AppTheme;
use crate::services::theme::ThemeVariant;
use std::path::Path;

use crate::services::theme::loader::load_theme_str::load_theme_str;

/// Loads an [`AppTheme`] from a JSON file path.
///
/// Reads the file contents and delegates to [`load_theme_str`] for parsing.
///
/// # Arguments
///
/// * `path` - Path to the JSON theme file
/// * `variant` - Which theme variant (dark/light) to use
///
/// # Returns
///
/// `Ok(AppTheme)` if the file can be read and parsed successfully,
/// `Err` with a description if reading or parsing fails.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The JSON is malformed
/// - Required color keys are missing
/// - Color values cannot be resolved
///
/// # Example
///
/// ```rust,ignore,no_run
/// use ratatui_toolkit::services::theme::{loader, ThemeVariant};
///
/// let theme = loader::load_theme_file("themes/gruvbox.json", ThemeVariant::Dark)
///     .expect("Failed to load theme");
/// ```
pub fn load_theme_file<P: AsRef<Path>>(path: P, variant: ThemeVariant) -> Result<AppTheme, String> {
    let path = path.as_ref();

    let contents =
        fs::read_to_string(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    load_theme_str(&contents, variant)
}
