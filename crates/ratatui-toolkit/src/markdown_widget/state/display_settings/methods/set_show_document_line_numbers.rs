//! Set show document line numbers method for DisplaySettings.

use crate::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Enable or disable document-wide line numbers.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show document line numbers.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_document_line_numbers(&mut self, show: bool) -> bool {
        if self.show_document_line_numbers != show {
            self.show_document_line_numbers = show;
            true
        } else {
            false
        }
    }
}
