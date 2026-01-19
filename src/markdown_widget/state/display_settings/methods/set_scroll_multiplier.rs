//! Set scroll multiplier method for DisplaySettings.

use crate::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Set the scroll multiplier (lines per scroll tick).
    ///
    /// # Arguments
    ///
    /// * `multiplier` - Number of lines to scroll per tick.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_scroll_multiplier(&mut self, multiplier: usize) -> bool {
        if self.scroll_multiplier != multiplier {
            self.scroll_multiplier = multiplier;
            true
        } else {
            false
        }
    }

    /// Get the current scroll multiplier.
    pub fn scroll_multiplier(&self) -> usize {
        self.scroll_multiplier
    }
}
