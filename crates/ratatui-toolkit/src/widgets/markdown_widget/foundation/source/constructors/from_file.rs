//! Constructor for creating a `MarkdownSource` from a file.

use std::fs;
use std::io;
use std::path::Path;

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Create a new `MarkdownSource` from a file path.
    ///
    /// Reads the file content immediately and caches it.
    ///
    /// # Arguments
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)?;
        Ok(Self::File { path, content })
    }
}
