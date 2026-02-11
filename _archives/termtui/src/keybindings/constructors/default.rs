//! Default implementation for TermTuiKeyBindings

use crate::keybindings::TermTuiKeyBindings;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl Default for TermTuiKeyBindings {
    fn default() -> Self {
        Self {
            enter_copy_mode: KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL),
            copy_selection: KeyEvent::new(
                KeyCode::Char('C'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),

            // Copy mode defaults
            copy_exit: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            copy_exit_alt: KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            copy_move_up: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            copy_move_up_alt: KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            copy_move_down: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            copy_move_down_alt: KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            copy_move_left: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            copy_move_left_alt: KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            copy_move_right: KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            copy_move_right_alt: KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
            copy_line_start: KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
            copy_line_start_alt: KeyEvent::new(KeyCode::Home, KeyModifiers::NONE),
            copy_line_end: KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE),
            copy_line_end_alt: KeyEvent::new(KeyCode::End, KeyModifiers::NONE),
            copy_page_up: KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE),
            copy_page_up_alt: KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE),
            copy_page_down: KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            copy_page_down_alt: KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
            copy_top: KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
            copy_bottom: KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE),
            copy_word_left: KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
            copy_word_right: KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
            copy_start_selection: KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
            copy_start_selection_alt: KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            copy_and_exit: KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE),
            copy_and_exit_alt: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        }
    }
}
