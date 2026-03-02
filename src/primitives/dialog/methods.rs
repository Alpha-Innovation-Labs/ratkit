use crate::primitives::dialog::types::{Dialog, DialogAction, DialogEventResult, DialogType};
use crossterm::event::KeyCode;
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
        if self.buttons.is_empty() {
            return;
        }

        if self.selected_button + 1 < self.buttons.len() {
            self.selected_button += 1;
        } else if self.wrap_button_navigation {
            self.selected_button = 0;
        }
    }

    pub fn select_previous_button(&mut self) {
        if self.buttons.is_empty() {
            return;
        }

        if self.selected_button > 0 {
            self.selected_button -= 1;
        } else if self.wrap_button_navigation {
            self.selected_button = self.buttons.len().saturating_sub(1);
        }
    }

    pub fn set_selected_button(&mut self, index: usize) {
        if self.buttons.is_empty() {
            self.selected_button = 0;
            return;
        }
        self.selected_button = index.min(self.buttons.len() - 1);
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

    pub fn blocks_background_events(&self) -> bool {
        matches!(
            self.modal_mode,
            crate::primitives::dialog::types::DialogModalMode::Blocking
        )
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> DialogEventResult {
        if self.keymap.next.contains(&key) {
            self.select_next_button();
            return DialogEventResult::consumed(Some(DialogAction::Select(self.selected_button)));
        }

        if self.keymap.previous.contains(&key) {
            self.select_previous_button();
            return DialogEventResult::consumed(Some(DialogAction::Select(self.selected_button)));
        }

        if self.keymap.confirm.contains(&key) {
            return DialogEventResult::consumed(Some(DialogAction::Confirm(self.selected_button)));
        }

        if self.keymap.cancel.contains(&key) {
            return DialogEventResult::consumed(Some(DialogAction::Cancel));
        }

        if self.keymap.close.contains(&key) {
            return DialogEventResult::consumed(Some(DialogAction::Close));
        }

        DialogEventResult::ignored()
    }

    pub fn handle_mouse_confirm(&mut self, column: u16, row: u16) -> DialogEventResult {
        if let Some(index) = self.handle_click(column, row) {
            self.set_selected_button(index);
            return DialogEventResult::consumed(Some(DialogAction::Confirm(index)));
        }
        DialogEventResult::ignored()
    }
}
