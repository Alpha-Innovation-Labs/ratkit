//! Set show heading collapse method for DisplaySettings.

use crate::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Enable or disable collapse indicators on headings.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show collapse indicators.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_heading_collapse(&mut self, show: bool) -> bool {
        if self.show_heading_collapse != show {
            self.show_heading_collapse = show;
            true
        } else {
            false
        }
    }
}
