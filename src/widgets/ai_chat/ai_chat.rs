//! AI Chat Widget for interactive chat interfaces.
//!
//! Provides a chat interface with:
//! - Multi-line text input (Ctrl+J for newline)
//! - File attachments via @ prefix with fuzzy search
//! - Commands via / prefix (e.g., /clear)
//! - Message history display
//! - Loading spinner for AI responses

use crate::widgets::ai_chat::{InputState, Message, MessageRole, MessageStore};
use ratatui::style::Style;

/// Result of handling a key event.
#[derive(Debug, Clone, PartialEq)]
pub enum AIChatEvent {
    /// No event
    None,
    /// Message submitted
    MessageSubmitted(String),
    /// File attached
    FileAttached(String),
    /// Command executed
    Command(String),
}

/// AI Chat widget for interactive chat interfaces.
pub struct AIChat {
    /// Store for chat messages
    messages: MessageStore,
    /// Input state for text entry
    input: InputState,
    /// Whether AI is generating a response
    is_loading: bool,
    /// Style for user messages
    user_message_style: Style,
    /// Style for AI messages
    ai_message_style: Style,
    /// Style for input area
    input_style: Style,
    /// Prompt text for input
    input_prompt: String,
    /// Available commands
    commands: Vec<String>,
    /// Selected command index in command mode
    selected_command_index: usize,
}

impl AIChat {
    /// Create a new AI chat widget.
    pub fn new() -> Self {
        Self {
            messages: MessageStore::new(),
            input: InputState::new(),
            is_loading: false,
            user_message_style: Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
            ai_message_style: Style::default().fg(Color::White),
            input_style: Style::default().fg(Color::White),
            input_prompt: "You: ".to_string(),
            commands: vec!["/clear".to_string()],
            selected_command_index: 0,
        }
    }

    /// Set selected command index (for builder pattern).
    pub fn with_selected_command_index(mut self, index: usize) -> Self {
        self.selected_command_index = index;
        self
    }

    /// Register a command.
    pub fn register_command(&mut self, command: String) {
        if !self.commands.contains(&command) {
            self.commands.push(command);
        }
    }

    /// Get available commands.
    pub fn commands(&self) -> &[String] {
        &self.commands
    }

    /// Get filtered commands matching the current command input.
    pub fn filtered_commands(&self) -> Vec<String> {
        let command_lower = self.input.command().to_lowercase();
        self.commands
            .iter()
            .filter(|c| c.to_lowercase().starts_with(&format!("/{}", command_lower)))
            .cloned()
            .collect()
    }

    /// Get selected command index.
    pub fn selected_command_index(&self) -> usize {
        self.selected_command_index
    }

    /// Set selected command index.
    pub fn set_selected_command_index(&mut self, index: usize) {
        self.selected_command_index = index;
    }

    /// Handle a command string (e.g., "/clear").
    ///
    /// Returns true if command was handled, false if unknown.
    pub fn handle_command(&mut self, command: &str) -> bool {
        match command {
            "/clear" => {
                self.messages.clear();
                true
            }
            _ => false,
        }
    }

    /// Set the loading state.
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    /// Get the loading state.
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Set user message style.
    pub fn with_user_message_style(mut self, style: Style) -> Self {
        self.user_message_style = style;
        self
    }

    /// Set AI message style.
    pub fn with_ai_message_style(mut self, style: Style) -> Self {
        self.ai_message_style = style;
        self
    }

    /// Set input style.
    pub fn with_input_style(mut self, style: Style) -> Self {
        self.input_style = style;
        self
    }

    /// Set input prompt text.
    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.input_prompt = prompt;
        self
    }

    /// Handle a key event.
    ///
    /// Returns an event indicating what happened.
    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) -> AIChatEvent {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let key = KeyEvent::new(key, KeyModifiers::NONE);

        if let Some(result) = self.input.handle_key(key) {
            if result.starts_with('@') {
                return AIChatEvent::FileAttached(result);
            }
            if result.starts_with('/') {
                if self.handle_command(&result) {
                    return AIChatEvent::Command(result);
                }
                return AIChatEvent::Command(result);
            }
            if !result.is_empty() {
                self.messages.add(Message::user(result.clone()));
                self.is_loading = true;
                return AIChatEvent::MessageSubmitted(result);
            }
        }
        AIChatEvent::None
    }

    /// Get messages reference.
    pub fn messages(&self) -> &MessageStore {
        &self.messages
    }

    /// Get messages mutable reference.
    pub fn messages_mut(&mut self) -> &mut MessageStore {
        &mut self.messages
    }

    /// Get input reference.
    pub fn input(&self) -> &InputState {
        &self.input
    }

    /// Get input mutable reference.
    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style as TuiStyle},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

