//! Git stats state for markdown widget.
//!
//! Tracks git diff statistics for the markdown source file.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::GitStats;
use std::time::Instant;

/// Git stats state for markdown source files.
///
/// Tracks additions, modifications, and deletions from git diff.
#[derive(Debug, Clone)]
pub struct GitStatsState {
    /// Whether to show git stats in the statusline.
    show: bool,
    /// Cached git stats for the source file.
    cache: Option<GitStats>,
    /// Last time git stats were updated.
    last_update: Option<Instant>,
}

/// Constructor for GitStatsState.

impl GitStatsState {
    /// Create a new git stats state with defaults.
    pub fn new() -> Self {
        Self {
            show: false,
            cache: None,
            last_update: None,
        }
    }
}

/// Compute git diff stats for a file.
use std::path::Path;
use std::process::Command;

/// Compute git diff stats (additions, modified_files, deletions) for a specific file.
///
/// # Arguments
///
/// * `file_path` - Optional path to the file. If None, gets stats for entire repo.
///
/// # Returns
///
/// A tuple of (additions, modified_file_count, deletions).
pub fn compute_git_stats(file_path: Option<&Path>) -> (usize, usize, usize) {
    let args = match file_path {
        Some(path) => vec![
            "diff",
            "--numstat",
            "HEAD",
            "--",
            path.to_str().unwrap_or(""),
        ],
        None => vec!["diff", "--numstat", "HEAD"],
    };

    let output = Command::new("git").args(&args).output().ok();

    if let Some(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let (mut adds, mut dels, mut modified) = (0usize, 0usize, 0usize);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let line_adds = parts[0].parse::<usize>().unwrap_or(0);
                    let line_dels = parts[1].parse::<usize>().unwrap_or(0);
                    adds += line_adds;
                    dels += line_dels;
                    // Count as modified if file has changes
                    if line_adds > 0 || line_dels > 0 {
                        modified += 1;
                    }
                }
            }
            return (adds, modified, dels);
        }
    }
    (0, 0, 0)
}

/// Git stats getter method for GitStatsState.

impl GitStatsState {
    /// Get the cached git stats.
    ///
    /// # Returns
    ///
    /// The cached `GitStats` if available and git stats are enabled.
    pub fn get(&self) -> Option<GitStats> {
        if self.show {
            self.cache
        } else {
            None
        }
    }

    /// Get the cached git stats (alias for `get()`).
    ///
    /// # Returns
    ///
    /// The cached `GitStats` if available and git stats are enabled.
    pub fn git_stats(&self) -> Option<GitStats> {
        self.get()
    }

    /// Check if git stats display is enabled.
    pub fn is_enabled(&self) -> bool {
        self.show
    }
}

/// Set show git stats method for GitStatsState.

impl GitStatsState {
    /// Enable or disable git stats display.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show git stats in the statusline.
    pub fn set_show(&mut self, show: bool) {
        self.show = show;
        if show && self.cache.is_none() {
            // Trigger immediate update when enabled
            self.last_update = None;
        }
    }
}

/// Update git stats method for GitStatsState.

/// Update interval for git stats (in seconds).
const GIT_STATS_UPDATE_INTERVAL_SECS: u64 = 2;

impl GitStatsState {
    /// Update git stats if show is enabled and enough time has passed.
    ///
    /// This method should be called periodically (e.g., in the render loop).
    /// It only computes stats every 2 seconds to avoid excessive git calls.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    ///
    /// # Returns
    ///
    /// `true` if stats were updated, `false` otherwise.
    pub fn update(&mut self, source_path: Option<&Path>) -> bool {
        if !self.show {
            return false;
        }

        let should_update = match self.last_update {
            Some(last_update) => last_update.elapsed().as_secs() >= GIT_STATS_UPDATE_INTERVAL_SECS,
            None => true, // First update
        };

        if should_update {
            let (adds, modified, dels) = compute_git_stats(source_path);
            self.cache = Some(GitStats {
                additions: adds,
                modified,
                deletions: dels,
            });
            self.last_update = Some(Instant::now());
            true
        } else {
            false
        }
    }
}

/// Update git stats with GitWatcher integration.
use crate::services::git_watcher::GitWatcher;

impl GitStatsState {
    /// Update git stats using a GitWatcher for change detection.
    ///
    /// This method only computes git stats when the watcher detects
    /// that the git repository state has changed, making it more
    /// efficient than time-based polling.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    /// * `watcher` - A mutable reference to the GitWatcher.
    ///
    /// # Returns
    ///
    /// `true` if stats were updated, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ratatui_toolkit::markdown_widget::state::git_stats::GitStatsState;
    /// use ratatui_toolkit::services::git_watcher::GitWatcher;
    /// use std::path::Path;
    ///
    /// let mut git_stats = GitStatsState::new();
    /// git_stats.set_show(true);
    ///
    /// let mut watcher = GitWatcher::new().unwrap();
    /// watcher.watch(Path::new(".")).unwrap();
    ///
    /// // In your event loop:
    /// if git_stats.update_if_changed(Some(Path::new("README.md")), &mut watcher) {
    ///     println!("Git stats updated!");
    /// }
    /// ```
    pub fn update_if_changed(
        &mut self,
        source_path: Option<&Path>,
        watcher: &mut GitWatcher,
    ) -> bool {
        if !self.show {
            return false;
        }

        // Only update if the watcher detected changes
        if watcher.check_for_changes() {
            let (adds, modified, dels) = compute_git_stats(source_path);
            self.cache = Some(GitStats {
                additions: adds,
                modified,
                deletions: dels,
            });
            self.last_update = Some(Instant::now());
            true
        } else {
            false
        }
    }

    /// Force update git stats immediately.
    ///
    /// This bypasses both time-based and watcher-based checks
    /// and immediately computes git stats.
    ///
    /// # Arguments
    ///
    /// * `source_path` - The path to the source file, if any.
    pub fn force_update(&mut self, source_path: Option<&Path>) {
        let (adds, modified, dels) = compute_git_stats(source_path);
        self.cache = Some(GitStats {
            additions: adds,
            modified,
            deletions: dels,
        });
        self.last_update = Some(Instant::now());
    }
}

/// Default trait implementation for GitStatsState.

impl Default for GitStatsState {
    fn default() -> Self {
        Self::new()
    }
}
