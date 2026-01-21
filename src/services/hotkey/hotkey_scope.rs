/// Scope/context for a hotkey.
///
/// Defines the context in which a hotkey is active, allowing
/// for different hotkey sets in different parts of the application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HotkeyScope {
    /// Global hotkeys (always active).
    Global,
    /// Modal hotkeys (active when modal is shown).
    Modal(&'static str),
    /// Tab-specific hotkeys.
    Tab(&'static str),
    /// Custom scope.
    Custom(&'static str),
}
