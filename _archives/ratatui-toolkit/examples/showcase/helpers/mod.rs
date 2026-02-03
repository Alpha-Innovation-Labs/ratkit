//! Helper functions for the showcase demo.

mod all_app_themes;
mod apply_theme;
mod dialog;
mod get_theme_name;

pub use all_app_themes::{all_app_themes, get_app_theme_display_name};
pub use apply_theme::{apply_theme_at_index, restore_original_theme};
pub use dialog::render_dialog;
pub use get_theme_name::get_theme_name;
