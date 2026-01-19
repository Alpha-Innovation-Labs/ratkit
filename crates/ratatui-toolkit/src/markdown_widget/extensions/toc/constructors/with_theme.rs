//! Theme application for TocConfig.

use ratatui::style::Style;

use crate::markdown_widget::extensions::toc::enums::TocConfig;

impl TocConfig {
    /// Creates a TocConfig with colors derived from the application theme.
    ///
    /// This applies theme colors to:
    /// - Text style (using markdown text color)
    /// - Active style (using theme primary color)
    /// - Hover style (using theme accent with background_element)
    /// - Background style (using theme background_panel)
    /// - Line style (using theme border color)
    /// - Active line style (using theme text color)
    /// - Border style (using theme border_active color)
    /// - Title style (using theme primary color)
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
    /// use ratatui_toolkit::{theme::AppTheme, markdown_widget::extensions::toc::TocConfig};
    ///
    /// let theme = AppTheme::default();
    /// let config = TocConfig::default().with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &crate::services::theme::AppTheme) -> Self {
        self.text_style = Style::default().fg(theme.text_muted);
        self.active_style = Style::default().fg(theme.primary);
        self.hover_style = Style::default().fg(theme.text).bg(theme.background_element);
        self.background_style = Style::default().bg(theme.background_panel);
        self.line_style = Style::default().fg(theme.border);
        self.active_line_style = Style::default().fg(theme.text);
        self.border_style = Style::default().fg(theme.border_active);
        self.title_style = Style::default().fg(theme.primary);
        self
    }
}
