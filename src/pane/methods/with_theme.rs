//! Method to apply an AppTheme to the Pane.

use crate::pane::Pane;
use crate::services::theme::AppTheme;
use ratatui::style::Style;

impl<'a> Pane<'a> {
    /// Applies theme colors to the pane.
    ///
    /// This method configures the pane's border and footer styles
    /// based on the provided theme.
    ///
    /// # Theme Mapping
    ///
    /// - Border style uses `theme.border`
    /// - Footer style uses `theme.text_muted`
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
    /// use ratatui_toolkit::{Pane, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let pane = Pane::new("My Panel")
    ///     .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.border_style = Style::default().fg(theme.border);
        self.footer_style = Style::default().fg(theme.text_muted);
        self
    }
}
