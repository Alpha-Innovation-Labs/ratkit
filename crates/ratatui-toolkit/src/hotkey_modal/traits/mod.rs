use ratatui::style::Color;

use crate::hotkey_modal::HotkeyModalConfig;

impl Default for HotkeyModalConfig {
    fn default() -> Self {
        Self {
            title: "Help".to_string(),
            border_color: Color::Cyan,
            width_percent: 0.6,
            height_percent: 0.6,
            footer: Some("Press any key to close".to_string()),
            title_inside: false,
        }
    }
}
