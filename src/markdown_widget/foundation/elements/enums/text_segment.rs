//! Text segment types for markdown styling.
//!
//! Represents different types of text segments within markdown content.

use crate::markdown_widget::foundation::elements::enums::CheckboxState;

/// Represents a segment of text with specific styling.
#[derive(Debug, Clone)]
pub enum TextSegment {
    /// Plain text.
    Plain(String),
    /// Bold text.
    Bold(String),
    /// Italic text.
    Italic(String),
    /// Bold and italic text.
    BoldItalic(String),
    /// Inline code with background.
    InlineCode(String),
    /// Link text with URL and whether it's an autolink (bare URL).
    Link {
        text: String,
        url: String,
        /// True if this is an autolink (bare URL), false if `[text](url)` style.
        is_autolink: bool,
        /// Whether the link text is bold.
        bold: bool,
        /// Whether the link text is italic.
        italic: bool,
        /// Whether to show the icon (only true for first segment of a link).
        show_icon: bool,
    },
    /// Strikethrough text.
    Strikethrough(String),
    /// HTML tag or autolink.
    Html(String),
    /// Checkbox for task lists.
    Checkbox(CheckboxState),
}
