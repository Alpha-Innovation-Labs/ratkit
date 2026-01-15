use crate::alac_term::{AlacTerm, DummyListener};
use crate::pane::Pane;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::Frame;

/// An AI chat panel that provides Claude Code-specific features
/// Built on top of AlacTerm for terminal management
pub struct AiChat {
    /// The underlying terminal component
    terminal: AlacTerm<DummyListener>,

    /// Title to display in the panel
    pub title: String,

    /// Whether the panel has focus
    pub focused: bool,

    /// Area occupied by this chat panel (from terminal)
    pub area: Option<Rect>,

    // Claude-specific state (for future features)
    /// Current session ID (if any)
    pub current_session: Option<String>,

    // Styling
    pub border_style: Style,
    pub focused_border_style: Style,
}

impl AiChat {
    /// Create a new AI chat panel and spawn Claude terminal
    pub fn new(title: impl Into<String>) -> Result<Self> {
        // Spawn Claude using AlacTerm
        let terminal = AlacTerm::spawn_with_command(
            "", // AlacTerm has its own title, but we'll use Pane for display
            "claude",
            &[],
        )?;

        Ok(Self {
            terminal,
            title: title.into(),
            focused: false,
            area: None,
            current_session: None,
            border_style: Style::default().fg(Color::White),
            focused_border_style: Style::default().fg(Color::Cyan),
        })
    }

    /// Restart with a specific session (for future session management)
    pub fn restart_with_session(&mut self, _session: &str) -> Result<()> {
        // TODO: Implement session management
        // self.terminal = AlacTerm::spawn_with_command("", "claude", &["--session", session])?;
        // self.current_session = Some(session.to_string());
        Ok(())
    }

    /// Scroll up through terminal history
    pub fn scroll_up(&mut self, amount: i32) {
        self.terminal.scroll_up(amount);
    }

    /// Scroll down through terminal history
    pub fn scroll_down(&mut self, amount: i32) {
        self.terminal.scroll_down(amount);
    }

    /// Reset scroll to bottom (latest content)
    pub fn scroll_to_bottom(&mut self) {
        self.terminal.scroll_to_bottom();
    }

    /// Send input to Claude
    pub fn send_input(&self, text: &str) {
        self.terminal.send_input(text);
    }

    /// Handle keyboard input - converts KeyEvent to terminal input
    /// Returns true if the event was handled
    pub fn handle_key_event(&self, key: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }

        let text = match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Convert Ctrl+letter to control code
                    if c.is_ascii_alphabetic() {
                        let ctrl_code = (c.to_ascii_lowercase() as u8 - b'a' + 1) as char;
                        ctrl_code.to_string()
                    } else {
                        return false;
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
            KeyCode::Left => "\x1b[D".to_string(),
            KeyCode::Right => "\x1b[C".to_string(),
            KeyCode::Up => "\x1b[A".to_string(),
            KeyCode::Down => "\x1b[B".to_string(),
            KeyCode::Home => "\x1b[H".to_string(),
            KeyCode::End => "\x1b[F".to_string(),
            KeyCode::PageUp => "\x1b[5~".to_string(),
            KeyCode::PageDown => "\x1b[6~".to_string(),
            KeyCode::Delete => "\x1b[3~".to_string(),
            KeyCode::Insert => "\x1b[2~".to_string(),
            _ => return false,
        };

        self.send_input(&text);
        true
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        self.terminal.set_focused(focused);
    }

    /// Handle mouse click - returns true if the click was handled
    pub fn handle_click(&mut self, mouse_column: u16, mouse_row: u16) -> bool {
        if self.terminal.handle_click(mouse_column, mouse_row) {
            self.focused = true;
            self.area = self.terminal.area;
            true
        } else {
            false
        }
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

    /// Render the chat panel using Pane for borders
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = Some(area);

        // Create pane with Claude-specific styling
        let mut pane = Pane::new(&self.title).border_style(if self.focused {
            self.focused_border_style
        } else {
            self.border_style
        });

        // Add session info to footer if present
        if let Some(ref session) = self.current_session {
            pane = pane.with_text_footer(Line::from(format!("Session: {}", session)));
        }

        // Render pane and get content area
        let (content_area, _) = pane.render_block(frame, area);

        // Render terminal content (no borders - Pane handles that)
        self.terminal.render_content(frame, content_area);
    }
}

impl Default for AiChat {
    fn default() -> Self {
        Self::new("AI Chat").expect("Failed to spawn Claude terminal")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alacritty_terminal::grid::Dimensions;
    use std::time::Duration;

    /// Helper method to extract text content from the terminal for testing
    impl AiChat {
        #[cfg(test)]
        fn get_terminal_content(&self) -> String {
            // Access the terminal through AlacTerm
            if let Some(ref term) = self.terminal.term {
                let term = term.lock().unwrap();
                let grid = term.grid();
                let mut content = String::new();

                // Read all visible lines from the terminal
                for row in 0..grid.screen_lines() {
                    let line = grid
                        .display_iter()
                        .skip(row * grid.columns())
                        .take(grid.columns())
                        .map(|cell| cell.c)
                        .collect::<String>();
                    content.push_str(&line);
                    content.push('\n');
                }

                content
            } else {
                String::new()
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires Claude to be installed and available
    async fn test_send_input_displays_in_terminal() {
        // Create an AiChat instance (spawns Claude)
        let ai_chat = AiChat::new("Test Chat").expect("Failed to create AiChat");

        // Wait for Claude to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Send test input
        let test_message = "test input message";
        ai_chat.send_input(test_message);
        ai_chat.send_input("\r"); // Send Enter key

        // Wait for the input to be processed and displayed
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Get terminal content
        let content = ai_chat.get_terminal_content();

        // Verify the input appears in the terminal
        assert!(
            content.contains(test_message),
            "Expected terminal to contain '{}', but got:\n{}",
            test_message,
            content
        );
    }

    #[tokio::test]
    #[ignore] // Requires Claude to be installed and available
    async fn test_multiple_inputs_display_sequentially() {
        // Create an AiChat instance
        let ai_chat = AiChat::new("Test Chat").expect("Failed to create AiChat");

        // Wait for Claude to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Send multiple inputs
        ai_chat.send_input("first message\r");
        tokio::time::sleep(Duration::from_millis(200)).await;

        ai_chat.send_input("second message\r");
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Get terminal content
        let content = ai_chat.get_terminal_content();

        // Verify both inputs appear
        assert!(
            content.contains("first message"),
            "Expected terminal to contain 'first message'"
        );
        assert!(
            content.contains("second message"),
            "Expected terminal to contain 'second message'"
        );
    }

    #[tokio::test]
    #[ignore] // Requires Claude to be installed and available
    async fn test_input_without_enter_key() {
        // Create an AiChat instance
        let ai_chat = AiChat::new("Test Chat").expect("Failed to create AiChat");

        // Wait for Claude to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Send input without pressing Enter
        let test_message = "incomplete input";
        ai_chat.send_input(test_message);

        // Wait for display
        tokio::time::sleep(Duration::from_millis(300)).await;

        // Get terminal content
        let content = ai_chat.get_terminal_content();

        // The input should still be visible (in the input line)
        assert!(
            content.contains(test_message),
            "Expected terminal to show typed text '{}' even without Enter",
            test_message
        );
    }
}
