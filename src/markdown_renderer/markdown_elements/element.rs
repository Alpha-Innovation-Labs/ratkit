//! Markdown element for markdown rendering.
//!
//! Represents a single markdown element that can be rendered to ratatui.

use super::enums::ElementKind;

#[derive(Debug, Clone, Default)]
pub struct MarkdownElement {
    /// The kind of element content.
    pub kind: ElementKind,
    /// The section this element belongs to (for collapse/expand).
    /// None means this element is not part of any collapsible section.
    pub section_id: Option<usize>,
    /// The source line number (1-indexed) in the original markdown.
    /// Used for double-click reporting. Default is 0 (unknown).
    pub source_line: usize,
}

impl MarkdownElement {
    /// Create a new markdown element with source line tracking.
    pub fn new(kind: ElementKind, section_id: Option<usize>, source_line: usize) -> Self {
        Self {
            kind,
            section_id,
            source_line,
        }
    }
}
