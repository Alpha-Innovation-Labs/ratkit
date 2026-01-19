//! From<&str> trait implementation for `MarkdownSource`.

use super::super::MarkdownSource;

impl From<&str> for MarkdownSource {
    fn from(s: &str) -> Self {
        Self::from_string(s)
    }
}
