//! Display string conversion for key events

use crate::primitives::termtui::keybindings::TermTuiKeyBindings;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl TermTuiKeyBindings {
    /// Convert a KeyEvent to a display string for showing in UI
    pub fn key_to_display_string(key: &KeyEvent) -> String {
        let mut parts = Vec::new();

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("^");
        }
        if key.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt+");
        }
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("\u{21e7}"); // Unicode shift symbol
        }

        let key_str = match key.code {
            KeyCode::Char(c) => c.to_uppercase().to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Backspace => "Bksp".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Up => "\u{2191}".to_string(),
            KeyCode::Down => "\u{2193}".to_string(),
            KeyCode::Left => "\u{2190}".to_string(),
            KeyCode::Right => "\u{2192}".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PgUp".to_string(),
            KeyCode::PageDown => "PgDn".to_string(),
            KeyCode::Delete => "Del".to_string(),
            KeyCode::Insert => "Ins".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            _ => "?".to_string(),
        };

        parts.push(&key_str);
        parts.concat()
    }
}
