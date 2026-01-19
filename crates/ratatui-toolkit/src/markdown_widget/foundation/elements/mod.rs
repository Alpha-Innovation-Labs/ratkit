//! Markdown element for markdown rendering.
//!
//! Represents a single markdown element that can be rendered to ratatui.

pub mod constants;
pub mod constructors;
pub mod enums;
pub mod methods;

pub use constants::{
    get_language_icon, get_link_icon, heading_bg_color, heading_fg_color, CodeBlockColors,
    CodeBlockTheme, BLOCKQUOTE_MARKER, BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO,
    CHECKBOX_UNCHECKED, HEADING_ICONS, HORIZONTAL_RULE_CHAR,
};
pub use enums::{
    CheckboxState, CodeBlockBorderKind, ColumnAlignment, ElementKind, TableBorderKind, TextSegment,
};
pub use methods::{render, render_with_options, RenderOptions};

/// A single markdown element that can be rendered to ratatui.
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
