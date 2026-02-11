use crate::services::hotkey_service::HotkeyRegistry;

impl Default for HotkeyRegistry {
    fn default() -> Self {
        Self::new()
    }
}
