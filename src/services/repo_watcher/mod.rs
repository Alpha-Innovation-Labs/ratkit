//! Repository watcher for detecting git and working tree changes.
//!
//! Combines the git watcher (for `.git` state changes) with a file watcher
//! (for working tree edits) and provides a cached list of modified files
//! via `git status --porcelain`.
//!
//! # Example
//!
//! ```no_run
//! use crate::services::repo_watcher::RepoWatcher;
//! use std::path::Path;
//!
//! let mut watcher = RepoWatcher::new().unwrap();
//! watcher.watch(Path::new("/path/to/repo")).unwrap();
//!
//! // In your event loop:
//! if watcher.check_for_changes() {
//!     let changes = watcher.get_change_set();
//!     for path in changes.all_paths() {
//!         println!("Changed: {}", path.display());
//!     }
//! }
//! ```

mod constructors;
mod helpers;
mod methods;
mod traits;

pub use constructors::{new, with_config};

use std::path::PathBuf;

use crate::services::file_watcher::{FileWatcher, WatchConfig, WatchMode};
use crate::services::git_watcher::{GitWatchConfig, GitWatcher};

/// Configuration for the repository watcher.
#[derive(Debug, Clone)]
pub struct RepoWatchConfig {
    /// Configuration for the git watcher.
    pub git: GitWatchConfig,
    /// Configuration for the file watcher.
    pub files: WatchConfig,
    /// Whether to include untracked files in the change set.
    pub include_untracked: bool,
}

impl Default for RepoWatchConfig {
    fn default() -> Self {
        Self {
            git: GitWatchConfig::default(),
            files: WatchConfig {
                mode: WatchMode::Recursive,
                debounce_ms: 200,
            },
            include_untracked: true,
        }
    }
}

impl RepoWatchConfig {
    /// Create a new config with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the git watcher configuration.
    pub fn git_config(mut self, config: GitWatchConfig) -> Self {
        self.git = config;
        self
    }

    /// Set the file watcher configuration.
    pub fn file_config(mut self, config: WatchConfig) -> Self {
        self.files = config;
        self
    }

    /// Control whether untracked files are included in the change set.
    pub fn include_untracked(mut self, include: bool) -> Self {
        self.include_untracked = include;
        self
    }
}

/// Status for a file reported by git.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitFileStatus {
    /// File was added or copied.
    Added,
    /// File was modified.
    Modified,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed,
    /// File is untracked.
    Untracked,
}

/// Collection of git changes discovered for a repository.
///
/// All paths are repository-relative.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GitChangeSet {
    /// Added files.
    pub added: Vec<PathBuf>,
    /// Modified files.
    pub modified: Vec<PathBuf>,
    /// Deleted files.
    pub deleted: Vec<PathBuf>,
    /// Renamed files (new paths).
    pub renamed: Vec<PathBuf>,
    /// Untracked files.
    pub untracked: Vec<PathBuf>,
}

impl GitChangeSet {
    /// Returns true if no changes are present.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty()
            && self.modified.is_empty()
            && self.deleted.is_empty()
            && self.renamed.is_empty()
            && self.untracked.is_empty()
    }

    /// Returns all changed paths in a single list.
    pub fn all_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        paths.extend(self.added.iter().cloned());
        paths.extend(self.modified.iter().cloned());
        paths.extend(self.deleted.iter().cloned());
        paths.extend(self.renamed.iter().cloned());
        paths.extend(self.untracked.iter().cloned());
        paths
    }
}

/// A watcher that combines git and working tree signals.
pub struct RepoWatcher {
    /// Underlying git watcher for `.git` events.
    pub(crate) git_watcher: GitWatcher,
    /// Underlying file watcher for working tree events.
    pub(crate) file_watcher: FileWatcher,
    /// Configuration for this watcher.
    pub(crate) config: RepoWatchConfig,
    /// Path to the repository root being watched.
    pub(crate) repo_path: Option<PathBuf>,
    /// Cached change set from the last update.
    pub(crate) change_set: GitChangeSet,
    /// Whether a refresh is pending.
    pub(crate) has_pending_changes: bool,
}
