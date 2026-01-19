use ratatui::style::Color;

use crate::hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

impl HotkeyItem {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

impl HotkeyFooter {
    pub fn new(items: Vec<HotkeyItem>) -> Self {
        Self {
            items,
            key_color: Color::Cyan,
            description_color: Color::DarkGray,
            background_color: Color::Black,
        }
    }

    pub fn key_color(mut self, color: Color) -> Self {
        self.key_color = color;
        self
    }

    pub fn description_color(mut self, color: Color) -> Self {
        self.description_color = color;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
}

impl HotkeyFooterBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.items.push(HotkeyItem::new(key, description));
        self
    }

    pub fn add_items(mut self, items: Vec<(String, &str)>) -> Self {
        for (key, desc) in items {
            self.items.push(HotkeyItem::new(key, desc));
        }
        self
    }

    pub fn build(self) -> HotkeyFooter {
        HotkeyFooter::new(self.items)
    }
}
