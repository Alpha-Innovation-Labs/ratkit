//! Default trait implementation for `MarkdownSource`.

use crate::markdown_renderer::MarkdownSource;

impl Default for MarkdownSource {
    fn default() -> Self {
        Self::String(String::new())
    }
}
