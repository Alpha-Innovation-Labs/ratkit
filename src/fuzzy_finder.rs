use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use vt100::Parser;

/// A generic fuzzy finder widget that spawns a PTY-based fuzzy finder (like fzf)
pub struct FuzzyFinder {
    /// PTY terminal state
    terminal: Option<FuzzyFinderTerminal>,
    /// Popup size (percentage of screen)
    size_percent: (u16, u16),
    /// Title for the popup
    title: String,
    /// Loading message (before terminal spawns)
    loading_message: String,
}

struct FuzzyFinderTerminal {
    parser: Arc<Mutex<Parser>>,
    _master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl FuzzyFinder {
    /// Create a new fuzzy finder with a title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            terminal: None,
            size_percent: (80, 80),
            title: title.into(),
            loading_message: "Loading...".to_string(),
        }
    }

    /// Set popup size (percentage of screen width and height)
    pub fn with_size(mut self, width_pct: u16, height_pct: u16) -> Self {
        self.size_percent = (width_pct, height_pct);
        self
    }

    /// Set loading message displayed before terminal spawns
    pub fn with_loading_message(mut self, msg: impl Into<String>) -> Self {
        self.loading_message = msg.into();
        self
    }

    /// Spawn fzf with items
    pub fn spawn_fzf(
        &mut self,
        items: Vec<String>,
        rows: u16,
        cols: u16,
        prompt: Option<&str>,
    ) -> Result<()> {
        if items.is_empty() {
            return Err(anyhow::anyhow!("Cannot spawn fzf with empty item list"));
        }

        if rows == 0 || cols == 0 {
            return Err(anyhow::anyhow!("Invalid terminal size: {}x{}", rows, cols));
        }

        let pty_system = native_pty_system();

        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        // Create the PTY pair
        tracing::debug!("Opening PTY with size {}x{}", rows, cols);
        let pair = pty_system
            .openpty(pty_size)
            .context("Failed to allocate PTY")?;

        // Build the fzf command
        let mut cmd = CommandBuilder::new("fzf");
        cmd.arg("--prompt");
        cmd.arg(prompt.unwrap_or("Select: "));
        cmd.arg("--height");
        cmd.arg("100%");
        cmd.arg("--layout");
        cmd.arg("reverse");
        cmd.arg("--info");
        cmd.arg("inline");
        cmd.arg("--ansi");

        // Spawn the child process
        tracing::debug!("Spawning fzf command");
        let child = pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn fzf process")?;
        tracing::debug!("fzf process spawned successfully");

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

        // Get reader for the master PTY
        let reader = pair.master.try_clone_reader()?;

        // Get writer to send items to fzf's stdin
        let mut writer = pair.master.take_writer()?;

        // Write items to fzf's stdin
        let items_str = items.join("\n") + "\n";
        writer.write_all(items_str.as_bytes())?;
        writer.flush()?;

        // Create VT100 parser for terminal emulation
        let parser = Arc::new(Mutex::new(Parser::new(rows, cols, 0)));

        self.terminal = Some(FuzzyFinderTerminal {
            parser,
            _master: Arc::new(Mutex::new(pair.master)),
            child: Arc::new(Mutex::new(child)),
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        });

        Ok(())
    }

    /// Spawn a custom command with optional stdin
    pub fn spawn_command(
        &mut self,
        cmd: &str,
        args: &[&str],
        stdin: Option<String>,
        rows: u16,
        cols: u16,
    ) -> Result<()> {
        if rows == 0 || cols == 0 {
            return Err(anyhow::anyhow!("Invalid terminal size: {}x{}", rows, cols));
        }

        let pty_system = native_pty_system();

        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = pty_system
            .openpty(pty_size)
            .context("Failed to allocate PTY")?;

        let mut command = CommandBuilder::new(cmd);
        for arg in args {
            command.arg(arg);
        }

        let child = pair
            .slave
            .spawn_command(command)
            .context("Failed to spawn command")?;

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
        let mut writer = pair.master.take_writer()?;

        // Write stdin if provided
        if let Some(input) = stdin {
            writer.write_all(input.as_bytes())?;
            writer.flush()?;
        }

        let parser = Arc::new(Mutex::new(Parser::new(rows, cols, 0)));

        self.terminal = Some(FuzzyFinderTerminal {
            parser,
            _master: Arc::new(Mutex::new(pair.master)),
            child: Arc::new(Mutex::new(child)),
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        });

        Ok(())
    }

    /// Send keyboard input to the terminal
    pub fn send_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let key_bytes = key_event_to_bytes(key);
            let mut writer = terminal.writer.lock().unwrap();
            writer.write_all(&key_bytes)?;
            writer.flush()?;
        }
        Ok(())
    }

    /// Read output from terminal (non-blocking) - call this regularly to update the display
    pub fn update(&mut self) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let mut buf = [0u8; 8192];
            let mut reader = terminal.reader.lock().unwrap();

            match reader.read(&mut buf) {
                Ok(0) => {} // EOF - process exited
                Ok(n) => {
                    let mut parser = terminal.parser.lock().unwrap();
                    parser.process(&buf[..n]);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No data available right now
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    /// Try to read the selected result (if process exited)
    /// Returns the last non-empty line from the terminal output
    pub fn get_selection(&mut self) -> Option<String> {
        if let Some(terminal) = &self.terminal {
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();
            let contents_bytes = screen.contents_formatted();
            let contents_str = String::from_utf8_lossy(&contents_bytes);

            // Return last non-empty line
            for line in contents_str.lines().rev() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
        None
    }

    /// Check if process is still running
    pub fn is_running(&self) -> bool {
        if let Some(terminal) = &self.terminal {
            if let Ok(mut child) = terminal.child.try_lock() {
                return child.try_wait().ok().flatten().is_none();
            }
        }
        false
    }

    /// Kill the process
    pub fn kill(&mut self) -> Result<()> {
        if let Some(terminal) = &self.terminal {
            let mut child = terminal.child.lock().unwrap();
            child.kill()?;
        }
        Ok(())
    }

    /// Get the parser for custom rendering (advanced use)
    pub fn get_parser(&self) -> Option<Arc<Mutex<Parser>>> {
        self.terminal.as_ref().map(|t| Arc::clone(&t.parser))
    }
}

impl Widget for &FuzzyFinder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate centered popup area
        let popup_area = centered_rect(self.size_percent.0, self.size_percent.1, area);

        // Clear background
        Clear.render(popup_area, buf);

        // Render border
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .style(Style::default().fg(Color::White));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        // Render content
        if let Some(terminal) = &self.terminal {
            // Render PTY output
            let parser = terminal.parser.lock().unwrap();
            let screen = parser.screen();

            // Render terminal content line by line
            for (row_idx, row) in screen.rows(0, inner.height).enumerate() {
                if row_idx >= inner.height as usize {
                    break;
                }

                // row is now a String, so we can use it directly
                let line = Line::from(row.as_str());
                let y = inner.y + row_idx as u16;
                buf.set_line(inner.x, y, &line, inner.width);
            }
        } else {
            // Render loading message
            let loading = Paragraph::new(self.loading_message.as_str())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            loading.render(inner, buf);
        }
    }
}

