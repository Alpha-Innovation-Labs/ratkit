use crossterm::event::KeyCode;

impl super::super::Hotkey {
    /// Parse KeyCode from key string.
    ///
    /// # Returns
    ///
    /// `Some(KeyCode)` if the key string can be parsed, `None` otherwise.
    ///
    /// # Supported Keys
    ///
    /// - Single letters: a-z
    /// - Special keys: tab, enter, escape, esc, backspace, up, down, left, right
    ///
    /// # Note
    ///
    /// This is a basic parser that supports common keys. For more complex
    /// combinations (e.g., Ctrl+C), you'll need to extend this logic.
    pub fn parse_keycode(&self) -> Option<KeyCode> {
        match self.key.to_lowercase().as_str() {
            "q" => Some(KeyCode::Char('q')),
            "w" => Some(KeyCode::Char('w')),
            "e" => Some(KeyCode::Char('e')),
            "r" => Some(KeyCode::Char('r')),
            "t" => Some(KeyCode::Char('t')),
            "y" => Some(KeyCode::Char('y')),
            "u" => Some(KeyCode::Char('u')),
            "i" => Some(KeyCode::Char('i')),
            "o" => Some(KeyCode::Char('o')),
            "p" => Some(KeyCode::Char('p')),
            "a" => Some(KeyCode::Char('a')),
            "s" => Some(KeyCode::Char('s')),
            "d" => Some(KeyCode::Char('d')),
            "f" => Some(KeyCode::Char('f')),
            "g" => Some(KeyCode::Char('g')),
            "h" => Some(KeyCode::Char('h')),
            "j" => Some(KeyCode::Char('j')),
            "k" => Some(KeyCode::Char('k')),
            "l" => Some(KeyCode::Char('l')),
            "z" => Some(KeyCode::Char('z')),
            "x" => Some(KeyCode::Char('x')),
            "c" => Some(KeyCode::Char('c')),
            "v" => Some(KeyCode::Char('v')),
            "b" => Some(KeyCode::Char('b')),
            "n" => Some(KeyCode::Char('n')),
            "m" => Some(KeyCode::Char('m')),
            "tab" => Some(KeyCode::Tab),
            "enter" => Some(KeyCode::Enter),
            "escape" | "esc" => Some(KeyCode::Esc),
            "backspace" => Some(KeyCode::Backspace),
            "up" => Some(KeyCode::Up),
            "down" => Some(KeyCode::Down),
            "left" => Some(KeyCode::Left),
            "right" => Some(KeyCode::Right),
            _ => None,
        }
    }
}
