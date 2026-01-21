use crate::services::hotkey::HotkeyRegistry;

impl Default for HotkeyRegistry {
    fn default() -> Self {
        Self::new()
    }
}
