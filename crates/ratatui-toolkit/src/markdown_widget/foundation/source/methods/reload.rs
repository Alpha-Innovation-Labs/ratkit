//! Method to reload content from a file.

use std::fs;
use std::io;

use super::super::MarkdownSource;

impl MarkdownSource {
    /// Reload the content from the file.
    ///
    /// For string sources, this is a no-op and returns `Ok(false)`.
    /// For file sources, this re-reads the file and returns `Ok(true)` if
    /// the content has changed.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read.
    pub fn reload(&mut self) -> io::Result<bool> {
        match self {
            Self::String(_) => Ok(false),
            Self::File { path, content } => {
                let new_content = fs::read_to_string(&*path)?;
                if new_content != *content {
                    *content = new_content;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
