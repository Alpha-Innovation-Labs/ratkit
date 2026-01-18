//! From<String> trait implementation for `MarkdownSource`.

use crate::markdown_renderer::MarkdownSource;

impl From<String> for MarkdownSource {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}
