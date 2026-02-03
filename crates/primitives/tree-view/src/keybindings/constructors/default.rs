use crossterm::event::KeyCode;

use crate::keybindings::TreeKeyBindings;

impl Default for TreeKeyBindings {
    fn default() -> Self {
        Self {
            next: vec![KeyCode::Char('j'), KeyCode::Down],
            previous: vec![KeyCode::Char('k'), KeyCode::Up],
            expand: vec![KeyCode::Char('l'), KeyCode::Right],
            collapse: vec![KeyCode::Char('h'), KeyCode::Left],
            toggle: vec![KeyCode::Enter],
            goto_top: vec![KeyCode::Char('g')],
            goto_bottom: vec![KeyCode::Char('G')],
        }
    }
}
