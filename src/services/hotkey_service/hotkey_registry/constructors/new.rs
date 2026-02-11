use crate::services::hotkey_service::HotkeyRegistry;

impl HotkeyRegistry {
    /// Create a new hotkey registry.
    ///
    /// # Returns
    ///
    /// A new `HotkeyRegistry` instance with empty hotkey list and no active scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::HotkeyRegistry;
    ///
    /// let registry = HotkeyRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            hotkeys: Vec::new(),
            active_scope: None,
        }
    }
}
