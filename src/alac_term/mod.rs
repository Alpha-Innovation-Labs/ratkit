mod key_binding;
mod keybindings;
mod widget;

pub use key_binding::KeyBinding;
pub use keybindings::AlacTermKeyBindings;

use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::vte::ansi::Processor;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, ScrollbarState};
use ratatui::Frame;
use std::io::{Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use throbber_widgets_tui::{Throbber, ThrobberState};
use widget::AlacrittyWidget;

/// Event listener that does nothing (we handle events ourselves)
pub struct DummyListener;

impl EventListener for DummyListener {
    fn send_event(&self, _event: Event) {}
}

/// Simple dimensions implementation for terminal size
#[derive(Debug, Clone, Copy)]
struct TermDimensions {
    columns: usize,
    screen_lines: usize,
    total_lines: usize,
}

impl TermDimensions {
    fn new(cols: usize, rows: usize, scrollback: usize) -> Self {
        Self {
            columns: cols,
            screen_lines: rows,
            total_lines: rows + scrollback,
        }
    }
}

impl Dimensions for TermDimensions {
    fn total_lines(&self) -> usize {
        self.total_lines
    }

    fn screen_lines(&self) -> usize {
        self.screen_lines
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

/// A terminal panel that displays an alacritty terminal instance
/// This is a terminal component using alacritty_terminal for better scrollback support
pub struct AlacTerm<T: EventListener> {
    /// The alacritty terminal instance (public for testing)
    pub term: Option<Arc<Mutex<Term<T>>>>,

    /// Title to display in the panel
    pub title: String,

    /// Whether the panel has focus
    pub focused: bool,

    /// Area occupied by this terminal panel
    pub area: Option<Rect>,

    /// Whether to show loading animation
    pub loading: bool,

    /// Throbber state for loading animation
    pub throbber_state: ThrobberState,

    /// Last known terminal dimensions (rows, cols) for change detection
    last_dimensions: Option<(u16, u16)>,

    /// Scroll offset for viewing terminal history (0 = bottom/latest)
    pub scroll_offset: i32,

    /// Scrollbar state for visualizing scroll position
    pub scrollbar_state: ScrollbarState,

    // Styling
    pub border_style: Style,
    pub focused_border_style: Style,

    // Text selection state
    selection_start: Option<(u16, u16)>, // (row, col) relative to terminal grid
    selection_end: Option<(u16, u16)>,   // (row, col) relative to terminal grid
    is_selecting: bool,

    /// Configurable key bindings
    pub keybindings: AlacTermKeyBindings,

    // Process management (for spawned processes)
    _master: Option<Arc<Mutex<Box<dyn MasterPty + Send>>>>,
    _child: Option<Box<dyn Child + Send + Sync>>,
    writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
    ready: Option<Arc<AtomicBool>>,
}

impl<T: EventListener> AlacTerm<T> {
    /// Create a new terminal panel without a terminal instance
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            term: None,
            title: title.into(),
            focused: false,
            area: None,
            loading: false,
            throbber_state: ThrobberState::default(),
            last_dimensions: None,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            border_style: Style::default().fg(Color::White),
            focused_border_style: Style::default().fg(Color::Cyan),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
            keybindings: AlacTermKeyBindings::default(),
            _master: None,
            _child: None,
            writer: None,
            ready: None,
        }
    }

    /// Set the key bindings configuration
    pub fn with_keybindings(mut self, keybindings: AlacTermKeyBindings) -> Self {
        self.keybindings = keybindings;
        self
    }

    /// Set the key bindings configuration (mutable reference version)
    pub fn set_keybindings(&mut self, keybindings: AlacTermKeyBindings) {
        self.keybindings = keybindings;
    }

    /// Send input to the spawned process
    pub fn send_input(&self, text: &str) {
        if let Some(ref writer) = self.writer {
            let mut writer = writer.lock().unwrap();
            if let Err(e) = writer.write_all(text.as_bytes()) {
                tracing::error!("Failed to send input to terminal: {}", e);
                return;
            }
            if let Err(e) = writer.flush() {
                tracing::error!("Failed to flush terminal input: {}", e);
            }
        }
    }

    /// Handle a key event and send appropriate terminal input
    /// Returns true if the key was handled, false otherwise
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        // Handle configurable keybindings first

        // Copy selection to clipboard
        if self.keybindings.copy_selection.matches(&key) {
            if let Some(text) = self.get_selected_text() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
                return true; // Consume the event - we copied
            }
            return false; // No selection, key not handled
        }

        // Paste from clipboard
        if self.keybindings.paste.matches(&key) {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    self.send_input(&text);
                    return true;
                }
            }
            return false;
        }

        // Scroll up
        if self.keybindings.scroll_up.matches(&key) {
            self.scroll_up(10);
            return true;
        }

        // Scroll down
        if self.keybindings.scroll_down.matches(&key) {
            self.scroll_down(10);
            return true;
        }

        // Clear selection (check if there's an active selection first)
        if self.keybindings.clear_selection.matches(&key) {
            if self.has_selection() {
                self.clear_selection();
                return true; // Consumed - we cleared a selection
            }
            // If no selection, fall through to send the key to terminal
        }

        // Convert keyevent to text and send to terminal
        let text = match key.code {
            KeyCode::Char(c) => {
                // Handle Ctrl+ combinations
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Convert Ctrl+letter to control character
                    // Ctrl+A = 0x01, Ctrl+B = 0x02, ..., Ctrl+Z = 0x1A
                    match c.to_ascii_lowercase() {
                        'a'..='z' => {
                            let code = (c.to_ascii_lowercase() as u8 - b'a' + 1) as char;
                            code.to_string()
                        }
                        '@' => "\x00".to_string(),  // Ctrl+@
                        '[' => "\x1b".to_string(),  // Ctrl+[
                        '\\' => "\x1c".to_string(), // Ctrl+\
                        ']' => "\x1d".to_string(),  // Ctrl+]
                        '^' => "\x1e".to_string(),  // Ctrl+^
                        '_' => "\x1f".to_string(),  // Ctrl+_
                        _ => c.to_string(),         // Pass through other Ctrl combinations
                    }
                } else if key.modifiers.contains(KeyModifiers::ALT) {
                    // Alt+key sends ESC followed by the key
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
            _ => return false,
        };

        self.send_input(&text);
        true
    }

    /// Resize the terminal
    fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        if let Some(ref master) = self._master {
            let pty_size = PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            };
            let master = master.lock().unwrap();
            master.resize(pty_size)?;
        }

        // Also resize the terminal
        if let Some(ref term) = self.term {
            let mut term = term.lock().unwrap();
            let dimensions = TermDimensions::new(cols as usize, rows as usize, 10000);
            term.resize(dimensions);
        }

        Ok(())
    }

    /// Scroll up through terminal history
    pub fn scroll_up(&mut self, amount: i32) {
        // Just increase the offset - bounds will be checked during render
        // Cap at a reasonable max to prevent overflow
        self.scroll_offset = (self.scroll_offset + amount).min(10000);
    }

    /// Scroll down through terminal history
    pub fn scroll_down(&mut self, amount: i32) {
        self.scroll_offset = (self.scroll_offset - amount).max(0);
    }

    /// Reset scroll to bottom (latest content)
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    /// Set the terminal instance
    pub fn with_term(mut self, term: Arc<Mutex<Term<T>>>) -> Self {
        self.term = Some(term);
        self
    }

    /// Set the term from a mutable reference
    pub fn set_term(&mut self, term: Arc<Mutex<Term<T>>>) {
        self.term = Some(term);
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Handle mouse click - returns true if the click was handled
    pub fn handle_click(&mut self, mouse_column: u16, mouse_row: u16) -> bool {
        if let Some(area) = self.area {
            let is_inside = mouse_column >= area.x
                && mouse_column < area.x + area.width
                && mouse_row >= area.y
                && mouse_row < area.y + area.height;

            if is_inside {
                self.focused = true;
                return true;
            }
        }
        false
    }

    /// Set border style
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Set focused border style
    pub fn focused_border_style(mut self, style: Style) -> Self {
        self.focused_border_style = style;
        self
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// Advance throbber animation (call this on each render when loading)
    pub fn tick_animation(&mut self) {
        self.throbber_state.calc_next();
    }

    /// Create the block with borders and title
    fn create_block(&self) -> Block<'_> {
        let border_style = if self.focused {
            self.focused_border_style
        } else {
            self.border_style
        };

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(&*self.title)
    }

    /// Render just the terminal content (no borders) - for composition with Pane
    pub fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        self.area = Some(area);

        // Update dimensions if changed
        let terminal_rows = area.height;
        let terminal_cols = area.width;
        let new_dimensions = (terminal_rows, terminal_cols);

        if self.last_dimensions != Some(new_dimensions) {
            if let Err(e) = self.resize(terminal_rows, terminal_cols) {
                tracing::error!("Failed to resize terminal: {}", e);
            }
            self.last_dimensions = Some(new_dimensions);
        }

        // Turn off loading animation when ready
        if let Some(ref ready) = self.ready {
            if ready.load(Ordering::Relaxed) {
                self.loading = false;
            }
        }

        // Show loading animation if loading
        if self.loading {
            let throbber = Throbber::default()
                .label("Loading Terminal...")
                .style(Style::default().fg(Color::Cyan))
                .throbber_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                )
                .throbber_set(throbber_widgets_tui::BRAILLE_SIX);

            self.tick_animation();
            frame.render_stateful_widget(throbber, area, &mut self.throbber_state);
        }
        // Render terminal content if available
        else if let Some(ref term) = self.term {
            let term_lock = term.lock().unwrap();
            let terminal_widget =
                AlacrittyWidget::new(&*term_lock).scroll_offset(self.scroll_offset.max(0) as usize);
            frame.render_widget(terminal_widget, area);
            drop(term_lock); // Release lock before rendering selection

            // Render selection highlight if active
            if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                self.render_selection(area, frame.buffer_mut(), start, end);
            }
        } else {
            let placeholder =
                Paragraph::new("No terminal attached").style(Style::default().fg(Color::DarkGray));
            frame.render_widget(placeholder, area);
        }
    }

    /// Render the terminal panel with borders (standalone mode)
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();
        let inner = block.inner(area);
        frame.render_widget(block, area);
        self.render_content(frame, inner);
    }

    /// Start text selection at the given coordinates
    pub fn start_selection(&mut self, x: u16, y: u16) {
        // Coordinates are relative to pane (including border)
        // Subtract 1 to account for border, then adjust for scroll
        let content_x = x.saturating_sub(1);
        let content_y = y.saturating_sub(1);
        let adjusted_y = content_y.saturating_add(self.scroll_offset.max(0) as u16);
        self.selection_start = Some((adjusted_y, content_x));
        self.selection_end = Some((adjusted_y, content_x));
        self.is_selecting = true;
    }

    /// Update text selection to the given coordinates
    pub fn update_selection(&mut self, x: u16, y: u16) {
        if self.is_selecting {
            // Same border adjustment as start_selection
            let content_x = x.saturating_sub(1);
            let content_y = y.saturating_sub(1);
            let adjusted_y = content_y.saturating_add(self.scroll_offset.max(0) as u16);
            self.selection_end = Some((adjusted_y, content_x));
        }
    }

    /// End text selection
    pub fn end_selection(&mut self) {
        self.is_selecting = false;
    }

    /// Clear the current selection
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
        self.is_selecting = false;
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some() && self.selection_end.is_some()
    }

    /// Get the currently selected text
    pub fn get_selected_text(&self) -> Option<String> {
        let (start, end) = match (self.selection_start, self.selection_end) {
            (Some(s), Some(e)) => (s, e),
            _ => return None,
        };

        // Normalize start/end so start is always before end
        let ((start_row, start_col), (end_row, end_col)) =
            if start.0 < end.0 || (start.0 == end.0 && start.1 <= end.1) {
                (start, end)
            } else {
                (end, start)
            };

        // Extract text from terminal grid
        let term = self.term.as_ref()?;
        let term = term.lock().ok()?;
        let grid = term.grid();

        let mut selected_text = String::new();

        // Iterate through each row in the selection
        for row in start_row..=end_row {
            let line_index = alacritty_terminal::index::Line(-(row as i32));

            // Build the line text from all cells in the row
            let mut line_text = String::new();
            let columns = grid.columns();

            for col in 0..columns {
                let cell = &grid[line_index][alacritty_terminal::index::Column(col)];
                let c = cell.c;
                if c != ' ' || col < columns - 1 {
                    line_text.push(c);
                }
            }

            // Trim trailing spaces from the line
            let line_text = line_text.trim_end();

            // Extract the appropriate substring based on selection bounds
            if row == start_row && row == end_row {
                // Single line selection
                let start_idx = start_col as usize;
                let end_idx = (end_col as usize).min(line_text.len());
                if start_idx < line_text.len() {
                    selected_text.push_str(&line_text[start_idx..end_idx]);
                }
            } else if row == start_row {
                // First line of multi-line selection
                let start_idx = (start_col as usize).min(line_text.len());
                selected_text.push_str(&line_text[start_idx..]);
                selected_text.push('\n');
            } else if row == end_row {
                // Last line of multi-line selection
                let end_idx = (end_col as usize).min(line_text.len());
                selected_text.push_str(&line_text[..end_idx]);
            } else {
                // Middle lines - entire line
                selected_text.push_str(line_text);
                selected_text.push('\n');
            }
        }

        if selected_text.is_empty() {
            None
        } else {
            Some(selected_text)
        }
    }

    /// Render selection highlighting on the buffer
    fn render_selection(
        &self,
        area: Rect,
        buf: &mut ratatui::buffer::Buffer,
        start: (u16, u16),
        end: (u16, u16),
    ) {
        use ratatui::style::{Color, Modifier, Style};

        // Normalize start/end so start is always before end
        let ((start_row, start_col), (end_row, end_col)) =
            if start.0 < end.0 || (start.0 == end.0 && start.1 <= end.1) {
                (start, end)
            } else {
                (end, start)
            };

        let selection_style = Style::default()
            .bg(Color::Rgb(70, 130, 180)) // Steel blue background
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        for row in start_row..=end_row {
            // Calculate visible row (accounting for scroll)
            let visible_row = row.saturating_sub(self.scroll_offset.max(0) as u16);

            // Skip if row is outside visible area
            if visible_row >= area.height {
                continue;
            }

            let y = area.y + visible_row;

            if row == start_row && row == end_row {
                // Single line selection
                for x in start_col..end_col {
                    if x < area.width && y < area.y + area.height {
                        if let Some(cell) = buf.cell_mut((area.x + x, y)) {
                            cell.set_style(selection_style);
                        }
                    }
                }
            } else if row == start_row {
                // First line - from start_col to end of line
                for x in start_col..area.width {
                    if y < area.y + area.height {
                        if let Some(cell) = buf.cell_mut((area.x + x, y)) {
                            cell.set_style(selection_style);
                        }
                    }
                }
            } else if row == end_row {
                // Last line - from start to end_col
                for x in 0..end_col.min(area.width) {
                    if y < area.y + area.height {
                        if let Some(cell) = buf.cell_mut((area.x + x, y)) {
                            cell.set_style(selection_style);
                        }
                    }
                }
            } else {
                // Middle lines - entire line
                for x in 0..area.width {
                    if y < area.y + area.height {
                        if let Some(cell) = buf.cell_mut((area.x + x, y)) {
                            cell.set_style(selection_style);
                        }
                    }
                }
            }
        }
    }
}

