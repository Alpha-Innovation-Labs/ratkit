//! Constructor for MarkdownElement.

use crate::markdown_widget::foundation::elements::enums::ElementKind;
use crate::markdown_widget::foundation::elements::MarkdownElement;

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
