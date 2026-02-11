//! Helper functions for the markdown widget.

//! Extract plain text content from a ElementKind.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render, ElementKind, MarkdownElement, TextSegment,
};

/// Convert segments to plain text.
fn segments_to_text(segments: &[TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            TextSegment::Plain(t) => t.clone(),
            TextSegment::Bold(t) => t.clone(),
            TextSegment::Italic(t) => t.clone(),
            TextSegment::BoldItalic(t) => t.clone(),
            TextSegment::InlineCode(t) => format!("`{}`", t),
            TextSegment::Link { text, .. } => text.clone(),
            TextSegment::Strikethrough(t) => t.clone(),
            TextSegment::Html(t) => t.clone(),
            TextSegment::Checkbox(_) => String::new(),
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Extract plain text content from a ElementKind.
///
/// # Arguments
///
/// * `kind` - The element kind to extract text from
///
/// # Returns
///
/// The plain text content of the element.
pub fn element_to_plain_text(kind: &ElementKind) -> String {
    match kind {
        ElementKind::Heading { text, .. } => segments_to_text(text),
        ElementKind::Paragraph(segments) => segments_to_text(segments),
        ElementKind::ListItem { content, .. } => segments_to_text(content),
        ElementKind::Blockquote { content, .. } => segments_to_text(content),
        ElementKind::CodeBlockHeader { language, .. } => format!("```{}", language),
        ElementKind::CodeBlockContent { content, .. } => content.clone(),
        ElementKind::TableRow { cells, .. } => cells.join(" | "),
        ElementKind::Frontmatter { fields, .. } => fields
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        ElementKind::FrontmatterStart { context_id, .. } => {
            context_id.clone().unwrap_or_else(|| "---".to_string())
        }
        ElementKind::FrontmatterField { key, value } => format!("{}: {}", key, value),
        ElementKind::FrontmatterEnd => "---".to_string(),
        _ => String::new(),
    }
}

/// Get line information at a given screen position.
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
    CollapseState, ScrollState,
};

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::events::MarkdownDoubleClickEvent;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;

/// Convert ElementKind to a human-readable string.
fn element_kind_to_string(kind: &ElementKind) -> String {
    match kind {
        ElementKind::Heading { level, .. } => format!("Heading (H{})", level),
        ElementKind::HeadingBorder { .. } => "Heading Border".to_string(),
        ElementKind::CodeBlockHeader { language, .. } => {
            format!(
                "Code Block Header ({})",
                if language.is_empty() {
                    "text"
                } else {
                    language
                }
            )
        }
        ElementKind::CodeBlockContent { .. } => "Code Block Content".to_string(),
        ElementKind::CodeBlockBorder { .. } => "Code Block Border".to_string(),
        ElementKind::Paragraph(_) => "Paragraph".to_string(),
        ElementKind::ListItem { ordered, depth, .. } => {
            if *ordered {
                format!("Ordered List Item (depth {})", depth)
            } else {
                format!("Unordered List Item (depth {})", depth)
            }
        }
        ElementKind::Blockquote { depth, .. } => format!("Blockquote (depth {})", depth),
        ElementKind::TableRow { is_header, .. } => {
            if *is_header {
                "Table Header".to_string()
            } else {
                "Table Row".to_string()
            }
        }
        ElementKind::TableBorder(_) => "Table Border".to_string(),
        ElementKind::HorizontalRule => "Horizontal Rule".to_string(),
        ElementKind::Empty => "Empty".to_string(),
        ElementKind::Frontmatter { .. } => "Frontmatter".to_string(),
        ElementKind::FrontmatterStart { .. } => "Frontmatter Start".to_string(),
        ElementKind::FrontmatterField { key, .. } => format!("Frontmatter Field ({})", key),
        ElementKind::FrontmatterEnd => "Frontmatter End".to_string(),
        ElementKind::Expandable { .. } => "Expandable Content".to_string(),
        ElementKind::ExpandToggle { .. } => "Expand Toggle".to_string(),
    }
}

/// Check if a markdown element should be rendered based on collapse state.
fn should_render_line(element: &MarkdownElement, _idx: usize, collapse: &CollapseState) -> bool {
    // Headings: visible unless a parent section is collapsed (hierarchical collapse)
    if let ElementKind::Heading { section_id, .. } = &element.kind {
        // Check if any parent section is collapsed
        if let Some((_, Some(parent))) = collapse.get_hierarchy(*section_id) {
            // If parent is collapsed, this heading is hidden
            if collapse.is_section_collapsed(parent) {
                return false;
            }
        }
        return true;
    }

    // Legacy Frontmatter block is always visible
    if matches!(element.kind, ElementKind::Frontmatter { .. }) {
        return true;
    }

    // FrontmatterStart is always visible (contains collapse toggle)
    if matches!(element.kind, ElementKind::FrontmatterStart { .. }) {
        return true;
    }

    // FrontmatterField and FrontmatterEnd are hidden when frontmatter is collapsed
    if matches!(
        element.kind,
        ElementKind::FrontmatterField { .. } | ElementKind::FrontmatterEnd
    ) {
        // Frontmatter uses section_id 0 for collapse state
        if collapse.is_section_collapsed(0) {
            return false;
        }
        return true;
    }

    // Check if this element belongs to a collapsed section
    if let Some(section_id) = element.section_id {
        if collapse.is_section_collapsed(section_id) {
            return false;
        }
    }

    true
}

/// Get line information at the given screen position.
///
/// # Arguments
///
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll state
/// * `collapse` - The collapse state
///
/// # Returns
///
/// A `MarkdownDoubleClickEvent` if a line was found at the position.
pub fn get_line_at_position(
    y: usize,
    width: usize,
    content: &str,
    scroll: &ScrollState,
    collapse: &CollapseState,
) -> Option<MarkdownDoubleClickEvent> {
    let elements = render_markdown_to_elements(content, true);
    let document_y = y + scroll.scroll_offset;
    let mut visual_line_idx = 0;
    let mut logical_line_num = 0; // Track the visible logical line number (1-indexed for display)

    for (idx, element) in elements.iter().enumerate() {
        if !should_render_line(element, idx, collapse) {
            continue;
        }

        logical_line_num += 1; // Increment for each visible logical line

        let rendered = render(element, width);
        let line_count = rendered.len();

        if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
            let line_kind = element_kind_to_string(&element.kind);
            let text_content = element_to_plain_text(&element.kind);

            return Some(MarkdownDoubleClickEvent {
                line_number: logical_line_num,
                line_kind,
                content: text_content,
            });
        }

        visual_line_idx += line_count;
    }

    None
}

/// Simple hash function for content change detection.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simple hash function for content change detection.
///
/// # Arguments
///
/// * `content` - The content to hash
///
/// # Returns
///
/// A 64-bit hash of the content.
pub fn hash_content(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

/// Check if a position is within an area.
use ratatui::layout::Rect;

/// Check if a position is within an area.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
/// * `area` - The area to check against
///
/// # Returns
///
/// `true` if the position is within the area.
pub fn is_in_area(x: u16, y: u16, area: Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
