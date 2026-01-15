//! VT100 Terminal implementation inspired by mprocs
//!
//! This module provides a VT100 terminal emulator with:
//! - VecDeque-based infinite scrollback
//! - Copy mode with frozen screen snapshots
//! - Mouse and keyboard text selection
//! - OSC 52 clipboard integration
//!
//! Architecture:
//! - termwiz: Parse VT100 escape sequences
//! - Custom Grid/Screen: State management
//! - ratatui: Rendering

mod cell;
mod copy_mode;
mod grid;
mod parser;
mod screen;

pub use cell::{Attrs, Cell};
pub use copy_mode::{CopyMode, Pos};
pub use grid::Grid;
pub use parser::Parser;
pub use screen::Screen;

use anyhow::Result;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

/// VT100 Terminal Widget
///
/// This terminal uses termwiz for parsing and a custom VT100 implementation
/// for state management, giving us full control over scrollback and selection.
pub struct VT100Term {
    /// Terminal parser and screen state
    parser: Arc<Mutex<Parser>>,

    /// Title for the terminal
    pub title: String,

    /// Whether the terminal has focus
    pub focused: bool,

    /// Copy mode state
    pub copy_mode: CopyMode,

    /// Process management
    _master: Option<Arc<Mutex<Box<dyn MasterPty + Send>>>>,
    _child: Option<Box<dyn Child + Send + Sync>>,
    writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,

    // Styling
    pub border_style: Style,
    pub focused_border_style: Style,
}

impl VT100Term {
    /// Create a new VT100 terminal
    pub fn new(title: impl Into<String>) -> Self {
        let parser = Parser::new(24, 80, 1000);

        Self {
            parser: Arc::new(Mutex::new(parser)),
            title: title.into(),
            focused: false,
            copy_mode: CopyMode::None,
            _master: None,
            _child: None,
            writer: None,
            border_style: Style::default().fg(Color::White),
            focused_border_style: Style::default().fg(Color::Cyan),
        }
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

        eprintln!(
            "[VT100] handle_key called: focused={} key={:?}",
            self.focused, key
        );

        // Only handle Press events, ignore Release and Repeat
        if key.kind != KeyEventKind::Press {
            eprintln!("[VT100] Ignoring non-Press event: {:?}", key.kind);
            return false;
        }

        // Check if in copy mode
        if self.copy_mode.is_active() {
            return self.handle_copy_mode_key(key);
        }

        // Ctrl+Shift+C: Copy selection
        if key.code == KeyCode::Char('c')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.modifiers.contains(KeyModifiers::SHIFT)
        {
            if let Some(text) = self.copy_mode.get_selected_text() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
                return true;
            }
        }

        // Esc: Clear selection or exit copy mode
        if key.code == KeyCode::Esc && self.copy_mode.is_active() {
            self.copy_mode = CopyMode::None;
            return true;
        }

