use ratatui::style::Color;

use crate::hotkey_modal::{Hotkey, HotkeyModalConfig, HotkeySection};

impl Hotkey {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

impl HotkeySection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            hotkeys: Vec::new(),
        }
    }

    pub fn with_hotkeys(mut self, hotkeys: Vec<Hotkey>) -> Self {
        self.hotkeys = hotkeys;
        self
    }

    pub fn add_hotkey(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.hotkeys.push(Hotkey::new(key, description));
        self
    }
}

impl HotkeyModalConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn with_size(mut self, width_percent: f32, height_percent: f32) -> Self {
        self.width_percent = width_percent.clamp(0.1, 1.0);
        self.height_percent = height_percent.clamp(0.1, 1.0);
        self
    }

    pub fn with_footer(mut self, footer: Option<String>) -> Self {
        self.footer = footer;
        self
    }

    pub fn with_title_inside(mut self, inside: bool) -> Self {
        self.title_inside = inside;
        self
    }
}
