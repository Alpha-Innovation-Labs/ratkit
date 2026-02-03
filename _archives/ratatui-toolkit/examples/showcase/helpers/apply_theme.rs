//! Helper function to apply theme at index.

use crate::app::App;
use crate::helpers::all_app_themes;
use ratatui_toolkit::services::theme::loader::load_builtin_theme;
use ratatui_toolkit::ThemeVariant;

pub fn apply_theme_at_index(app: &mut App, index: usize) {
    let themes = all_app_themes();
    if let Some(theme_name) = themes.get(index) {
        if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
            app.current_theme = theme;
            app.menu_bar.apply_theme(&app.current_theme);
            app.code_diff.apply_theme(&app.current_theme);
        }
    }
}

pub fn restore_original_theme(app: &mut App) {
    if let Some(original_idx) = app.theme_picker.state().original_index() {
        apply_theme_at_index(app, original_idx);
    }
}
