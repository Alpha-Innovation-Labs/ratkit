//! Builder method for setting the theme.

use crate::services::theme::AppTheme;
use crate::widgets::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Sets the application theme for styling.
    ///
    /// This method applies the theme colors to the diff widget, including
    /// borders, backgrounds, and text colors.
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::CodeDiff;
    /// use ratatui_toolkit::services::theme::AppTheme;
    ///
    /// let theme = AppTheme::default();
    /// let diff = CodeDiff::new().with_theme(&theme);
    /// ```
    #[must_use]
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.theme = theme.clone();
        self.file_tree = self.file_tree.with_theme(theme);
        self
    }

    /// Applies a theme to the existing widget (non-consuming).
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to apply
    pub fn apply_theme(&mut self, theme: &AppTheme) {
        self.theme = theme.clone();
        self.file_tree.apply_theme(theme);
    }
}