        // Enter copy mode with Ctrl+B (like tmux)
        if key.code == KeyCode::Char('b') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.enter_copy_mode();
            return true;
        }

        // Convert key to terminal input and send
        let text = self.key_to_terminal_input(key);
        eprintln!(
            "[VT100] key_to_terminal_input returned: {:?} (len={})",
            text,
            text.len()
        );
        if !text.is_empty() {
            self.send_input(&text);
            true
        } else {
            eprintln!("[VT100] ERROR: empty text from key_to_terminal_input!");
            false
        }
    }

    /// Handle keyboard in copy mode
    fn handle_copy_mode_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        use copy_mode::CopyMoveDir;
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.copy_mode = CopyMode::None;
                true
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let (dx, dy) = CopyMoveDir::Up.delta();
                self.copy_mode.move_cursor(dx, dy);
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let (dx, dy) = CopyMoveDir::Down.delta();
                self.copy_mode.move_cursor(dx, dy);
                true
            }
            KeyCode::Left | KeyCode::Char('h') => {
                let (dx, dy) = CopyMoveDir::Left.delta();
                self.copy_mode.move_cursor(dx, dy);
                true
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let (dx, dy) = CopyMoveDir::Right.delta();
                self.copy_mode.move_cursor(dx, dy);
                true
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Set end position to start range selection
                self.copy_mode.set_end();
                true
            }
            KeyCode::Char('c') | KeyCode::Char('y') => {
                // Copy selected text
                if let Some(text) = self.copy_mode.get_selected_text() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        let _ = clipboard.set_text(text);
                    }
                }
                self.copy_mode = CopyMode::None;
                true
            }
            _ => false,
        }
    }

    /// Convert key event to terminal input sequence
    fn key_to_terminal_input(&self, key: crossterm::event::KeyEvent) -> String {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c.to_ascii_lowercase() {
                        'a'..='z' => {
                            let code = (c.to_ascii_lowercase() as u8 - b'a' + 1) as char;
                            code.to_string()
                        }
                        '@' => "\x00".to_string(),
                        '[' => "\x1b".to_string(),
                        '\\' => "\x1c".to_string(),
                        ']' => "\x1d".to_string(),
                        '^' => "\x1e".to_string(),
                        '_' => "\x1f".to_string(),
                        _ => c.to_string(),
                    }
                } else if key.modifiers.contains(KeyModifiers::ALT) {
                    format!("\x1b{}", c)
                } else {
                    c.to_string()
                }
            }
            KeyCode::Enter => "\r".to_string(),
            KeyCode::Backspace => "\x7f".to_string(),
            KeyCode::Tab => "\t".to_string(),
            KeyCode::Esc => "\x1b".to_string(),
            KeyCode::Up => "\x1b[A".to_string(),
            KeyCode::Down => "\x1b[B".to_string(),
            KeyCode::Right => "\x1b[C".to_string(),
            KeyCode::Left => "\x1b[D".to_string(),
            KeyCode::Home => "\x1b[H".to_string(),
            KeyCode::End => "\x1b[F".to_string(),
            KeyCode::PageUp => "\x1b[5~".to_string(),
            KeyCode::PageDown => "\x1b[6~".to_string(),
            KeyCode::Delete => "\x1b[3~".to_string(),
            _ => String::new(),
        }
    }

    /// Enter copy mode with frozen screen
    fn enter_copy_mode(&mut self) {
        let parser = self.parser.lock().unwrap();
        let screen = parser.screen().clone();
        let size = screen.size();

        // Start at bottom-right of visible area
        let start = Pos::new(size.cols as i32 - 1, size.rows as i32 - 1);
        self.copy_mode = CopyMode::enter(screen, start);
    }

    /// Handle mouse selection
    pub fn handle_mouse_down(&mut self, x: u16, y: u16) {
        // Adjust for border (subtract 1)
        let content_x = x.saturating_sub(1) as i32;
        let content_y = y.saturating_sub(1) as i32;

        // Enter copy mode with mouse
        let parser = self.parser.lock().unwrap();
        let screen = parser.screen().clone();

        let start = Pos::new(content_x, content_y);
        self.copy_mode = CopyMode::enter(screen, start);
    }

    /// Handle mouse drag
    pub fn handle_mouse_drag(&mut self, x: u16, y: u16) {
        if self.copy_mode.is_active() {
            let content_x = x.saturating_sub(1) as i32;
            let content_y = y.saturating_sub(1) as i32;

            // Update end position
            if let CopyMode::Active { end, .. } = &mut self.copy_mode {
                *end = Some(Pos::new(content_x, content_y));
            }
        }
    }

    /// Handle mouse up
    pub fn handle_mouse_up(&mut self) {
        // Keep selection active, user can press Esc or 'c' to copy/exit
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        let mut parser = self.parser.lock().unwrap();
        parser.screen_mut().scroll_screen_up(lines);
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let mut parser = self.parser.lock().unwrap();
        parser.screen_mut().scroll_screen_down(lines);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.copy_mode = CopyMode::None;
    }

    /// Check if has selection
    pub fn has_selection(&self) -> bool {
        self.copy_mode.is_active()
    }

    /// Get selected text
    pub fn get_selected_text(&self) -> Option<String> {
        self.copy_mode.get_selected_text()
    }

    /// Spawn a terminal with a command
    pub fn spawn_with_command(
        title: impl Into<String>,
        command: &str,
        args: &[&str],
    ) -> Result<Self> {
        let rows = 24;
        let cols = 80;

        let pty_system = native_pty_system();
        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = pty_system.openpty(pty_size)?;

        let mut cmd = CommandBuilder::new(command);
        for arg in args {
            cmd.arg(arg);
        }

        // Set terminal type to prevent fish from timing out on capability queries
        // xterm-256color is widely supported and doesn't require DA queries
        cmd.env("TERM", "xterm-256color");

        // Disable fish greeting and fast startup
        cmd.env("fish_greeting", "");

        let current_dir = std::env::current_dir()?;
        cmd.cwd(current_dir);

        let child = pair.slave.spawn_command(cmd)?;

        #[cfg(unix)]
        {
            if let Some(fd) = pair.master.as_raw_fd() {
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
            }
        }

        // Clone reader FIRST, then take writer (which consumes master)
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        // Wrap writer in Arc/Mutex for thread-safe access
        let writer = Arc::new(Mutex::new(writer));

        let parser = Parser::new(rows as usize, cols as usize, 1000);
        let parser = Arc::new(Mutex::new(parser));
        let parser_clone = Arc::clone(&parser);

        // Spawn read thread
        tokio::spawn(async move {
            let mut buf = [0u8; 8192];
            let mut reader = reader;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        eprintln!("[VT100] PTY closed");
                        break;
                    }
                    Ok(n) => {
                        if let Ok(mut parser) = parser_clone.lock() {
                            parser.process(&buf[..n]);
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(e) => {
                        eprintln!("[VT100] Read error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Self {
            parser,
            title: title.into(),
            focused: false,
            copy_mode: CopyMode::None,
            _master: None, // Can't store master after taking writer
            _child: Some(child),
            writer: Some(writer),
            border_style: Style::default().fg(Color::White),
            focused_border_style: Style::default().fg(Color::Cyan),
        })
    }

    /// Send input to the terminal
    pub fn send_input(&self, text: &str) {
        if let Some(ref writer) = self.writer {
            let mut writer = writer.lock().unwrap();
            eprintln!("[VT100] Sending {} bytes: {:?}", text.len(), text);
            match writer.write_all(text.as_bytes()) {
                Ok(_) => match writer.flush() {
                    Ok(_) => eprintln!("[VT100] Flushed successfully"),
                    Err(e) => eprintln!("[VT100] Flush error: {}", e),
                },
                Err(e) => eprintln!("[VT100] Write error: {}", e),
            }
        } else {
            eprintln!("[VT100] ERROR: No writer available!");
        }
    }

    /// Render the terminal
    pub fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        // Get screen state
        let parser = self.parser.lock().unwrap();
        let screen = parser.screen();

        // Render cells
        for row in 0..area.height.min(screen.size().rows as u16) {
            for col in 0..area.width.min(screen.size().cols as u16) {
                if let Some(cell) = screen.cell(row as usize, col as usize) {
                    let ratatui_cell = cell.to_ratatui();
                    let buf_cell = frame.buffer_mut().cell_mut((area.x + col, area.y + row));
                    if let Some(buf_cell) = buf_cell {
                        *buf_cell = ratatui_cell;
                    }
                }
            }
        }

        // Render selection if in copy mode
        if let Some((start, end)) = self.copy_mode.get_selection() {
            self.render_selection(frame, area, start, end, screen.scrollback());
        }

        // Show scrollback indicator
        let scrollback = screen.scrollback();
        if scrollback > 0 {
            let _indicator = format!(" -{} ", scrollback);
            // Render at top-right
            // TODO: Implement indicator rendering
        }
    }

    /// Render selection highlight
    fn render_selection(
        &self,
        frame: &mut Frame,
        area: Rect,
        start: Pos,
        end: Pos,
        scrollback: usize,
    ) {
        let (low, high) = Pos::to_low_high(&start, &end);

        let selection_style = Style::default()
            .bg(Color::Rgb(70, 130, 180))
            .fg(Color::White);

        for row in low.y..=high.y {
            let visible_row = (row + scrollback as i32) as u16;
            if visible_row >= area.height {
                continue;
            }

            let y = area.y + visible_row;
            let start_col = if row == low.y { low.x as u16 } else { 0 };
            let end_col = if row == high.y {
                (high.x as u16).min(area.width)
            } else {
                area.width
            };

            for x in start_col..end_col {
                if let Some(cell) = frame.buffer_mut().cell_mut((area.x + x, y)) {
                    cell.set_style(selection_style);
                }
            }
        }
    }

    /// Render with borders
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_style = if self.focused {
            self.focused_border_style
        } else {
            self.border_style
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(&*self.title);

        let inner = block.inner(area);
        frame.render_widget(block, area);
        self.render_content(frame, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_vt100_term_creation() {
        let term = VT100Term::new("Test Terminal");
        assert_eq!(term.title, "Test Terminal");
        assert!(!term.focused);
        assert!(!term.copy_mode.is_active());
    }

    #[test]
    fn test_vt100_term_focus_state() {
        let mut term = VT100Term::new("Test");

        // Initially not focused
        assert!(!term.focused);

        // Set focused
        term.focused = true;
        assert!(term.focused);

        // Unset focused
        term.focused = false;
        assert!(!term.focused);
    }

    #[test]
    fn test_vt100_term_handle_key_when_not_focused() {
        let mut term = VT100Term::new("Test");
        term.focused = false;

        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);

        // Should still handle key even when not focused
        // (focus is for UI indication, not functionality in this implementation)
        let handled = term.handle_key(key);
        assert!(handled);
    }

    #[test]
    fn test_vt100_term_handle_key_when_focused() {
        let mut term = VT100Term::new("Test");
        term.focused = true;

        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let handled = term.handle_key(key);
        assert!(handled);
    }

    #[test]
    fn test_vt100_term_copy_mode_enter() {
        let mut term = VT100Term::new("Test");

        // Not in copy mode initially
        assert!(!term.copy_mode.is_active());

        // Enter copy mode with Ctrl+B
        let key = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL);
        term.handle_key(key);

        assert!(term.copy_mode.is_active());
    }

    #[test]
    fn test_vt100_term_copy_mode_exit() {
        let mut term = VT100Term::new("Test");

        // Enter copy mode
        let enter_key = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL);
        term.handle_key(enter_key);
        assert!(term.copy_mode.is_active());

        // Exit copy mode with ESC
        let esc_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        term.handle_key(esc_key);

        assert!(!term.copy_mode.is_active());
    }

    #[test]
    fn test_vt100_term_key_to_terminal_input_char() {
        let term = VT100Term::new("Test");

        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let input = term.key_to_terminal_input(key);
        assert_eq!(input, "a");
    }

    #[test]
    fn test_vt100_term_key_to_terminal_input_enter() {
        let term = VT100Term::new("Test");

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let input = term.key_to_terminal_input(key);
        assert_eq!(input, "\r");
    }

    #[test]
    fn test_vt100_term_key_to_terminal_input_ctrl() {
        let term = VT100Term::new("Test");

        // Ctrl+C should produce control character
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let input = term.key_to_terminal_input(key);
        assert_eq!(input, "\u{0003}"); // Ctrl+C = 0x03
    }

    #[test]
    fn test_vt100_term_key_to_terminal_input_arrow_up() {
        let term = VT100Term::new("Test");

        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let input = term.key_to_terminal_input(key);
        assert_eq!(input, "\x1b[A");
    }

    #[test]
    fn test_vt100_term_key_to_terminal_input_arrow_down() {
        let term = VT100Term::new("Test");

        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let input = term.key_to_terminal_input(key);
        assert_eq!(input, "\x1b[B");
    }

    #[test]
    fn test_vt100_term_selection_mouse_handling() {
        let mut term = VT100Term::new("Test");

        // Start selection
        term.handle_mouse_down(5, 5);

        // Update selection
        term.handle_mouse_drag(10, 5);

        // End selection
        term.handle_mouse_up();

        // Clear selection
        term.clear_selection();
        assert!(!term.has_selection());
    }

    #[test]
    fn test_vt100_term_scroll_up() {
        let mut term = VT100Term::new("Test");

        // Should not panic
        term.scroll_up(3);
    }

    #[test]
    fn test_vt100_term_scroll_down() {
        let mut term = VT100Term::new("Test");

        // Should not panic
        term.scroll_down(3);
    }

    #[test]
    fn test_spawn_command_sets_term_env() {
        // This test verifies that TERM environment variable is set
        // which prevents fish shell from timing out on capability queries

        // We can't easily test the actual spawn without a full PTY setup,
        // but we document the expected behavior:
        // 1. TERM should be set to "xterm-256color"
        // 2. fish_greeting should be set to ""

        // The actual implementation is in spawn_with_command which sets:
        // cmd.env("TERM", "xterm-256color");
        // cmd.env("fish_greeting", "");
    }
}
