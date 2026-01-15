//! VT100 parser integration with termwiz

use super::Screen;
use std::sync::{Arc, Mutex};
use termwiz::escape::parser::Parser as TermwizParser;

/// VT100 parser
///
/// Wraps termwiz's parser and feeds actions to our custom Screen
pub struct Parser {
    /// termwiz parser (only used for parsing, not state)
    parser: Arc<Mutex<TermwizParser>>,

    /// Our custom screen state
    screen: Screen,
}

impl Parser {
    /// Create a new parser
    pub fn new(rows: usize, cols: usize, scrollback_len: usize) -> Self {
        Self {
            parser: Arc::new(Mutex::new(TermwizParser::new())),
            screen: Screen::new(rows, cols, scrollback_len),
        }
    }

    /// Get the screen
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Get mutable screen
    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    /// Process raw bytes from PTY
    pub fn process(&mut self, bytes: &[u8]) {
        let mut parser = self.parser.lock().unwrap();

        parser.parse(bytes, |action| {
            self.screen.handle_action(&action);
        });
    }

    /// Resize the terminal
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.screen.resize(rows, cols);
    }
}