/// Impl block for spawnable terminals (using DummyListener)
impl AlacTerm<DummyListener> {
    /// Spawn a new terminal with a command
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

        // Create the PTY pair
        let pair = pty_system.openpty(pty_size)?;

        // Build the command
        let mut cmd = CommandBuilder::new(command);
        for arg in args {
            cmd.arg(arg);
        }

        // Set the current working directory
        let current_dir = std::env::current_dir()?;
        cmd.cwd(current_dir);

        // Spawn the child process
        let child = pair.slave.spawn_command(cmd)?;

        // Set master PTY to non-blocking mode on Unix
        #[cfg(unix)]
        {
            if let Some(fd) = pair.master.as_raw_fd() {
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
            }
        }

        // Get reader and writer for the master PTY
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        // Create alacritty terminal with scrollback
        let event_listener = DummyListener;
        let dimensions = TermDimensions::new(cols as usize, rows as usize, 10000);
        let term = Term::new(Config::default(), &dimensions, event_listener);

        let processor: Processor = Processor::new();

        // Wrap everything in Arc/Mutex for sharing across threads
        let term = Arc::new(Mutex::new(term));
        let processor = Arc::new(Mutex::new(processor));
        let master = Arc::new(Mutex::new(pair.master));
        let reader = Arc::new(Mutex::new(reader));
        let writer = Arc::new(Mutex::new(writer));
        let ready = Arc::new(AtomicBool::new(false));

