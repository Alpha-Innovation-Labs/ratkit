//! Theme picker tab handler.

use super::TabHandler;
use crate::app::App;
use crate::helpers::get_app_theme_display_name;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui_toolkit::services::theme::persistence::save_theme;
use ratatui_toolkit::{ThemePickerEvent, Toast, ToastLevel};

pub struct ThemePickerHandler;

impl TabHandler for ThemePickerHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) {
        let event = app.theme_picker.handle_key(&key.code);

        if let Some(event) = event {
            match event {
                ThemePickerEvent::Selected(theme_name) => {
                    let original_idx = app.theme_picker.saved_index();
                    app.theme_picker.hide();
                    app.theme_picker.state_mut().clear_filter();
                    app.theme_picker.state_mut().set_index(0);
                    if let Err(e) = save_theme(&theme_name, None) {
                        app.toast_manager.add(Toast::new(
                            &format!("Failed to save theme: {}", e),
                            ToastLevel::Warning,
                            None,
                        ));
                    }
                    app.toast_manager.add(Toast::new(
                        &format!("Theme: {}", get_app_theme_display_name(&theme_name)),
                        ToastLevel::Success,
                        None,
                    ));
                }
                ThemePickerEvent::Cancelled => {
                    app.theme_picker.state_mut().restore_original();
                    crate::helpers::restore_original_theme(app);
                    app.theme_picker.hide();
                    app.theme_picker.state_mut().clear_filter();
                    app.theme_picker.state_mut().set_index(0);
                }
                ThemePickerEvent::PreviewChanged(theme_name) => {
                    if let Some((original_idx, _)) = app
                        .theme_picker
                        .state()
                        .filter()
                        .is_empty()
                        .then(|| {
                            ratatui_toolkit::services::theme::loader::BUILTIN_THEMES
                                .iter()
                                .enumerate()
                                .find(|(i, name)| *name == theme_name)
                        })
                        .flatten()
                    {
                        crate::helpers::apply_theme_at_index(app, original_idx);
                    }
                }
            }
        }
    }

    fn handle_mouse(&mut self, _app: &mut App, _mouse: MouseEvent) {}

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        false
    }
}