impl Drop for FuzzyFinder {
    fn drop(&mut self) {
        // Clean up resources
        if let Some(terminal) = &self.terminal {
            if let Ok(mut child) = terminal.child.lock() {
                let _ = child.kill();
            }
        }
    }
}

/// Helper function to center a rect within another rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width.saturating_mul(percent_x) / 100;
    let popup_height = r.height.saturating_mul(percent_y) / 100;

    let popup_x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let popup_y = r.y + (r.height.saturating_sub(popup_height)) / 2;

    Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    }
}

/// Convert crossterm KeyEvent to bytes for PTY input
fn key_event_to_bytes(key: crossterm::event::KeyEvent) -> Vec<u8> {
    use crossterm::event::{KeyCode, KeyModifiers};

    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Ctrl+key combinations
                match c {
                    'a'..='z' => vec![(c as u8) - b'a' + 1],
                    _ => vec![c as u8],
                }
            } else {
                c.to_string().into_bytes()
            }
        }
        KeyCode::Enter => vec![b'\r'],
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Delete => vec![0x1b, b'[', b'3', b'~'],
        KeyCode::Left => vec![0x1b, b'[', b'D'],
        KeyCode::Right => vec![0x1b, b'[', b'C'],
        KeyCode::Up => vec![0x1b, b'[', b'A'],
        KeyCode::Down => vec![0x1b, b'[', b'B'],
        KeyCode::Home => vec![0x1b, b'[', b'H'],
        KeyCode::End => vec![0x1b, b'[', b'F'],
        KeyCode::PageUp => vec![0x1b, b'[', b'5', b'~'],
        KeyCode::PageDown => vec![0x1b, b'[', b'6', b'~'],
        KeyCode::Tab => vec![b'\t'],
        KeyCode::Esc => vec![0x1b],
        _ => vec![],
    }
}
