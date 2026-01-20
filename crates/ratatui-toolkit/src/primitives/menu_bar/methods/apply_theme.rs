//! Method to apply an AppTheme to an existing MenuBar in place.

use crate::primitives::menu_bar::MenuBar;
use crate::services::theme::AppTheme;
use ratatui::style::{Modifier, Style};

impl MenuBar {
    /// Applies theme colors to the menu bar in place.
    ///
    /// Unlike `with_theme` which consumes self, this method updates
    /// the menu bar's styles without consuming it. Useful for updating
    /// the theme dynamically at runtime.
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to apply
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::{MenuBar, MenuItem, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let mut menu_bar = MenuBar::new(vec![
    ///     MenuItem::new("File", 0),
    ///     MenuItem::new("Edit", 1),
    /// ]);
    /// menu_bar.apply_theme(&theme);
    /// ```
    pub fn apply_theme(&mut self, theme: &AppTheme) {
        self.normal_style = Style::default().fg(theme.text);
        self.selected_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.hover_style = Style::default().fg(theme.secondary);
        self.selected_hover_style = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
    }
}
