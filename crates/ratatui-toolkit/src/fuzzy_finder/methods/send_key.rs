use anyhow::Result;
use crossterm::event::KeyEvent;
use std::io::Write;

use crate::fuzzy_finder::FuzzyFinder;

impl FuzzyFinder {
    pub fn send_key(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(ref terminal) = self.terminal {
            let mut writer = terminal.writer.lock().unwrap();
            let input = key_to_ansi(key)?;
            writer.write_all(input.as_bytes())?;
            writer.flush()?;
        }
        Ok(())
    }
}

fn key_to_ansi(key: KeyEvent) -> Result<String> {
    use crossterm::event::{KeyCode, KeyModifiers};

    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                let code = (c.to_ascii_lowercase() as u8 - b'a' + 1) as char;
                Ok(format!("{}", code))
            } else if key.modifiers.contains(KeyModifiers::ALT) {
                Ok(format!("\x1b{}", c))
            } else {
                Ok(c.to_string())
            }
        }
        KeyCode::Enter => Ok("\r".to_string()),
        KeyCode::Backspace => Ok("\x7f".to_string()),
        KeyCode::Tab => Ok("\t".to_string()),
        KeyCode::Esc => Ok("\x1b".to_string()),
        KeyCode::Up => Ok("\x1b[A".to_string()),
        KeyCode::Down => Ok("\x1b[B".to_string()),
        KeyCode::Right => Ok("\x1b[C".to_string()),
        KeyCode::Left => Ok("\x1b[D".to_string()),
        KeyCode::Home => Ok("\x1b[H".to_string()),
        KeyCode::End => Ok("\x1b[F".to_string()),
        KeyCode::PageUp => Ok("\x1b[5~".to_string()),
        KeyCode::PageDown => Ok("\x1b[6~".to_string()),
        KeyCode::Delete => Ok("\x1b[3~".to_string()),
        _ => Ok(String::new()),
    }
}
