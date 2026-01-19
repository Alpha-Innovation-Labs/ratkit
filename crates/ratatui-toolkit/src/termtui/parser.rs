//! Terminal parser - bridge between termwiz and Screen

use crate::termtui::screen::Screen;
use crate::termtui::size::Size;
use std::io::Write;
use termwiz::escape::parser::Parser as TermwizParser;

/// Terminal parser that processes VT100 escape sequences
///
/// Uses termwiz for parsing and delegates state management to Screen
pub struct Parser {
    /// The termwiz parser
    parser: TermwizParser,
    /// The terminal screen state
    screen: Screen,
}

impl Parser {
    /// Create a new parser
    pub fn new(rows: usize, cols: usize, scrollback: usize) -> Self {
        Self {
            parser: TermwizParser::new(),
            screen: Screen::new(rows, cols, scrollback),
        }
    }

    /// Process bytes and update terminal state
    pub fn process(&mut self, bytes: &[u8]) {
        self.parser.parse(bytes, |action| {
            self.screen.handle_action(action);
        });
    }

    /// Get the screen state
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Get mutable screen state
    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    /// Resize the terminal
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.screen.resize(rows, cols);
    }

    /// Get screen size
    pub fn size(&self) -> Size {
        self.screen.size()
    }

    /// Get scrollback offset
    pub fn scrollback(&self) -> usize {
        self.screen.scrollback()
    }

    /// Set scrollback offset
    pub fn set_scrollback(&mut self, offset: usize) {
        self.screen.set_scrollback(offset);
    }
}

impl Write for Parser {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.process(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let parser = Parser::new(24, 80, 1000);
        assert_eq!(parser.size().rows, 24);
        assert_eq!(parser.size().cols, 80);
    }

    #[test]
    fn test_parser_process_text() {
        let mut parser = Parser::new(24, 80, 1000);
        parser.process(b"Hello");

        assert_eq!(parser.screen().cursor_pos().col, 5);
    }

    #[test]
    fn test_parser_process_escape() {
        let mut parser = Parser::new(24, 80, 1000);

        // Move cursor to row 2, col 3 (1-indexed in escape sequences)
        parser.process(b"\x1b[2;3H");

        assert_eq!(parser.screen().cursor_pos().row, 1); // 0-indexed
        assert_eq!(parser.screen().cursor_pos().col, 2); // 0-indexed
    }

    #[test]
    fn test_parser_write_trait() {
        let mut parser = Parser::new(24, 80, 1000);

        // Use Write trait
        write!(parser, "Test").unwrap();

        assert_eq!(parser.screen().cursor_pos().col, 4);
    }

    #[test]
    fn test_parser_resize() {
        let mut parser = Parser::new(24, 80, 1000);

        parser.resize(40, 120);
        assert_eq!(parser.size().rows, 40);
        assert_eq!(parser.size().cols, 120);
    }
}
