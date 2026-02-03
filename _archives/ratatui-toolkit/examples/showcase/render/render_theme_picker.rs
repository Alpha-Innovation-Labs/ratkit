//! Render the theme picker modal dialog.
//!
//! Delegates to the ThemePicker widget from ratatui-toolkit.

use ratatui::Frame;

use crate::app::App;
use ratatui_toolkit::ThemePicker;

pub fn render_theme_picker(frame: &mut Frame, app: &mut App, picker: &mut ThemePicker) {
    picker.set_current_theme(&app.current_theme);
    picker.set_saved_index(app.saved_theme_index);
    picker.render(frame, frame.area());
}

pub fn get_filtered_themes(filter: &str) -> Vec<(usize, &'static str)> {
    let themes = ratatui_toolkit::services::theme::loader::BUILTIN_THEMES;
    themes
        .iter()
        .enumerate()
        .filter(|(_, name)| {
            if filter.is_empty() {
                true
            } else {
                let filter_lower = filter.to_lowercase();
                name.to_lowercase().contains(&filter_lower)
                    || name
                        .split('-')
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().chain(chars).collect(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                        .to_lowercase()
                        .contains(&filter_lower)
            }
        })
        .map(|(i, name)| (i, *name))
        .collect()
}
