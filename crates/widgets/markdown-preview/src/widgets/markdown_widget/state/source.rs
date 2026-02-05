//! Source state for markdown widget.
//!
//! Manages markdown content source - either from a string or a file.

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

/// Constructor for SourceState.

impl SourceState {
    /// Create a new source state with no source.
    pub fn new() -> Self {
        Self {
            source: None,
            line_count: 0,
            watcher: None,
            watch_path: None,
        }
    }
}

/// Content method for SourceState.

impl SourceState {
    /// Get the current content from the source.
    ///
    /// # Returns
    ///
    /// The markdown content, or `None` if no source is set.
    pub fn content(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.content())
    }
}

/// Is file source method for SourceState.

impl SourceState {
    /// Check if the source is file-based.
    ///
    /// # Returns
    ///
    /// `true` if the source is loaded from a file, `false` otherwise.
    pub fn is_file_source(&self) -> bool {
        self.source.as_ref().map(|s| s.is_file()).unwrap_or(false)
    }
}

/// Line count method for SourceState.

impl SourceState {
    /// Get the line count of the source content.
    ///
    /// # Returns
    ///
    /// The number of lines in the source content.
    pub fn line_count(&self) -> usize {
        self.line_count
    }
}

/// Reload content if the file watcher detected changes.

impl SourceState {
    /// Reload the source content if the watcher detected changes.
    ///
    /// Returns `Ok(true)` when content changed and was reloaded.
    pub fn reload_if_changed(&mut self) -> std::io::Result<bool> {
        let Some(watcher) = self.watcher.as_mut() else {
            return Ok(false);
        };

        if !watcher.check_for_changes() {
            return Ok(false);
        }

        self.reload_source()
    }
}

/// Reload source method for SourceState.

impl SourceState {
    /// Reload the source content from disk (for file-based sources).
    ///
    /// This re-reads the file. The caller should check the return value
    /// and invalidate caches if content changed.
    ///
    /// For string-based sources, this is a no-op.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Content changed, caller should invalidate caches.
    /// * `Ok(false)` - Content unchanged or source is string-based.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn reload_source(&mut self) -> std::io::Result<bool> {
        if let Some(ref mut source) = self.source {
            let changed = source.reload()?;
            if changed {
                self.line_count = source.content().lines().count();
            }
            Ok(changed)
        } else {
            Ok(false)
        }
    }
}

/// Set source file method for SourceState.
use std::path::Path;


impl SourceState {
    /// Set a file-based markdown source.
    ///
    /// This loads the file content and enables auto-reload support.
    /// Use `reload_source()` to check for and apply file changes.
    ///
    /// **Note:** Caller should invalidate any caches after calling this.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the markdown file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn set_source_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let source = MarkdownSource::from_file(path.as_ref())?;
        self.line_count = source.content().lines().count();
        self.source = Some(source);
        self.watch_path = Some(path.as_ref().to_path_buf());

        let mut watcher = FileWatcher::for_file()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        watcher
            .watch(path.as_ref())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        self.watcher = Some(watcher);
        Ok(())
    }
}

/// Set source string method for SourceState.

impl SourceState {
    /// Set a string-based markdown source.
    ///
    /// **Note:** Caller should invalidate any caches after calling this.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content string.
    pub fn set_source_string(&mut self, content: impl Into<String>) {
        let content_str: String = content.into();
        self.line_count = content_str.lines().count();
        self.source = Some(MarkdownSource::from_string(content_str));
        self.watcher = None;
        self.watch_path = None;
    }
}

/// Source path method for SourceState.


impl SourceState {
    /// Get the file path if this is a file-based source.
    ///
    /// # Returns
    ///
    /// The file path, or `None` if this is a string source or no source is set.
    pub fn source_path(&self) -> Option<&Path> {
        self.source.as_ref().and_then(|s| s.path())
    }
}

/// Default trait implementation for SourceState.

impl Default for SourceState {
    fn default() -> Self {
        Self::new()
    }
}
