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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_string_source() {
        let source = MarkdownSource::from_string("# Hello");
        assert_eq!(source.content(), "# Hello");
        assert!(source.is_string());
        assert!(!source.is_file());
        assert!(source.path().is_none());
    }

    #[test]
    fn test_file_source() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# From File").unwrap();

        let source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(source.content().contains("# From File"));
        assert!(source.is_file());
        assert!(!source.is_string());
        assert!(source.path().is_some());
    }

    #[test]
    fn test_reload() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# Original").unwrap();

        let mut source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(source.content().contains("# Original"));

        // Overwrite file content
        temp.reopen().unwrap();
        let mut file = std::fs::File::create(temp.path()).unwrap();
        writeln!(file, "# Modified").unwrap();

        // Reload and verify
        assert!(source.reload().unwrap());
        assert!(source.content().contains("# Modified"));
    }

    #[test]
    fn test_reload_no_change() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "# Same").unwrap();

        let mut source = MarkdownSource::from_file(temp.path()).unwrap();
        assert!(!source.reload().unwrap()); // No change
    }

    #[test]
    fn test_string_reload_noop() {
        let mut source = MarkdownSource::from_string("# Static");
        assert!(!source.reload().unwrap());
    }

    #[test]
    fn test_set_content() {
        let mut source = MarkdownSource::from_string("# Old");
        assert!(source.set_content("# New"));
        assert_eq!(source.content(), "# New");
        assert!(!source.set_content("# New")); // No change
    }

    #[test]
    fn test_from_impls() {
        let source: MarkdownSource = "# Hello".into();
        assert_eq!(source.content(), "# Hello");

        let source: MarkdownSource = String::from("# World").into();
        assert_eq!(source.content(), "# World");
    }
}
