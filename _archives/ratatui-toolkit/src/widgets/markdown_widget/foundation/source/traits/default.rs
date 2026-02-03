//! Default trait implementation for `MarkdownSource`.

use super::super::MarkdownSource;

impl Default for MarkdownSource {
    fn default() -> Self {
        Self::String(String::new())
    }
}
