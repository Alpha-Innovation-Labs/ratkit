//! Cache for parsed markdown elements.
//!
//! This cache stores parsed markdown elements that don't depend on rendering width.

use crate::widgets::markdown_widget::foundation::elements::MarkdownElement;

/// Cache for parsed markdown (doesn't depend on width).
#[derive(Debug, Clone)]
pub struct ParsedCache {
    /// Hash of the content that was parsed.
    pub content_hash: u64,
    /// Parsed markdown elements.
    pub elements: Vec<MarkdownElement>,
}

impl ParsedCache {
    /// Create a new parsed cache.
    pub fn new(content_hash: u64, elements: Vec<MarkdownElement>) -> Self {
        Self {
            content_hash,
            elements,
        }
    }
}
