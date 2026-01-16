mod constructors;
mod methods;

use crate::alac_term::key_binding::KeyBinding;

/// Configurable key bindings for AlacTerm
#[derive(Debug, Clone)]
pub struct AlacTermKeyBindings {
    /// Key binding for copying selection to clipboard (default: Ctrl+Shift+C)
    pub copy_selection: KeyBinding,
    /// Key binding for clearing selection (default: Esc)
    pub clear_selection: KeyBinding,
    /// Key binding for pasting from clipboard (default: Ctrl+Shift+V)
    pub paste: KeyBinding,
    /// Key binding for scrolling up (default: Shift+PageUp)
    pub scroll_up: KeyBinding,
    /// Key binding for scrolling down (default: Shift+PageDown)
    pub scroll_down: KeyBinding,
}
