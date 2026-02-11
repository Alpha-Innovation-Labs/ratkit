//! TreeNavigator::get_hotkey_items helper method.

use crossterm::event::KeyCode;

use crate::primitives::tree_view::tree_navigator::TreeNavigator;

impl TreeNavigator {
    /// Gets hotkey items for display in HotkeyFooter.
    ///
    /// # Returns
    ///
    /// A vector of (key_display, description) pairs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeNavigator;
    ///
    /// let navigator = TreeNavigator::new();
    /// let items = navigator.get_hotkey_items();
    /// assert!(!items.is_empty());
    /// ```
    pub fn get_hotkey_items(&self) -> Vec<(String, &'static str)> {
        let mut items = Vec::new();

        // Helper to format multiple keys
        let format_keys = |keys: &[KeyCode]| -> String {
            keys.iter()
                .map(|k| match k {
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Up => "\u{2191}".to_string(),
                    KeyCode::Down => "\u{2193}".to_string(),
                    KeyCode::Left => "\u{2190}".to_string(),
                    KeyCode::Right => "\u{2192}".to_string(),
                    KeyCode::Enter => "Enter".to_string(),
                    _ => format!("{:?}", k),
                })
                .collect::<Vec<_>>()
                .join("/")
        };

        items.push((format_keys(&self.keybindings.next), "Next"));
        items.push((format_keys(&self.keybindings.previous), "Previous"));
        items.push((format_keys(&self.keybindings.expand), "Expand"));
        items.push((format_keys(&self.keybindings.collapse), "Collapse"));
        items.push((format_keys(&self.keybindings.toggle), "Toggle"));
        items.push((format_keys(&self.keybindings.goto_top), "Top"));
        items.push((format_keys(&self.keybindings.goto_bottom), "Bottom"));

        items
    }
}
