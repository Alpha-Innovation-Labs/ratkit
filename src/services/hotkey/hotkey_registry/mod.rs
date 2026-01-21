use crate::services::hotkey::Hotkey;
use crate::services::hotkey::HotkeyScope;

/// Registry for managing hotkeys.
///
/// Stores all registered hotkeys and provides methods for
/// registration, querying, and filtering by scope.
pub struct HotkeyRegistry {
    /// All registered hotkeys.
    pub(crate) hotkeys: Vec<Hotkey>,
    /// Active scope for filtering.
    pub(crate) active_scope: Option<HotkeyScope>,
}
