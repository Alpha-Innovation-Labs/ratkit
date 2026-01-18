//! Default config directory function.

use std::path::PathBuf;

/// Gets the default config directory for theme settings.
///
/// Returns the platform-specific configuration directory with the ratatui-toolkit
/// subdirectory appended.
///
/// # Returns
///
/// - Linux/macOS: `~/.config/ratatui-toolkit/`
/// - Windows: `%APPDATA%\ratatui-toolkit\`
///
/// Returns `None` if the config directory cannot be determined.
#[must_use]
pub fn default_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("ratatui-toolkit"))
}
