//! Configurable keybindings for VT100Term

mod constructors;
mod methods;

use crate::vt100_term::key_binding::KeyBinding;
use crossterm::event::KeyCode;

/// Configurable keybindings for VT100Term
#[derive(Debug, Clone)]
pub struct VT100TermKeyBindings {
    /// Key to enter copy mode (default: Ctrl+B)
    pub enter_copy_mode: KeyBinding,
    /// Key to copy selection (default: Ctrl+Shift+C)
    pub copy_selection: KeyBinding,

    // Copy mode keys
    /// Exit copy mode (default: Esc, q)
    pub copy_exit: Vec<KeyCode>,
    /// Move cursor up in copy mode (default: k, Up)
    pub copy_move_up: Vec<KeyCode>,
    /// Move cursor down in copy mode (default: j, Down)
    pub copy_move_down: Vec<KeyCode>,
    /// Move cursor left in copy mode (default: h, Left)
    pub copy_move_left: Vec<KeyCode>,
    /// Move cursor right in copy mode (default: l, Right)
    pub copy_move_right: Vec<KeyCode>,
    /// Set selection anchor in copy mode (default: Space, Enter)
    pub copy_set_selection: Vec<KeyCode>,
    /// Copy selection and exit copy mode (default: c, y)
    pub copy_and_exit: Vec<KeyCode>,
}
