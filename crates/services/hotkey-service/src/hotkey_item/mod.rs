use crate::HotkeyScope;

/// A registered hotkey.
///
/// Represents a single keyboard shortcut with associated metadata
/// for display and matching against user input.
#[derive(Debug, Clone)]
pub struct Hotkey {
    /// Key combination (e.g., "q", "Ctrl+C", "Tab").
    pub key: String,
    /// Human-readable description.
    pub description: String,
    /// Scope/context where this hotkey is active.
    pub scope: HotkeyScope,
    /// Priority for conflict resolution (higher = more important).
    pub priority: u32,
}
