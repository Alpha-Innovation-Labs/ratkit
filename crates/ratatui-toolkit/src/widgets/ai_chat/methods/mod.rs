use crate::widgets::ai_chat::state::{InputState, MessageStore};
use ratatui::style::{Color, Modifier, Style};

use crate::widgets::ai_chat::AIChat;

impl<'a> AIChat<'a> {
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
}
