use crate::services::hotkey_service::HotkeyRegistry;
use crate::services::hotkey_service::HotkeyScope;

impl HotkeyRegistry {
    /// Set the active scope for hotkey filtering.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope to set active, or `None` to clear
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::{HotkeyRegistry, HotkeyScope};
    ///
    /// let mut registry = HotkeyRegistry::new();
    /// registry.set_active_scope(Some(HotkeyScope::Tab("Markdown")));
    /// ```
    pub fn set_active_scope(&mut self, scope: Option<HotkeyScope>) {
        self.active_scope = scope;
    }
}
