//! Constructor for ParsedCache.

use crate::markdown_widget::foundation::elements::MarkdownElement;
use crate::markdown_widget::state::scroll_manager::parsed_cache::ParsedCache;

impl ParsedCache {
    /// Create a new parsed cache.
    ///
    /// # Arguments
    ///
    /// * `content_hash` - Hash of the content that was parsed.
    /// * `elements` - Parsed markdown elements.
    pub fn new(content_hash: u64, elements: Vec<MarkdownElement>) -> Self {
        Self {
            content_hash,
            elements,
        }
    }
}
