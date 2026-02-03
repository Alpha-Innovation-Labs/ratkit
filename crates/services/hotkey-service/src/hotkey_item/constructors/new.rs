use crate::Hotkey;
use crate::HotkeyScope;

impl Hotkey {
    /// Create a new hotkey.
    ///
    /// # Arguments
    ///
    /// * `key` - Key combination (e.g., "q", "Ctrl+C", "Tab")
    /// * `description` - Human-readable description
    ///
    /// # Returns
    ///
    /// A new `Hotkey` instance with default global scope and zero priority.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::hotkey::Hotkey;
    ///
    /// let hotkey = Hotkey::new("q", "Quit application");
    /// ```
    pub fn new(key: &str, description: &str) -> Self {
        Self {
            key: key.to_string(),
            description: description.to_string(),
            scope: HotkeyScope::Global,
            priority: 0,
        }
    }

    /// Set the scope for this hotkey.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope to set
    ///
    /// # Returns
    ///
    /// The hotkey with the updated scope.
    pub fn scope(mut self, scope: HotkeyScope) -> Self {
        self.scope = scope;
        self
    }

    /// Set the priority for this hotkey.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority value (higher = more important)
    ///
    /// # Returns
    ///
    /// The hotkey with the updated priority.
    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}
