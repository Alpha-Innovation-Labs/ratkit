//! Clear saved theme preference.

use std::fs;
use std::io;
use std::path::PathBuf;

use super::default_config_path::default_config_path;

/// Clears the saved theme preference.
///
/// Removes the theme config file if it exists.
///
/// # Arguments
///
/// * `config_path` - Optional custom path. If None, uses the default config location.
///
/// # Returns
///
/// `Ok(())` on success, or an error if the file couldn't be removed.
///
/// # Errors
///
/// Returns an error if:
/// - The config directory cannot be determined (when using default path)
/// - The file exists but cannot be removed
pub fn clear_saved_theme(config_path: Option<PathBuf>) -> io::Result<()> {
    let path = config_path.or_else(default_config_path).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        )
    })?;

    if path.exists() {
        fs::remove_file(&path)?;
    }

    Ok(())
}
