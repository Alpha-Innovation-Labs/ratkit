//! File watching service for detecting file system changes.
//!
//! A reusable, configurable file watcher that can be used across multiple
//! components in the toolkit. Supports watching single files or entire
//! directory trees.
//!
//! # Example
//!
//! ```no_run
//! use ratatui_toolkit::services::file_watcher::{FileWatcher, WatchMode};
//! use std::path::Path;
//!
//! // Watch a single file
//! let mut watcher = FileWatcher::for_file().unwrap();
//! watcher.watch(Path::new("README.md")).unwrap();
//!
//! // In your event loop:
//! if watcher.check_for_changes() {
//!     println!("File changed!");
//! }
//!
//! // Watch a directory recursively
//! let mut dir_watcher = FileWatcher::for_directory().unwrap();
//! dir_watcher.watch(Path::new("./src")).unwrap();
//!
//! // Get which paths changed
//! let changed = dir_watcher.get_changed_paths();
//! for path in changed {
//!     println!("Changed: {}", path.display());
//! }
//! ```

mod constructors;
mod helpers;
mod methods;
mod traits;

use notify::{Event, RecommendedWatcher};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

/// Mode for file watching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WatchMode {
    /// Watch a single file (non-recursive).
    #[default]
    File,
    /// Watch a directory tree recursively.
    Recursive,
}

/// Configuration for the file watcher.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Watch mode - single file or recursive directory.
    pub mode: WatchMode,
    /// Debounce interval in milliseconds.
    pub debounce_ms: u64,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            mode: WatchMode::File,
            debounce_ms: 100,
        }
    }
}

impl WatchConfig {
    /// Create a new config with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the watch mode.
    pub fn mode(mut self, mode: WatchMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the debounce interval in milliseconds.
    pub fn debounce_ms(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }
}

/// A file watcher for detecting file system changes.
///
/// Uses the `notify` crate internally to watch files and directories
/// for changes. Provides a non-blocking interface suitable for use
/// in TUI event loops.
pub struct FileWatcher {
    /// The underlying file system watcher.
    pub(crate) watcher: RecommendedWatcher,
    /// Receiver for file change events.
    pub(crate) rx: Receiver<Result<Event, notify::Error>>,
    /// Configuration for the watcher.
    pub(crate) config: WatchConfig,
    /// Paths that have changed since last check.
    pub(crate) changed_paths: Vec<PathBuf>,
}
