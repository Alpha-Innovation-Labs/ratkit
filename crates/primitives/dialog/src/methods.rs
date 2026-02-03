use crate::types::{Dialog, DialogType};
use ratatui::style::Color;

impl<'a> Dialog<'a> {
    pub fn get_selected_button(&self) -> usize {
        self.selected_button
    }

    pub fn get_selected_button_text(&self) -> Option<&str> {
        self.buttons.get(self.selected_button).copied()
    }

    pub fn get_border_color(&self) -> Color {
        if let Some(color) = self.border_color {
            return color;
        }

        match self.dialog_type {
            DialogType::Info => self.theme_info_color.unwrap_or(Color::Cyan),
            DialogType::Success => self.theme_success_color.unwrap_or(Color::Green),
            DialogType::Warning => self.theme_warning_color.unwrap_or(Color::Yellow),
            DialogType::Error => self.theme_error_color.unwrap_or(Color::Red),
            DialogType::Confirm => self.theme_confirm_color.unwrap_or(Color::Blue),
        }
    }

    pub fn select_next_button(&mut self) {
        if !self.buttons.is_empty() && self.selected_button < self.buttons.len() - 1 {
            self.selected_button += 1;
        }
    }

    pub fn select_previous_button(&mut self) {
        if self.selected_button > 0 {
            self.selected_button -= 1;
        }
    }

    pub fn handle_click(&self, column: u16, row: u16) -> Option<usize> {
        for (idx, area) in self.button_areas.iter().enumerate() {
            if column >= area.x
                && column < area.x + area.width
                && row >= area.y
                && row < area.y + area.height
            {
                return Some(idx);
            }
        }
        None
    }
}
