//! Set show git stats method for GitStatsState.

use crate::markdown_widget::state::git_stats_state::GitStatsState;

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
