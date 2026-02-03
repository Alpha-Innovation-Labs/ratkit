//! Default trait implementation for DisplaySettings.

use crate::widgets::markdown_widget::state::display_settings::DisplaySettings;

impl Default for DisplaySettings {
    fn default() -> Self {
        Self::new()
    }
}
