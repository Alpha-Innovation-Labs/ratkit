//! Get line information at a given screen position.

use crate::widgets::markdown_widget::state::{CollapseState, ScrollState};

use crate::widgets::markdown_widget::foundation::elements::{render, ElementKind, MarkdownElement};
use crate::widgets::markdown_widget::foundation::events::MarkdownDoubleClickEvent;
use crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;

use super::element_to_plain_text::element_to_plain_text;

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
