use crate::widgets::ai_chat::state::{InputState, MessageStore};
use ratatui::style::{Color, Modifier, Style};

use crate::widgets::ai_chat::AIChat;

impl<'a> AIChat<'a> {
    /// Create a new AI chat widget.
    pub fn new_ai_chat(messages: &'a mut MessageStore, input: &'a mut InputState) -> Self {
        Self {
            messages,
            input,
            attached_files: Vec::new(),
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
}
