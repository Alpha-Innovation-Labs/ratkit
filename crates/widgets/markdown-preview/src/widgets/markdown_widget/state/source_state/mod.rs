//! Source state for markdown widget.
//!
//! Manages markdown content source - either from a string or a file.

pub mod constructors;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use std::path::PathBuf;

use crate::services::file_watcher::FileWatcher;
use crate::widgets::markdown_widget::foundation::source::MarkdownSource;

/// Source state for markdown content management.
///
/// Manages the markdown source (string or file) and tracks line count.
#[derive(Debug)]
pub struct SourceState {
    /// Optional markdown source (string or file-based).
    source: Option<MarkdownSource>,
    /// Source file line count (for accurate status bar display).
    pub line_count: usize,
    /// Optional watcher for file-based sources.
    watcher: Option<FileWatcher>,
    /// Cached watched path (for re-initializing watchers).
    watch_path: Option<PathBuf>,
}

impl Clone for SourceState {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            line_count: self.line_count,
            watcher: None,
            watch_path: self.watch_path.clone(),
        }
    }
}
