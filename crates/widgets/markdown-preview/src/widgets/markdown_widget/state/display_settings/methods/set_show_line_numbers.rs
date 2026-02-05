//! Set show line numbers method for DisplaySettings.

use crate::widgets::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Enable or disable line numbers in code blocks.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_line_numbers(&mut self, show: bool) -> bool {
        if self.show_line_numbers != show {
            self.show_line_numbers = show;
            true
        } else {
            false
        }
    }
}
