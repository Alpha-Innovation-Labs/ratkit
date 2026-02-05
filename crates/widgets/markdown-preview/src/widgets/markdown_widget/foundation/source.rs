//! Markdown content source abstraction.
//!
//! Provides a unified interface for loading markdown content from either
//! a string or a file path, with support for auto-reload when the file changes.


use std::path::PathBuf;

/// Represents the source of markdown content.
///
/// This enum provides a unified interface for working with markdown content
/// that can come from either a static string or a file on disk.
#[derive(Debug, Clone)]
pub enum MarkdownSource {
    /// Static string content (no auto-reload support).
    String(String),

    /// File-based content with auto-reload support.
    File {
        /// Path to the markdown file.
        path: PathBuf,
        /// Cached content from the file.
        content: String,
    },
}

/// Constructor for creating a `MarkdownSource` from a file.

use std::fs;
use std::io;
use std::path::Path;


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


/// Constructor for creating a `MarkdownSource` from a string.


impl MarkdownSource {
    /// Create a new `MarkdownSource` from a string.
    ///
    /// # Arguments
    /// * `s` - The markdown string content.
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }
}


/// Method to get the content of a `MarkdownSource`.


impl MarkdownSource {
    /// Get the current content of the markdown source.
    ///
    /// For string sources, this returns the original string.
    /// For file sources, this returns the cached content.
    pub fn content(&self) -> &str {
        match self {
            Self::String(s) => s,
            Self::File { content, .. } => content,
        }
    }
}


/// Method to check if a `MarkdownSource` is file-based.


impl MarkdownSource {
    /// Check if this source is file-based.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }
}


/// Method to check if a `MarkdownSource` is string-based.


impl MarkdownSource {
    /// Check if this source is string-based.
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
}


/// Method to get the file path of a `MarkdownSource`.



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


/// Method to reload content from a file.



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


/// Method to set content directly on a `MarkdownSource`.


impl MarkdownSource {
    /// Set the content directly (for string sources).
    ///
    /// This is useful for updating string-based sources programmatically.
    /// For file sources, this updates the cached content but does not write to disk.
    ///
    /// Returns `true` if the content was changed.
    pub fn set_content(&mut self, new_content: impl Into<String>) -> bool {
        let new_content = new_content.into();
        match self {
            Self::String(content) => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
            Self::File { content, .. } => {
                if *content != new_content {
                    *content = new_content;
                    true
                } else {
                    false
                }
            }
        }
    }
}


/// Default trait implementation for `MarkdownSource`.


impl Default for MarkdownSource {
    fn default() -> Self {
        Self::String(String::new())
    }
}


/// From<&str> trait implementation for `MarkdownSource`.


impl From<&str> for MarkdownSource {
    fn from(s: &str) -> Self {
        Self::from_string(s)
    }
}


/// From<String> trait implementation for `MarkdownSource`.


impl From<String> for MarkdownSource {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

