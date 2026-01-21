use crate::services::hotkey::Hotkey;
use crate::services::hotkey::HotkeyRegistry;

impl HotkeyRegistry {
    /// Register a hotkey in the registry.
    ///
    /// # Arguments
    ///
    /// * `hotkey` - The hotkey to register
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyRegistry, HotkeyScope};
    ///
    /// let mut registry = HotkeyRegistry::new();
    /// registry.register(Hotkey::new("q", "Quit").scope(HotkeyScope::Global));
    /// ```
    pub fn register(&mut self, hotkey: Hotkey) {
        self.hotkeys.push(hotkey);
    }
}
