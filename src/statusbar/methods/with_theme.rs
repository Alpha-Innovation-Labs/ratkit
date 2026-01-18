//! Method to apply an AppTheme to the StatusBar.

use crate::statusbar::StatusBar;
use crate::services::theme::AppTheme;
use ratatui::style::Style;

impl<'a> StatusBar<'a> {
    /// Applies theme colors to the status bar.
    ///
    /// This method configures the status bar's background and foreground
    /// colors based on the provided theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to apply
    ///
    /// # Returns
    ///
    /// Self with theme colors applied for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::{StatusBar, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let status_bar = StatusBar::new()
    ///     .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.style = Style::default().bg(theme.background_panel).fg(theme.text);
        self
    }
}
