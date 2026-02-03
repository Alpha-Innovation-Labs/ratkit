//! Extract plain text content from a ElementKind.

use crate::widgets::markdown_widget::foundation::elements::{ElementKind, TextSegment};

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
