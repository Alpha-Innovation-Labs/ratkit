//! Vim keybinding state for markdown widget.
//!
//! Tracks state for vim-style keyboard navigation.

use std::time::Instant;

/// Vim keybinding state.
///
/// Tracks pending keypresses for vim-style multi-key commands.
#[derive(Debug, Clone)]
pub struct VimState {
    /// Pending 'g' keypress time for vim-style gg (go to top).
    pending_g_time: Option<Instant>,
}

/// Constructor for VimState.

impl VimState {
    /// Create a new vim state with defaults.
    pub fn new() -> Self {
        Self {
            pending_g_time: None,
        }
    }
}

/// Check pending gg method for VimState.
use std::time::Duration;


/// Timeout for 'gg' command (in milliseconds).
const GG_TIMEOUT_MS: u64 = 500;

impl VimState {
    /// Check if there's a pending 'g' that would complete a 'gg' command.
    ///
    /// This checks if 'g' was pressed recently enough to form a 'gg' command.
    /// If valid, clears the pending state and returns true.
    ///
    /// # Returns
    ///
    /// `true` if 'gg' command should be executed, `false` otherwise.
    pub fn check_pending_gg(&mut self) -> bool {
        if let Some(pending_time) = self.pending_g_time {
            if pending_time.elapsed() < Duration::from_millis(GG_TIMEOUT_MS) {
                self.pending_g_time = None;
                return true;
            }
        }
        false
    }
}

/// Clear pending g method for VimState.

impl VimState {
    /// Clear any pending 'g' keypress.
    pub fn clear_pending_g(&mut self) {
        self.pending_g_time = None;
    }
}

/// Set pending g method for VimState.


impl VimState {
    /// Set a pending 'g' keypress for potential 'gg' command.
    pub fn set_pending_g(&mut self) {
        self.pending_g_time = Some(Instant::now());
    }
}

/// Default trait implementation for VimState.

impl Default for VimState {
    fn default() -> Self {
        Self::new()
    }
}
