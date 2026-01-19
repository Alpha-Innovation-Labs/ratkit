//! Default constructor for MasterLayoutKeyBindings

use crate::master_layout::keybindings::MasterLayoutKeyBindings;
use crossterm::event::{KeyCode, KeyModifiers};

impl Default for MasterLayoutKeyBindings {
    fn default() -> Self {
        Self {
            quit: vec![
                (KeyCode::Char('q'), KeyModifiers::empty()),
                (KeyCode::Char('Q'), KeyModifiers::empty()),
            ],
            clear_selection: (KeyCode::Esc, KeyModifiers::empty()),
            deselect_pane: (KeyCode::Char('a'), KeyModifiers::CONTROL),
            switch_tabs: (1..=9)
                .map(|n| {
                    (
                        KeyCode::Char(char::from_digit(n, 10).unwrap()),
                        KeyModifiers::empty(),
                    )
                })
                .collect(),
            navigate_left: (KeyCode::Char('h'), KeyModifiers::empty()),
            navigate_right: (KeyCode::Char('l'), KeyModifiers::empty()),
            navigate_up: (KeyCode::Char('k'), KeyModifiers::empty()),
            navigate_down: (KeyCode::Char('j'), KeyModifiers::empty()),
            focus_pane: (KeyCode::Enter, KeyModifiers::empty()),
            exit_focus_mode: (KeyCode::Char('a'), KeyModifiers::CONTROL),
            copy_selection: (
                KeyCode::Char('c'),
                KeyModifiers::CONTROL.union(KeyModifiers::SHIFT),
            ),
        }
    }
}
