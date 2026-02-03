//! Builder method for setting the theme.

use super::super::super::diff_file_tree::DiffFileTree;
use crate::services::theme::AppTheme;

impl DiffFileTree {
    /// Sets the application theme for styling.
    ///
    /// This method applies the theme colors to the file tree widget,
    /// including selection colors, borders, and text colors.
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
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::DiffFileTree;
    /// use ratatui_toolkit::services::theme::AppTheme;
    ///
    /// let theme = AppTheme::default();
    /// let tree = DiffFileTree::new().with_theme(&theme);
    /// ```
    #[must_use]
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.theme = theme.clone();
        self
    }

    /// Applies a theme to the existing widget (non-consuming).
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to apply
    pub fn apply_theme(&mut self, theme: &AppTheme) {
        self.theme = theme.clone();
    }
}
