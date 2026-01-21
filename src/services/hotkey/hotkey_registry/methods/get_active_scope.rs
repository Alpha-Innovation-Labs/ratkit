use crate::services::hotkey::HotkeyRegistry;
use crate::services::hotkey::HotkeyScope;

impl HotkeyRegistry {
    /// Get the active scope.
    ///
    /// # Returns
    ///
    /// `Some(scope)` if a scope is active, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::HotkeyRegistry;
    ///
    /// let registry = HotkeyRegistry::new();
    /// let scope = registry.get_active_scope();
    /// ```
    pub fn get_active_scope(&self) -> Option<HotkeyScope> {
        self.active_scope
    }
}
