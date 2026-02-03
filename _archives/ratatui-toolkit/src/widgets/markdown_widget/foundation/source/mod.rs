//! Markdown content source abstraction.
//!
//! Provides a unified interface for loading markdown content from either
//! a string or a file path, with support for auto-reload when the file changes.

mod constructors;
mod methods;
mod traits;

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
