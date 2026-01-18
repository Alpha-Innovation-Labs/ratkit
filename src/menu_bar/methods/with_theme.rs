//! Method to apply an AppTheme to the MenuBar.

use crate::menu_bar::MenuBar;
use crate::theme::AppTheme;
use ratatui::style::{Modifier, Style};

impl MenuBar {
    /// Applies theme colors to the menu bar.
    ///
    /// This method configures all menu bar styles based on the provided theme,
    /// ensuring consistent styling across the application.
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
    /// use ratatui_toolkit::{MenuBar, MenuItem, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let menu_bar = MenuBar::new(vec![
    ///     MenuItem::new("File", 0),
    ///     MenuItem::new("Edit", 1),
    /// ])
    /// .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.normal_style = Style::default().fg(theme.text);
        self.selected_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.hover_style = Style::default().fg(theme.secondary);
        self.selected_hover_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self
    }
}
