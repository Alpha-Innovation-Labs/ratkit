//! Set code block theme method for DisplaySettings.

use crate::widgets::markdown_widget::foundation::elements::CodeBlockTheme;
use crate::widgets::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Set the code block color theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The theme to use for code blocks.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_code_block_theme(&mut self, theme: CodeBlockTheme) -> bool {
        if self.code_block_theme != theme {
            self.code_block_theme = theme;
            true
        } else {
            false
        }
    }
}
