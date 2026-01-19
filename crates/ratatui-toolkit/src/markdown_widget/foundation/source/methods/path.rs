//! Method to get the file path of a `MarkdownSource`.

use std::path::Path;

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Get the file path if this is a file-based source.
    ///
    /// Returns `None` for string sources.
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::String(_) => None,
            Self::File { path, .. } => Some(path),
        }
    }
}
