//! Keybindings configuration for TermTui
//!
//! This module provides customizable keybindings for terminal operations.

mod constructors;
mod methods;

use crossterm::event::KeyEvent;

/// Customizable keybindings for TermTui
///
/// This struct holds all keybindings that can be customized.
/// Each field is a `KeyEvent` that specifies the key combination.
#[derive(Debug, Clone)]
pub struct TermTuiKeyBindings {
    /// Enter copy mode (default: Ctrl+X)
    pub enter_copy_mode: KeyEvent,
    /// Copy selection to clipboard (default: Ctrl+Shift+C)
    pub copy_selection: KeyEvent,

    // Copy mode keybindings
    /// Exit copy mode (default: Esc)
    pub copy_exit: KeyEvent,
    /// Alternative exit copy mode (default: q)
    pub copy_exit_alt: KeyEvent,
    /// Move cursor up in copy mode (default: k or Up)
    pub copy_move_up: KeyEvent,
    /// Alternative move up (default: Up arrow)
    pub copy_move_up_alt: KeyEvent,
    /// Move cursor down in copy mode (default: j or Down)
    pub copy_move_down: KeyEvent,
    /// Alternative move down (default: Down arrow)
    pub copy_move_down_alt: KeyEvent,
    /// Move cursor left in copy mode (default: h or Left)
    pub copy_move_left: KeyEvent,
    /// Alternative move left (default: Left arrow)
    pub copy_move_left_alt: KeyEvent,
    /// Move cursor right in copy mode (default: l or Right)
    pub copy_move_right: KeyEvent,
    /// Alternative move right (default: Right arrow)
    pub copy_move_right_alt: KeyEvent,
    /// Move to line start (default: 0 or Home)
    pub copy_line_start: KeyEvent,
    /// Alternative line start (default: Home)
    pub copy_line_start_alt: KeyEvent,
    /// Move to line end (default: $ or End)
    pub copy_line_end: KeyEvent,
    /// Alternative line end (default: End)
    pub copy_line_end_alt: KeyEvent,
    /// Page up (default: u or PageUp)
    pub copy_page_up: KeyEvent,
    /// Alternative page up (default: PageUp)
    pub copy_page_up_alt: KeyEvent,
    /// Page down (default: d or PageDown)
    pub copy_page_down: KeyEvent,
    /// Alternative page down (default: PageDown)
    pub copy_page_down_alt: KeyEvent,
    /// Go to top (default: g)
    pub copy_top: KeyEvent,
    /// Go to bottom (default: G)
    pub copy_bottom: KeyEvent,
    /// Move word left (default: b)
    pub copy_word_left: KeyEvent,
    /// Move word right (default: w)
    pub copy_word_right: KeyEvent,
    /// Start/toggle selection (default: v or Space)
    pub copy_start_selection: KeyEvent,
    /// Alternative start selection (default: Space)
    pub copy_start_selection_alt: KeyEvent,
    /// Copy and exit (default: y or Enter)
    pub copy_and_exit: KeyEvent,
    /// Alternative copy and exit (default: Enter)
    pub copy_and_exit_alt: KeyEvent,
}
