//! Flush accumulated text segments as a paragraph or blockquote.

use crate::markdown_renderer::markdown_elements::{MarkdownElement, ElementKind, TextSegment};

/// Flush accumulated segments as a paragraph or blockquote.
///
/// # Arguments
///
/// * `lines` - The vector of styled lines to append to
/// * `segments` - The accumulated text segments to flush
/// * `blockquote_depth` - The current blockquote nesting depth
/// * `section_id` - The current section ID for collapse/expand tracking
/// * `source_line` - The source line number for this content
pub fn flush_paragraph(
    lines: &mut Vec<MarkdownElement>,
    segments: &mut Vec<TextSegment>,
    blockquote_depth: usize,
    section_id: Option<usize>,
    source_line: usize,
) {
    if segments.is_empty() {
        return;
    }

    let content = std::mem::take(segments);

    if blockquote_depth > 0 {
        lines.push(MarkdownElement {
            kind: ElementKind::Blockquote {
                content,
                depth: blockquote_depth,
            },
            section_id,
            source_line,
        });
    } else {
        lines.push(MarkdownElement {
            kind: ElementKind::Paragraph(content),
            section_id,
            source_line,
        });
    }
}
