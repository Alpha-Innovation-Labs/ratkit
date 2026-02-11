//! Configuration structure for persisted theme settings.

/// Configuration structure for persisted theme settings.
///
/// This struct holds the user's theme preferences and is serialized to JSON
/// for persistence across sessions.
///
/// # Fields
///
/// * `theme_name` - The name of the selected theme (e.g., "ayu", "dracula")
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeConfig {
    /// The name of the selected theme (e.g., "ayu", "dracula").
    pub theme_name: String,
}