impl AIChat {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let messages_area = chunks[0];
        let input_area = chunks[1];

        self.render_messages(frame, messages_area);
        self.render_input(frame, input_area);

        if self.input.is_file_mode() {
            self.render_file_popup(frame, input_area);
        } else if self.input.is_command_mode() {
            self.render_command_popup(frame, input_area);
        }
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Chat ");

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut items = Vec::new();

        for msg in self.messages.messages() {
            let prefix = match msg.role {
                MessageRole::User => "You: ",
                MessageRole::Assistant => "AI:  ",
            };

            let style = match msg.role {
                MessageRole::User => self.user_message_style,
                MessageRole::Assistant => self.ai_message_style,
            };

            let mut content = vec![Span::styled(prefix, style)];

            if !msg.attachments.is_empty() {
                let files_str = msg
                    .attachments
                    .iter()
                    .map(|f| format!("@{}", f))
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push(Span::styled(
                    format!("[{}] ", files_str),
                    TuiStyle::default().fg(Color::Yellow),
                ));
            }

            content.push(Span::raw(&msg.content));

            let line = Line::from(content);
            items.push(ListItem::new(line));
        }

        if self.is_loading {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("AI:  ", self.ai_message_style),
                Span::styled("⠋ Thinking...", TuiStyle::default().fg(Color::Gray)),
            ])));
        }

        let list = List::new(items)
            .block(Block::default())
            .style(TuiStyle::default());

        frame.render_widget(list, inner);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let mut input_text = self.input.text().to_string();

        if self.input.is_file_mode() {
            let filtered = self.input.filtered_files();
            if let Some(file) = filtered.get(self.input.selected_file_index()) {
                input_text = format!("@{}{}", self.input.file_query(), file);
            } else {
                input_text = format!("@{}", self.input.file_query());
            }
        } else if self.input.is_command_mode() {
            let filtered = self.filtered_commands();
            if let Some(cmd) = filtered.get(self.selected_command_index()) {
                input_text = cmd.clone();
            } else {
                input_text = format!("/{}", self.input.command());
            }
        }

        let prompt = &self.input_prompt;
        let cursor_pos = prompt.len() + self.input.cursor();

        let paragraph = Paragraph::new(format!("{}{}", prompt, input_text))
            .style(self.input_style)
            .block(Block::default());

        frame.render_widget(paragraph, area);

        if cursor_pos < input_text.len() + prompt.len() {
            let cursor_x = area.x + cursor_pos as u16;
            let cursor_y = area.y;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }

    fn render_file_popup(&self, frame: &mut Frame, input_area: Rect) {
        let filtered = self.input.filtered_files();

        if filtered.is_empty() {
            return;
        }

        let max_height = 10.min(filtered.len() as u16);
        let popup_height = max_height + 2;

        let popup_y = if input_area.y.saturating_sub(popup_height) > 0 {
            input_area.y.saturating_sub(popup_height)
        } else {
            input_area.y.saturating_add(1)
        };

        let popup_width = 40.min(input_area.width);
        let popup_x = input_area.x;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let style = if i == self.input.selected_file_index() {
                    TuiStyle::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    TuiStyle::default().fg(Color::White).bg(Color::Black)
                };
                ListItem::new(Span::styled(file.clone(), style))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(TuiStyle::default().bg(Color::Black)),
        );

        frame.render_widget(list, popup_area);
    }

    fn render_command_popup(&self, frame: &mut Frame, input_area: Rect) {
        let filtered = self.filtered_commands();

        if filtered.is_empty() {
            return;
        }

        let max_height = 10.min(filtered.len() as u16);
        let popup_height = max_height + 2;

        let popup_y = if input_area.y.saturating_sub(popup_height) > 0 {
            input_area.y.saturating_sub(popup_height)
        } else {
            input_area.y.saturating_add(1)
        };

        let popup_width = 40.min(input_area.width);
        let popup_x = input_area.x;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(i, cmd)| {
                let style = if i == self.selected_command_index() {
                    TuiStyle::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    TuiStyle::default().fg(Color::White).bg(Color::Black)
                };
                ListItem::new(Span::styled(cmd.clone(), style))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(TuiStyle::default().bg(Color::Black)),
        );

        frame.render_widget(list, popup_area);
    }
}
