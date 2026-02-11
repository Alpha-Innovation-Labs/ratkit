//! Git watching service for detecting git repository changes.
//!
//! A reusable, configurable git watcher that monitors the `.git` directory
//! for changes and can be used to trigger updates only when git state changes.
//!
//! # Example
//!
//! ```no_run
//! use crate::services::git_watcher::GitWatcher;
//! use std::path::Path;
//!
//! // Create a watcher for a git repository
//! let mut watcher = GitWatcher::new().unwrap();
//! watcher.watch(Path::new("/path/to/repo")).unwrap();
//!
//! // In your event loop:
//! if watcher.check_for_changes() {
//!     println!("Git state changed!");
//! }
//! ```

mod constructors;
mod helpers;
mod methods;
mod traits;

pub use constructors::{new, with_config};

use notify::{Event, RecommendedWatcher};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

/// Configuration for the git watcher.
#[derive(Debug, Clone)]
pub struct GitWatchConfig {
    /// Debounce interval in milliseconds.
    pub debounce_ms: u64,
}

impl Default for GitWatchConfig {
    fn default() -> Self {
        Self { debounce_ms: 100 }
    }
}

impl GitWatchConfig {
    /// Create a new config with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the debounce interval in milliseconds.
    pub fn debounce_ms(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }
}

/// A git watcher for detecting git repository state changes.
///
/// Uses the `notify` crate internally to watch the `.git` directory
/// for changes. Provides a non-blocking interface suitable for use
/// in TUI event loops.
///
/// This is useful for caching git statistics and only recomputing
/// them when the git state actually changes, rather than polling
/// at a fixed interval.
pub struct GitWatcher {
    /// The underlying file system watcher.
    pub(crate) watcher: RecommendedWatcher,
    /// Receiver for file change events.
    pub(crate) rx: Receiver<Result<Event, notify::Error>>,
    /// Configuration for the watcher.
    pub(crate) config: GitWatchConfig,
    /// Path to the repository root being watched.
    pub(crate) repo_path: Option<PathBuf>,
    /// Whether changes have been detected since last check.
    pub(crate) has_pending_changes: bool,
}
