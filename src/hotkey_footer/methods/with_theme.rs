//! Method to apply an AppTheme to the HotkeyFooter.

use crate::hotkey_footer::HotkeyFooter;
use crate::theme::AppTheme;

impl HotkeyFooter {
    /// Applies theme colors to the hotkey footer.
    ///
    /// This method configures the footer's key color, description color,
    /// and background color based on the provided theme.
    ///
    /// # Theme Mapping
    ///
    /// - Key color uses `theme.primary`
    /// - Description color uses `theme.text_muted`
    /// - Background color uses `theme.background`
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
    /// use ratatui_toolkit::{HotkeyFooter, HotkeyItem, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let footer = HotkeyFooter::new(vec![
    ///     HotkeyItem::new("q", "quit"),
    ///     HotkeyItem::new("?", "help"),
    /// ])
    /// .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.key_color = theme.primary;
        self.description_color = theme.text_muted;
        self.background_color = theme.background;
        self
    }
}
