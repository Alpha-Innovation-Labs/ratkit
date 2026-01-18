//! Theme application for MinimapConfig.

use ratatui::style::Style;

use crate::markdown_widget::extensions::minimap::enums::MinimapConfig;

impl MinimapConfig {
    /// Creates a MinimapConfig with colors derived from the application theme.
    ///
    /// This applies theme colors to:
    /// - Text style (using theme border color for the Braille characters)
    /// - Viewport style (using theme primary color with background_element)
    /// - Background style (using theme background_panel)
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to derive colors from
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::{theme::AppTheme, markdown_widget::extensions::minimap::MinimapConfig};
    ///
    /// let theme = AppTheme::default();
    /// let config = MinimapConfig::default().with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &crate::services::theme::AppTheme) -> Self {
        self.text_style = Style::default().fg(theme.border);
        self.viewport_style = Style::default().fg(theme.primary).bg(theme.background_element);
        self.background_style = Style::default().bg(theme.background_panel);
        self
    }
}
