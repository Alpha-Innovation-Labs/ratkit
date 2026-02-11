//! Method to apply an AppTheme to the Pane.
//!
//! NOTE: This method requires the `markdown-preview` feature flag and depends on
//! the markdown_preview services which provides the AppTheme type.

use ratatui::style::Style;

#[cfg(feature = "markdown-preview")]
use crate::widgets::markdown_preview::services::theme::AppTheme;

impl<'a> super::Pane<'a> {
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
    /// # #[cfg(feature = "markdown-preview")]
    /// use crate::primitives::pane::Pane;
    /// # #[cfg(feature = "markdown-preview")]
    /// use crate::widgets::markdown_preview::services::theme::AppTheme;
    ///
    /// # #[cfg(feature = "markdown-preview")]
    /// let theme = AppTheme::default();
    /// # #[cfg(feature = "markdown-preview")]
    /// let pane = Pane::new("My Panel")
    ///     .with_theme(&theme);
    /// ```
    #[cfg(feature = "markdown-preview")]
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.border_style = Style::default().fg(theme.border);
        self.footer_style = Style::default().fg(theme.text_muted);
        self
    }
}
