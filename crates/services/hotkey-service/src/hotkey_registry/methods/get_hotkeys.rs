use crate::Hotkey;
use crate::HotkeyRegistry;

impl HotkeyRegistry {
    /// Get all registered hotkeys.
    ///
    /// # Returns
    ///
    /// A slice of all registered hotkeys.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::HotkeyRegistry;
    ///
    /// let registry = HotkeyRegistry::new();
    /// let all = registry.get_hotkeys();
    /// ```
    pub fn get_hotkeys(&self) -> &[Hotkey] {
        &self.hotkeys
    }
}
