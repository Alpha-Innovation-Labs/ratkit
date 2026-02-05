//! Default config path function.

use std::path::PathBuf;

use super::default_config_dir::default_config_dir;

/// Gets the default path for the theme config file.
///
/// # Returns
///
/// The full path to the theme.json config file, or `None` if the config
/// directory cannot be determined.
#[must_use]
pub fn default_config_path() -> Option<PathBuf> {
    default_config_dir().map(|p| p.join("theme.json"))
}
