//! TermTui - Terminal emulator using mprocs architecture
//!
//! This module provides a terminal emulator with:
//! - termwiz for VT100 escape sequence parsing
//! - VecDeque-based infinite scrollback
//! - Copy mode with frozen screen snapshots
//! - Mouse and keyboard text selection
//!
//! Architecture (matching mprocs):
//! ```text
//! bytes → termwiz Parser → actions → Screen.handle_action() → Grid (VecDeque<Row>)
//! ```

mod attrs;
mod cell;
mod copy_mode;
mod grid;
mod keybindings;
mod parser;
mod row;
mod screen;
mod size;
mod widget;

pub use attrs::{Attrs, Color};
pub use cell::Cell;
pub use copy_mode::{CopyMode, CopyMoveDir, CopyPos};
pub use grid::{Grid, Pos};
pub use keybindings::TermTuiKeyBindings;
pub use parser::Parser;
pub use row::Row;
pub use screen::Screen;
pub use size::Size;
pub use widget::TermTuiWidget;

use anyhow::Result;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::layout::Rect;
use ratatui::style::{Color as RatatuiColor, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

/// TermTui - Terminal widget with mprocs-style architecture
///
/// Features:
/// - termwiz-based VT100 parsing
/// - VecDeque scrollback buffer
/// - Copy mode with frozen screen snapshots
/// - Mouse and keyboard selection
pub struct TermTui {
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

    /// Styling
    pub border_style: Style,
    pub focused_border_style: Style,

    /// Customizable keybindings
    pub keybindings: TermTuiKeyBindings,
}

impl TermTui {
    /// Create a new terminal
    pub fn new(title: impl Into<String>) -> Self {
        let parser = Parser::new(24, 80, 10000);

        Self {
            parser: Arc::new(Mutex::new(parser)),
            title: title.into(),
            focused: false,
            copy_mode: CopyMode::None,
            _master: None,
            _child: None,
            writer: None,
            border_style: Style::default().fg(RatatuiColor::White),
            focused_border_style: Style::default().fg(RatatuiColor::Cyan),
            keybindings: TermTuiKeyBindings::default(),
        }
    }

    /// Set custom keybindings (builder pattern)
    pub fn with_keybindings(mut self, keybindings: TermTuiKeyBindings) -> Self {
        self.keybindings = keybindings;
        self
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

        // Set terminal type
        cmd.env("TERM", "xterm-256color");

        let current_dir = std::env::current_dir()?;
        cmd.cwd(current_dir);

        let child = pair.slave.spawn_command(cmd)?;

        // Set non-blocking mode
        #[cfg(unix)]
        {
            if let Some(fd) = pair.master.as_raw_fd() {
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
            }
        }

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let writer = Arc::new(Mutex::new(writer));

        let parser = Parser::new(rows as usize, cols as usize, 10000);
        let parser = Arc::new(Mutex::new(parser));
        let parser_clone = Arc::clone(&parser);

        // Spawn read thread (using std::thread for sync compatibility)
        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let mut reader = reader;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut parser) = parser_clone.lock() {
                            parser.process(&buf[..n]);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            parser,
            title: title.into(),
            focused: false,
            copy_mode: CopyMode::None,
            _master: None,
            _child: Some(child),
            writer: Some(writer),
            border_style: Style::default().fg(RatatuiColor::White),
            focused_border_style: Style::default().fg(RatatuiColor::Cyan),
            keybindings: TermTuiKeyBindings::default(),
        })
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        use crossterm::event::KeyEventKind;

        // Only handle Press events
        if key.kind != KeyEventKind::Press {
            return false;
        }

        // Handle copy mode keys
        if self.copy_mode.is_active() {
            return self.handle_copy_mode_key(key);
        }

        // Copy selection (configurable, default: Ctrl+Shift+C)
        if TermTuiKeyBindings::key_matches(&key, &self.keybindings.copy_selection) {
            if let Some(text) = self.copy_mode.get_selected_text() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
                return true;
            }
        }

        // Enter copy mode (configurable, default: Ctrl+X)
        if TermTuiKeyBindings::key_matches(&key, &self.keybindings.enter_copy_mode) {
            self.enter_copy_mode();
            return true;
        }

        // Convert key to terminal input and send
        let text = self.key_to_terminal_input(key);
        if !text.is_empty() {
            self.send_input(&text);
            true
        } else {
            false
        }
    }

    /// Handle keyboard in copy mode
    fn handle_copy_mode_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        let kb = &self.keybindings;

        // Exit copy mode
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_exit)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_exit_alt)
        {
            self.copy_mode = CopyMode::None;
            return true;
        }

        // Navigation - Up
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_move_up)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_move_up_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::Up);
            return true;
        }

        // Navigation - Down
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_move_down)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_move_down_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::Down);
            return true;
        }

        // Navigation - Left
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_move_left)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_move_left_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::Left);
            return true;
        }

        // Navigation - Right
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_move_right)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_move_right_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::Right);
            return true;
        }

        // Line start
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_line_start)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_line_start_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::LineStart);
            return true;
        }

        // Line end
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_line_end)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_line_end_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::LineEnd);
            return true;
        }

        // Page up
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_page_up)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_page_up_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::PageUp);
            return true;
        }

        // Page down
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_page_down)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_page_down_alt)
        {
            self.copy_mode.move_dir(CopyMoveDir::PageDown);
            return true;
        }

        // Top
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_top) {
            self.copy_mode.move_dir(CopyMoveDir::Top);
            return true;
        }

        // Bottom
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_bottom) {
            self.copy_mode.move_dir(CopyMoveDir::Bottom);
            return true;
        }

        // Word left
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_word_left) {
            self.copy_mode.move_dir(CopyMoveDir::WordLeft);
            return true;
        }

        // Word right
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_word_right) {
            self.copy_mode.move_dir(CopyMoveDir::WordRight);
            return true;
        }

        // Start/toggle selection
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_start_selection)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_start_selection_alt)
        {
            self.copy_mode.set_anchor();
            return true;
        }

        // Copy and exit
        if TermTuiKeyBindings::key_matches(&key, &kb.copy_and_exit)
            || TermTuiKeyBindings::key_matches(&key, &kb.copy_and_exit_alt)
        {
            if let Some(text) = self.copy_mode.get_selected_text() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
            }
            self.copy_mode = CopyMode::None;
            return true;
        }

        false
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
    pub fn enter_copy_mode(&mut self) {
        let parser = self.parser.lock().unwrap();
        let screen = parser.screen().clone();
        let size = screen.size();

        // Start at bottom-right of visible area
        let start = CopyPos::new(size.cols as i32 - 1, size.rows as i32 - 1);
        self.copy_mode = CopyMode::enter(screen, start);
    }

    /// Handle mouse events
    ///
    /// This method handles all mouse interactions:
    /// - Mouse drag: automatically enters copy mode and starts selection (like mprocs)
    /// - Mouse wheel: scrolls the terminal content
    /// - In copy mode: mouse click moves cursor, mouse drag selects
    ///
    /// # Arguments
    /// * `event` - The mouse event from crossterm
    /// * `area` - The area where the terminal is rendered (for coordinate translation)
    ///
    /// # Returns
    /// `true` if the event was handled, `false` otherwise
    pub fn handle_mouse(&mut self, event: crossterm::event::MouseEvent, area: Rect) -> bool {
        use crossterm::event::{MouseButton, MouseEventKind};

        // Translate coordinates relative to the content area (inside borders)
        let content_x = event.column.saturating_sub(area.x + 1) as i32;
        let content_y = event.row.saturating_sub(area.y + 1) as i32;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.copy_mode.is_active() {
                    // In copy mode, click moves cursor
                    if let CopyMode::Active { cursor, .. } = &mut self.copy_mode {
                        cursor.x = content_x;
                        cursor.y = content_y;
                    }
                } else {
                    // Not in copy mode - enter copy mode and position cursor
                    let parser = self.parser.lock().unwrap();
                    let screen = parser.screen().clone();
                    drop(parser);

                    let start = CopyPos::new(content_x, content_y);
                    self.copy_mode = CopyMode::enter(screen, start);
                }
                true
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if !self.copy_mode.is_active() {
                    // Auto-enter copy mode on drag (like mprocs)
                    let parser = self.parser.lock().unwrap();
                    let screen = parser.screen().clone();
                    drop(parser);

                    let start = CopyPos::new(content_x, content_y);
                    self.copy_mode = CopyMode::enter(screen, start);
                    // Set anchor immediately for selection
                    self.copy_mode.set_anchor();
                } else {
                    // Already in copy mode - set anchor if not set, then update cursor
                    self.copy_mode.set_end();

                    // Move cursor to new position
                    if let CopyMode::Active { cursor, .. } = &mut self.copy_mode {
                        cursor.x = content_x;
                        cursor.y = content_y;
                    }
                }
                true
            }
            MouseEventKind::Up(MouseButton::Left) => {
                // Keep selection active on mouse up
                true
            }
            MouseEventKind::ScrollUp => {
                self.scroll_up(3);
                true
            }
            MouseEventKind::ScrollDown => {
                self.scroll_down(3);
                true
            }
            _ => false,
        }
    }

    /// Handle mouse down (start selection)
    ///
    /// Note: Consider using `handle_mouse` instead for comprehensive mouse handling.
    #[deprecated(
        since = "0.2.0",
        note = "Use handle_mouse instead for comprehensive mouse handling"
    )]
    pub fn handle_mouse_down(&mut self, x: u16, y: u16) {
        let content_x = x.saturating_sub(1) as i32;
        let content_y = y.saturating_sub(1) as i32;

        let parser = self.parser.lock().unwrap();
        let screen = parser.screen().clone();

        let start = CopyPos::new(content_x, content_y);
        self.copy_mode = CopyMode::enter(screen, start);
    }

    /// Handle mouse drag (update selection)
    ///
    /// Note: Consider using `handle_mouse` instead for comprehensive mouse handling.
    #[deprecated(
        since = "0.2.0",
        note = "Use handle_mouse instead for comprehensive mouse handling"
    )]
    pub fn handle_mouse_drag(&mut self, x: u16, y: u16) {
        if self.copy_mode.is_active() {
            let content_x = x.saturating_sub(1) as i32;
            let content_y = y.saturating_sub(1) as i32;

            // First set anchor if not set
            self.copy_mode.set_end();

            // Move cursor to new position
            if let CopyMode::Active { cursor, .. } = &mut self.copy_mode {
                cursor.x = content_x;
                cursor.y = content_y;
            }
        }
    }

    /// Handle mouse up
    ///
    /// Note: Consider using `handle_mouse` instead for comprehensive mouse handling.
    #[deprecated(
        since = "0.2.0",
        note = "Use handle_mouse instead for comprehensive mouse handling"
    )]
    pub fn handle_mouse_up(&mut self) {
        // Keep selection active
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

    /// Send input to the terminal
    pub fn send_input(&self, text: &str) {
        if let Some(ref writer) = self.writer {
            let mut writer = writer.lock().unwrap();
            let _ = writer.write_all(text.as_bytes());
            let _ = writer.flush();
        }
    }

    /// Resize the terminal
    pub fn resize(&mut self, rows: u16, cols: u16) {
        let mut parser = self.parser.lock().unwrap();
        parser.resize(rows as usize, cols as usize);
    }

    /// Render terminal content (without borders)
    pub fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        let parser = self.parser.lock().unwrap();
        let screen = if let Some(frozen) = self.copy_mode.frozen_screen() {
            frozen
        } else {
            parser.screen()
        };

        let widget = TermTuiWidget::new(screen)
            .scroll_offset(screen.scrollback())
            .copy_mode(&self.copy_mode);

        frame.render_widget(widget, area);
    }

    /// Render terminal with borders
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        use ratatui::layout::{Constraint, Direction, Layout};
        use ratatui::text::{Line, Span};

        let border_style = if self.focused {
            self.focused_border_style
        } else {
            self.border_style
        };

        // Split area for content and hotkey footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(self.title.as_str());

        let inner = block.inner(chunks[0]);
        frame.render_widget(block, chunks[0]);
        self.render_content(frame, inner);

        // Render hotkey footer based on mode (showing configured keybindings)
        let kb = &self.keybindings;
        let hotkeys = if self.copy_mode.is_active() {
            // Build move keys display string
            let move_keys = format!(
                "{}/{}",
                TermTuiKeyBindings::key_to_display_string(&kb.copy_move_up),
                TermTuiKeyBindings::key_to_display_string(&kb.copy_move_down)
            );
            let select_key = TermTuiKeyBindings::key_to_display_string(&kb.copy_start_selection);
            let copy_key = TermTuiKeyBindings::key_to_display_string(&kb.copy_and_exit);
            let word_keys = format!(
                "{}/{}",
                TermTuiKeyBindings::key_to_display_string(&kb.copy_word_right),
                TermTuiKeyBindings::key_to_display_string(&kb.copy_word_left)
            );
            let line_keys = format!(
                "{}/{}",
                TermTuiKeyBindings::key_to_display_string(&kb.copy_line_start),
                TermTuiKeyBindings::key_to_display_string(&kb.copy_line_end)
            );
            let top_bot_keys = format!(
                "{}/{}",
                TermTuiKeyBindings::key_to_display_string(&kb.copy_top),
                TermTuiKeyBindings::key_to_display_string(&kb.copy_bottom)
            );
            let exit_key = TermTuiKeyBindings::key_to_display_string(&kb.copy_exit);

            Line::from(vec![
                Span::styled(
                    " COPY ",
                    Style::default()
                        .fg(RatatuiColor::Black)
                        .bg(RatatuiColor::Yellow),
                ),
                Span::raw(" "),
                Span::styled(move_keys, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" move "),
                Span::styled(select_key, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" select "),
                Span::styled(copy_key, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" copy "),
                Span::styled(word_keys, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" word "),
                Span::styled(line_keys, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" line "),
                Span::styled(top_bot_keys, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" top/bot "),
                Span::styled(exit_key, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" exit"),
            ])
        } else {
            let enter_copy_key = TermTuiKeyBindings::key_to_display_string(&kb.enter_copy_mode);
            let copy_selection_key = TermTuiKeyBindings::key_to_display_string(&kb.copy_selection);

            Line::from(vec![
                Span::styled(enter_copy_key, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" copy mode "),
                Span::styled(copy_selection_key, Style::default().fg(RatatuiColor::Cyan)),
                Span::raw(" copy "),
                Span::styled("scroll", Style::default().fg(RatatuiColor::DarkGray)),
                Span::raw(" mouse wheel"),
            ])
        };

        frame.render_widget(hotkeys, chunks[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_termtui_creation() {
        let term = TermTui::new("Test Terminal");
        assert_eq!(term.title, "Test Terminal");
        assert!(!term.focused);
        assert!(!term.copy_mode.is_active());
    }

    #[test]
    fn test_termtui_focus() {
        let mut term = TermTui::new("Test");

        term.focused = true;
        assert!(term.focused);

        term.focused = false;
        assert!(!term.focused);
    }

    #[test]
    fn test_termtui_copy_mode_enter() {
        let mut term = TermTui::new("Test");

        assert!(!term.copy_mode.is_active());

        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL);
        term.handle_key(key);

        assert!(term.copy_mode.is_active());
    }

    #[test]
    fn test_termtui_copy_mode_exit() {
        let mut term = TermTui::new("Test");

        // Enter copy mode
        let enter_key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL);
        term.handle_key(enter_key);
        assert!(term.copy_mode.is_active());

        // Exit with Esc
        let esc_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        term.handle_key(esc_key);
        assert!(!term.copy_mode.is_active());
    }

    #[test]
    fn test_termtui_key_conversion() {
        let term = TermTui::new("Test");

        // Regular character
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(term.key_to_terminal_input(key), "a");

        // Enter
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(term.key_to_terminal_input(key), "\r");

        // Ctrl+C
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(term.key_to_terminal_input(key), "\x03");

        // Arrow up
        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        assert_eq!(term.key_to_terminal_input(key), "\x1b[A");
    }

    #[test]
    fn test_termtui_selection() {
        use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

        let mut term = TermTui::new("Test");
        let area = ratatui::layout::Rect::new(0, 0, 80, 24);

        // Start selection via mouse down
        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 5,
            modifiers: KeyModifiers::NONE,
        };
        term.handle_mouse(mouse_event, area);
        assert!(term.has_selection());

        // Clear selection
        term.clear_selection();
        assert!(!term.has_selection());
    }

    #[test]
    fn test_termtui_keybindings() {
        // Test default keybindings
        let kb = TermTuiKeyBindings::default();
        assert_eq!(kb.enter_copy_mode.code, KeyCode::Char('x'));
        assert!(kb.enter_copy_mode.modifiers.contains(KeyModifiers::CONTROL));

        // Test key_to_display_string
        let display = TermTuiKeyBindings::key_to_display_string(&kb.enter_copy_mode);
        assert_eq!(display, "^X");

        // Test key_matches
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL);
        assert!(TermTuiKeyBindings::key_matches(&key, &kb.enter_copy_mode));

        let wrong_key = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL);
        assert!(!TermTuiKeyBindings::key_matches(
            &wrong_key,
            &kb.enter_copy_mode
        ));
    }

    #[test]
    fn test_termtui_with_keybindings() {
        let custom_kb = TermTuiKeyBindings {
            enter_copy_mode: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            ..Default::default()
        };

        let term = TermTui::new("Test").with_keybindings(custom_kb);
        assert_eq!(term.keybindings.enter_copy_mode.code, KeyCode::Char('c'));
    }

    #[test]
    fn test_mouse_scroll() {
        use crossterm::event::{MouseEvent, MouseEventKind};

        let mut term = TermTui::new("Test");
        let area = ratatui::layout::Rect::new(0, 0, 80, 24);

        // Test scroll up
        let scroll_up = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        assert!(term.handle_mouse(scroll_up, area));

        // Test scroll down
        let scroll_down = MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        assert!(term.handle_mouse(scroll_down, area));
    }
}
