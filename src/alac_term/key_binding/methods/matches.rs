use crate::alac_term::key_binding::KeyBinding;

impl KeyBinding {
    /// Check if this key binding matches a key event
    pub fn matches(&self, key: &crossterm::event::KeyEvent) -> bool {
        self.code == key.code && key.modifiers.contains(self.modifiers)
    }
}
