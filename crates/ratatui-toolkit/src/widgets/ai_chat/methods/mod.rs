use ratatui::style::Style;

use crate::widgets::ai_chat::{AIChat, AIChatEvent, InputState, MessageStore};

impl<'a> AIChat {
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
                self.messages
                    .add(crate::widgets::ai_chat::Message::user(result.clone()));
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
