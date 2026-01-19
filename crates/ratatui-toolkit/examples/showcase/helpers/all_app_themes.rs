//! Get all available application theme names.

use ratatui_toolkit::services::theme::loader::BUILTIN_THEMES;

/// Get all available application theme names.
///
/// Returns the list of builtin theme names from the toolkit's theme system.
/// Currently includes 33 themes like gruvbox, dracula, tokyonight, etc.
///
/// # Returns
///
/// A slice of static strings representing theme names.
pub fn all_app_themes() -> &'static [&'static str] {
    BUILTIN_THEMES
}

/// Get a display-friendly name for an app theme.
///
/// Converts kebab-case theme names to Title Case.
///
/// # Arguments
///
/// * `name` - The theme name in kebab-case (e.g., "one-dark")
///
/// # Returns
///
/// A formatted display name (e.g., "One Dark")
pub fn get_app_theme_display_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
