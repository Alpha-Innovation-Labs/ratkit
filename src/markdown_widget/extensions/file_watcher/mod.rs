//! File watching for markdown auto-reload.
//!
//! Provides a non-blocking file watcher that can detect changes to markdown
//! files and notify the application to reload content.

pub mod constructors;
mod helpers;
mod methods;
mod traits;

use notify::{Event, RecommendedWatcher};
use std::sync::mpsc::Receiver;

pub use constructors::*;
pub use methods::*;

/// A file watcher for detecting changes to markdown files.
///
/// Uses the `notify` crate to watch files for modifications and provides
/// a non-blocking interface to check for changes in an event loop.
///
/// # Example
/// ```no_run
/// use ratatui_toolkit::markdown_widget::extensions::file_watcher::MarkdownFileWatcher;
/// use ratatui_toolkit::markdown_widget::foundation::source::MarkdownSource;
/// use std::path::Path;
///
/// let mut source = MarkdownSource::from_file("README.md").unwrap();
/// let mut watcher = MarkdownFileWatcher::new().unwrap();
/// watcher.watch(source.path().unwrap()).unwrap();
///
/// // In your event loop:
/// loop {
///     if watcher.check_for_changes() {
///         if source.reload().unwrap() {
///             // Content changed, update your UI
///         }
///     }
///     // ... rest of your event loop
///     # break;
/// }
/// ```
pub struct MarkdownFileWatcher {
    /// The underlying file system watcher.
    pub(crate) watcher: RecommendedWatcher,
    /// Receiver for file change events.
    pub(crate) rx: Receiver<Result<Event, notify::Error>>,
}
