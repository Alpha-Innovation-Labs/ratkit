//! Default implementation for VT100TermKeyBindings

use crate::vt100_term::key_binding::KeyBinding;
use crate::vt100_term::keybindings::VT100TermKeyBindings;
use crossterm::event::KeyCode;

impl Default for VT100TermKeyBindings {
    fn default() -> Self {
        Self {
            enter_copy_mode: KeyBinding::ctrl(KeyCode::Char('b')),
            copy_selection: KeyBinding::ctrl_shift(KeyCode::Char('c')),
            copy_exit: vec![KeyCode::Esc, KeyCode::Char('q')],
            copy_move_up: vec![KeyCode::Char('k'), KeyCode::Up],
            copy_move_down: vec![KeyCode::Char('j'), KeyCode::Down],
            copy_move_left: vec![KeyCode::Char('h'), KeyCode::Left],
            copy_move_right: vec![KeyCode::Char('l'), KeyCode::Right],
            copy_set_selection: vec![KeyCode::Char(' '), KeyCode::Enter],
            copy_and_exit: vec![KeyCode::Char('c'), KeyCode::Char('y')],
        }
    }
}
