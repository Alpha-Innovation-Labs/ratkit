//! Theme persistence for saving and loading user theme preferences.
//!
//! This module provides functions to save and load the user's selected theme
//! to a configuration file, allowing the preference to persist across sessions.
//!
//! # Default Config Location
//!
//! By default, the config is stored at:
//! - Linux/macOS: `~/.config/ratatui-toolkit/theme.json`
//! - Windows: `%APPDATA%\ratatui-toolkit\theme.json`
//!
//! # Example
//!
//! ```rust,ignore,no_run
//! use ratatui_toolkit::services::theme::persistence::{save_theme, load_saved_theme};
//!
//! // Save the current theme
//! save_theme("ayu", None).ok();
//!
//! // Load the saved theme on startup
//! if let Some(theme_name) = load_saved_theme(None) {
//!     println!("Loaded theme: {}", theme_name);
//! }
//! ```

mod clear_saved_theme;
mod default_config_dir;
mod default_config_path;
mod load_saved_theme;
mod save_theme;
mod theme_config;

// Re-export public API
pub use clear_saved_theme::clear_saved_theme;
pub use default_config_dir::default_config_dir;
pub use default_config_path::default_config_path;
pub use load_saved_theme::load_saved_theme;
pub use save_theme::save_theme;
pub use theme_config::ThemeConfig;

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_theme() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("theme.json");

        save_theme("dracula", Some(path.clone())).unwrap();

        let loaded = load_saved_theme(Some(path.clone()));
        assert_eq!(loaded, Some("dracula".to_string()));
    }

    #[test]
    fn test_load_nonexistent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        let loaded = load_saved_theme(Some(path));
        assert_eq!(loaded, None);
    }

    #[test]
    fn test_clear_theme() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("theme.json");

        save_theme("ayu", Some(path.clone())).unwrap();
        assert!(path.exists());

        clear_saved_theme(Some(path.clone())).unwrap();
        assert!(!path.exists());
    }
}
