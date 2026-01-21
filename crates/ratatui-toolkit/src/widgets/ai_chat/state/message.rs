use ratatui::style::{Color, Modifier, Style};

/// Role of a message sender.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    /// Message from user
    User,
    /// Message from AI
    Assistant,
}

/// A single chat message.
#[derive(Debug, Clone)]
pub struct Message {
    /// Role of sender
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Optional file attachments
    pub attachments: Vec<String>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl Message {
    /// Create a new user message.
    pub fn user(content: String) -> Self {
        Self {
            role: MessageRole::User,
            content,
            attachments: Vec::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create a new AI message.
    pub fn assistant(content: String) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            attachments: Vec::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Add file attachment.
    pub fn with_attachment(mut self, file: String) -> Self {
        self.attachments.push(file);
        self
    }
}

/// Store for chat message history.
#[derive(Debug, Default, Clone)]
pub struct MessageStore {
    /// All messages in conversation
    messages: Vec<Message>,
}

impl MessageStore {
    /// Create a new empty message store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a message to store.
    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Get all messages.
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Clear all messages.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get number of messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if store is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get last message.
    pub fn last(&self) -> Option<&Message> {
        self.messages.last()
    }
}
