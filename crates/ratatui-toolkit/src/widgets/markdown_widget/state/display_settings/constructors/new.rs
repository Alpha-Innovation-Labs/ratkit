//! Constructor for DisplaySettings.

use crate::widgets::markdown_widget::foundation::elements::CodeBlockTheme;
use crate::widgets::markdown_widget::state::display_settings::DisplaySettings;

impl DisplaySettings {
    /// Create new display settings with defaults.
    pub fn new() -> Self {
        Self {
            show_line_numbers: false,
            show_document_line_numbers: false,
            code_block_theme: CodeBlockTheme::default(),
            show_heading_collapse: false,
            scroll_multiplier: 3,
        }
    }
}
