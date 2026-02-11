/// Markdown element for markdown rendering.
///
/// Represents a single markdown element that can be rendered to ratatui.
pub mod blockquote;
pub mod code_block;
pub mod constants;
pub mod enums;
pub mod expandable;
pub mod frontmatter;
pub mod heading;
pub mod horizontal_rule;
pub mod list_item;
pub mod paragraph;
pub mod render;
pub mod table;
pub mod text;

pub use constants::{
    get_language_icon, get_link_icon, heading_bg_color, heading_fg_color, CodeBlockColors,
    CodeBlockTheme, BLOCKQUOTE_MARKER, BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO,
    CHECKBOX_UNCHECKED, HEADING_ICONS, HORIZONTAL_RULE_CHAR,
};
pub use enums::{
    CheckboxState, CodeBlockBorderKind, ColumnAlignment, ElementKind, TableBorderKind, TextSegment,
};
pub use render::{render, render_with_options, RenderOptions};

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

/// Constructor for MarkdownElement.
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