        // Start background task to read process output
        let term_clone = Arc::clone(&term);
        let processor_clone = Arc::clone(&processor);
        let reader_clone = Arc::clone(&reader);
        let ready_clone = Arc::clone(&ready);

        tokio::spawn(async move {
            let mut first_output = false;
            let mut buf = [0u8; 8192];

            loop {
                let result = {
                    let mut reader = reader_clone.lock().unwrap();
                    reader.read(&mut buf)
                };

                match result {
                    Ok(0) => {
                        tracing::info!("Terminal process exited");
                        break;
                    }
                    Ok(n) => {
                        {
                            let mut term = term_clone.lock().unwrap();
                            let mut processor = processor_clone.lock().unwrap();

                            // Process the bytes through the VT parser
                            processor.advance(&mut *term, &buf[..n]);
                        }

                        if !first_output {
                            first_output = true;
                            ready_clone.store(true, Ordering::Relaxed);
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(e) => {
                        tracing::error!("Error reading process output: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Self {
            term: Some(term),
            title: title.into(),
            focused: false,
            area: None,
            loading: true,
            throbber_state: ThrobberState::default(),
            last_dimensions: None,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            border_style: Style::default().fg(Color::White),
            focused_border_style: Style::default().fg(Color::Cyan),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
            keybindings: AlacTermKeyBindings::default(),
            _master: Some(master),
            _child: Some(child),
            writer: Some(writer),
            ready: Some(ready),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alacritty_terminal::term::{Config, Term};
    use alacritty_terminal::vte::ansi::Processor;

    #[test]
    fn test_selection_coordinate_mapping() {
        // Test that selection correctly accounts for border offset
        let mut term = AlacTerm::<DummyListener>::new("Test Terminal");

        // Simulate mouse down at pane coordinates (5, 3)
        // With border, content coordinates should be (4, 2)
        term.start_selection(5, 3);

        assert!(term.has_selection());
        assert_eq!(term.selection_start, Some((2, 4))); // (row, col) in content coordinates
    }

    #[test]
    fn test_selection_drag() {
        let mut term = AlacTerm::<DummyListener>::new("Test Terminal");

        // Start selection at (5, 3) -> content (4, 2)
        term.start_selection(5, 3);

        // Drag to (15, 5) -> content (14, 4)
        term.update_selection(15, 5);

        assert_eq!(term.selection_start, Some((2, 4)));
        assert_eq!(term.selection_end, Some((4, 14)));
    }

    #[test]
    fn test_clear_selection() {
        let mut term = AlacTerm::<DummyListener>::new("Test Terminal");

        term.start_selection(5, 3);
        term.update_selection(15, 5);
        assert!(term.has_selection());

        term.clear_selection();
        assert!(!term.has_selection());
        assert_eq!(term.selection_start, None);
        assert_eq!(term.selection_end, None);
    }

    #[test]
    fn test_text_extraction_single_line() {
        // Create a terminal with some content
        let event_listener = DummyListener;
        let dimensions = TermDimensions::new(80, 24, 100);
        let mut term_instance = Term::new(Config::default(), &dimensions, event_listener);

        // Write some test text to the terminal
        let mut processor: Processor = Processor::new();
        let test_text = b"Hello, World!\r\n";
        processor.advance(&mut term_instance, test_text);

        let term = Arc::new(Mutex::new(term_instance));
        let alac_term = AlacTerm {
            term: Some(term),
            title: "Test".to_string(),
            focused: false,
            area: None,
            loading: false,
            throbber_state: ThrobberState::default(),
            last_dimensions: None,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            border_style: Style::default(),
            focused_border_style: Style::default(),
            selection_start: Some((0, 0)), // Start of "Hello, World!"
            selection_end: Some((0, 5)),   // "Hello"
            is_selecting: false,
            keybindings: AlacTermKeyBindings::default(),
            _master: None,
            _child: None,
            writer: None,
            ready: None,
        };

        let selected = alac_term.get_selected_text();
        assert!(selected.is_some());
        let text = selected.unwrap();
        assert!(
            text.starts_with("Hello"),
            "Expected text starting with 'Hello', got: {:?}",
            text
        );
    }

    #[test]
    fn test_end_selection_stops_selecting() {
        let mut term = AlacTerm::<DummyListener>::new("Test Terminal");

        term.start_selection(5, 3);
        assert!(term.is_selecting);

        term.end_selection();
        assert!(!term.is_selecting);

        // Selection coordinates should still be present
        assert!(term.has_selection());
    }
}
