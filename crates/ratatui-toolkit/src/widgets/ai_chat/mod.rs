//! AI Chat Widget for interactive chat interfaces.
//!
//! Provides a chat interface with:
//! - Multi-line text input (Ctrl+J for newline)
//! - File attachments via @ prefix with fuzzy search
//! - Commands via / prefix (e.g., /clear)
//! - Message history display
//! - Loading spinner for AI responses

mod constructors;
pub mod methods;
pub mod state;
pub mod traits;

use ratatui::style::Style;

pub use state::{InputState, Message, MessageRole, MessageStore};

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
pub struct AIChat<'a> {
    /// Store for chat messages
    messages: &'a mut MessageStore,
    /// Input state for text entry
    input: &'a mut InputState,
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
