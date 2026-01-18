//! Convert ElementKind to a human-readable string.

use crate::markdown_renderer::markdown_elements::ElementKind;

/// Convert ElementKind to a human-readable string.
///
/// # Arguments
///
/// * `kind` - The element kind to convert
///
/// # Returns
///
/// A human-readable string describing the element kind.
pub(crate) fn element_kind_to_string(kind: &ElementKind) -> String {
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
