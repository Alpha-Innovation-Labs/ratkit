//! Kind of markdown element for markdown rendering.

use super::{ColumnAlignment, TextSegment};
use crate::markdown_renderer::markdown_elements::MarkdownElement;

/// Represents the kind of markdown element.
#[derive(Debug, Clone, Default)]
pub enum ElementKind {
    /// Empty line (default).
    #[default]
    Empty,
    /// Heading with level (1-6).
    Heading {
        level: u8,
        text: Vec<TextSegment>,
        /// Unique section ID for tracking collapse state (index in elements vector).
        section_id: usize,
        /// Whether this section is collapsed.
        collapsed: bool,
    },
    /// Border below heading.
    #[allow(dead_code)]
    HeadingBorder { level: u8 },
    /// Code block header with language.
    CodeBlockHeader {
        language: String,
        /// Blockquote nesting depth (0 = not in blockquote)
        blockquote_depth: usize,
    },
    /// Code block content line (plain text or syntax highlighted).
    CodeBlockContent {
        /// Plain text content
        content: String,
        /// Syntax highlighted text (if available)
        highlighted: Option<ratatui::text::Text<'static>>,
        /// Line number (1-indexed)
        line_number: usize,
        /// Blockquote nesting depth (0 = not in blockquote)
        blockquote_depth: usize,
    },
    /// Code block border (top, middle, bottom).
    CodeBlockBorder {
        kind: super::CodeBlockBorderKind,
        /// Blockquote nesting depth (0 = not in blockquote)
        blockquote_depth: usize,
    },
    /// Paragraph text with formatting.
    Paragraph(Vec<TextSegment>),
    /// List item with nesting level.
    ListItem {
        depth: usize,
        ordered: bool,
        number: Option<usize>,
        content: Vec<TextSegment>,
    },
    /// Blockquote with nesting depth.
    Blockquote {
        content: Vec<TextSegment>,
        /// Nesting depth (1 = single >, 2 = >> , etc.)
        depth: usize,
    },
    /// Table row.
    TableRow {
        cells: Vec<String>,
        is_header: bool,
        alignments: Vec<ColumnAlignment>,
    },
    /// Table border.
    TableBorder(super::TableBorderKind),
    /// Horizontal rule.
    HorizontalRule,
    /// YAML frontmatter (collapsible) - legacy single-block format.
    /// Contains the parsed fields as key-value pairs.
    Frontmatter {
        /// The frontmatter fields (key, value).
        fields: Vec<(String, String)>,
        /// Whether the frontmatter is collapsed (shows only context_id).
        collapsed: bool,
    },
    /// Frontmatter top border with collapse icon.
    FrontmatterStart {
        /// Whether the frontmatter section is collapsed.
        collapsed: bool,
        /// Context ID to show when collapsed (from frontmatter fields).
        context_id: Option<String>,
    },
    /// A single frontmatter field (key: value).
    FrontmatterField {
        /// The field key.
        key: String,
        /// The field value.
        value: String,
    },
    /// Frontmatter bottom border.
    FrontmatterEnd,
    /// Expandable content block (e.g., "Show more" / "Show less").
    Expandable {
        /// Unique ID for tracking state
        content_id: String,
        /// The content to display (already markdown elements)
        lines: Vec<MarkdownElement>,
        /// Maximum number of lines to show when collapsed
        max_lines: usize,
        /// Whether currently collapsed
        collapsed: bool,
        /// Total number of lines in the content
        total_lines: usize,
    },
    /// Show more / Show less toggle button.
    ExpandToggle {
        /// The content_id this toggle belongs to
        content_id: String,
        /// Whether in expanded state (shows "Show less") or collapsed (shows "Show more")
        expanded: bool,
        /// Number of hidden lines
        hidden_count: usize,
    },
}
