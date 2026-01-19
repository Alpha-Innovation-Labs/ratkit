use crate::dialog::{Dialog, DialogType};
use ratatui::style::Color;

impl<'a> Dialog<'a> {
    /// Returns the border color for the dialog based on its type.
    ///
    /// If theme colors have been applied via `with_theme`, those colors are used.
    /// Otherwise, falls back to default colors for each dialog type.
    ///
    /// # Returns
    ///
    /// The color to use for the dialog border.
    pub fn get_border_color(&self) -> Color {
        match self.dialog_type {
            DialogType::Info => self.theme_info_color.unwrap_or(Color::Cyan),
            DialogType::Success => self.theme_success_color.unwrap_or(Color::Green),
            DialogType::Warning => self.theme_warning_color.unwrap_or(Color::Yellow),
            DialogType::Error => self.theme_error_color.unwrap_or(Color::Red),
            DialogType::Confirm => self.theme_confirm_color.unwrap_or(Color::Blue),
        }
    }
}
