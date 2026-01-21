use ratatui::style::Color;

#[cfg(feature = "hotkey")]
use crate::services::hotkey::HotkeyRegistry;
use crate::widgets::hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

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

    /// Create a HotkeyFooter from a HotkeyRegistry.
    ///
    /// This constructor extracts all global hotkeys from the registry
    /// and converts them to HotkeyItems for display.
    ///
    /// # Arguments
    ///
    /// * `registry` - The HotkeyRegistry to extract hotkeys from
    ///
    /// # Returns
    ///
    /// A new HotkeyFooter with items from the registry's global hotkeys.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyRegistry, HotkeyScope};
    /// use ratatui_toolkit::hotkey_footer::HotkeyFooter;
    ///
    /// let registry = HotkeyRegistry::new();
    /// let footer = HotkeyFooter::from_registry(&registry);
    /// ```
    #[cfg(feature = "hotkey")]
    pub fn from_registry(registry: &HotkeyRegistry) -> Self {
        let items: Vec<HotkeyItem> = registry
            .get_global()
            .iter()
            .map(|h| HotkeyItem::new(&h.key, &h.description))
            .collect();

        Self::new(items)
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
