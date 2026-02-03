use crate::Hotkey;
use crate::HotkeyRegistry;
use crate::HotkeyScope;
use crossterm::event::KeyCode;

impl HotkeyRegistry {
    /// Look up a hotkey by key code and scope.
    ///
    /// Searches for a registered hotkey that matches the given key code
    /// and is active in the given scope.
    ///
    /// # Arguments
    ///
    /// * `key` - The key code to look up
    /// * `scope` - The scope to search in
    ///
    /// # Returns
    ///
    /// `Some(&Hotkey)` if found, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyRegistry, HotkeyScope};
    /// use crossterm::event::KeyCode;
    ///
    /// let mut registry = HotkeyRegistry::new();
    /// registry.register(Hotkey::new("q", "Quit").scope(HotkeyScope::Global));
    ///
    /// let found = registry.lookup(&KeyCode::Char('q'), &HotkeyScope::Global);
    /// assert!(found.is_some());
    /// ```
    pub fn lookup(&self, key: &KeyCode, scope: &HotkeyScope) -> Option<&Hotkey> {
        self.hotkeys.iter().find(|hotkey| {
            let key_matches = match key {
                KeyCode::Char(c) => hotkey.key.to_lowercase() == c.to_string().to_lowercase(),
                KeyCode::Tab => hotkey.key.to_lowercase() == "tab",
                KeyCode::Enter => hotkey.key.to_lowercase() == "enter",
                KeyCode::Esc => {
                    hotkey.key.to_lowercase() == "escape" || hotkey.key.to_lowercase() == "esc"
                }
                KeyCode::Up => hotkey.key.to_lowercase() == "up",
                KeyCode::Down => hotkey.key.to_lowercase() == "down",
                KeyCode::Left => hotkey.key.to_lowercase() == "left",
                KeyCode::Right => hotkey.key.to_lowercase() == "right",
                KeyCode::Backspace => hotkey.key.to_lowercase() == "backspace",
                _ => false,
            };

            if !key_matches {
                return false;
            }

            match &hotkey.scope {
                HotkeyScope::Global => true,
                HotkeyScope::Modal(m) => matches!(scope, HotkeyScope::Modal(s) if s == m),
                HotkeyScope::Tab(t) => matches!(scope, HotkeyScope::Tab(s) if s == t),
                HotkeyScope::Custom(c) => matches!(scope, HotkeyScope::Custom(s) if s == c),
            }
        })
    }

    /// Get hotkeys filtered by scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope to filter by
    ///
    /// # Returns
    ///
    /// A slice of hotkeys that are active in the given scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyRegistry, HotkeyScope};
    ///
    /// let mut registry = HotkeyRegistry::new();
    /// registry.register(Hotkey::new("j", "Down").scope(HotkeyScope::Tab("Markdown")));
    /// registry.register(Hotkey::new("k", "Up").scope(HotkeyScope::Tab("Markdown")));
    /// registry.register(Hotkey::new("q", "Quit").scope(HotkeyScope::Global));
    ///
    /// let markdown_hotkeys = registry.get_by_scope(&HotkeyScope::Tab("Markdown"));
    /// assert_eq!(markdown_hotkeys.len(), 2);
    /// ```
    pub fn get_by_scope(&self, scope: &HotkeyScope) -> Vec<&Hotkey> {
        self.hotkeys
            .iter()
            .filter(|hotkey| match &hotkey.scope {
                HotkeyScope::Global => true,
                HotkeyScope::Modal(m) => matches!(scope, HotkeyScope::Modal(s) if s == m),
                HotkeyScope::Tab(t) => matches!(scope, HotkeyScope::Tab(s) if s == t),
                HotkeyScope::Custom(c) => matches!(scope, HotkeyScope::Custom(s) if s == c),
            })
            .collect()
    }

    /// Get all global hotkeys.
    ///
    /// # Returns
    ///
    /// A slice of all global hotkeys.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyRegistry, HotkeyScope};
    ///
    /// let mut registry = HotkeyRegistry::new();
    /// registry.register(Hotkey::new("q", "Quit").scope(HotkeyScope::Global));
    /// registry.register(Hotkey::new("j", "Down").scope(HotkeyScope::Tab("Markdown")));
    ///
    /// let global = registry.get_global();
    /// assert_eq!(global.len(), 1);
    /// ```
    pub fn get_global(&self) -> Vec<&Hotkey> {
        self.hotkeys
            .iter()
            .filter(|h| matches!(h.scope, HotkeyScope::Global))
            .collect()
    }
}
