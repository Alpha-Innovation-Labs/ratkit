//! Event returned when a line is double-clicked in the markdown widget.

/// Event returned when a line is double-clicked in the markdown widget.
#[derive(Debug, Clone)]
pub struct MarkdownDoubleClickEvent {
    /// The logical line number (0-indexed) in the document.
    pub line_number: usize,
    /// The kind of line that was clicked.
    pub line_kind: String,
    /// Plain text content of the line.
    pub content: String,
}
