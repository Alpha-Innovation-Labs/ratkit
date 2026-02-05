//! Color resolution methods for [`AppTheme`].

use ratatui::style::Color;

use crate::services::theme::AppTheme;

impl AppTheme {
    /// Gets a semantic color by name.
    ///
    /// This method allows looking up theme colors by their semantic name,
    /// which is useful for dynamic color resolution from configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The semantic name of the color (e.g., "primary", "error")
    ///
    /// # Returns
    ///
    /// `Some(Color)` if the name matches a known semantic color,
    /// `None` otherwise.
    ///
    /// # Supported Names
    ///
    /// - UI: `primary`, `secondary`, `accent`, `error`, `warning`, `success`, `info`
    /// - Text: `text`, `text_muted`, `selected_text`
    /// - Background: `background`, `background_panel`, `background_element`, `background_menu`
    /// - Border: `border`, `border_active`, `border_subtle`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::services::theme::AppTheme;
    ///
    /// let theme = AppTheme::default();
    /// let primary = theme.get_color("primary");
    /// assert!(primary.is_some());
    /// ```
    pub fn get_color(&self, name: &str) -> Option<Color> {
        match name {
            // UI colors
            "primary" => Some(self.primary),
            "secondary" => Some(self.secondary),
            "accent" => Some(self.accent),
            "error" => Some(self.error),
            "warning" => Some(self.warning),
            "success" => Some(self.success),
            "info" => Some(self.info),
            // Text colors
            "text" => Some(self.text),
            "text_muted" | "textMuted" => Some(self.text_muted),
            "selected_text" | "selectedText" => Some(self.selected_text),
            // Background colors
            "background" => Some(self.background),
            "background_panel" | "backgroundPanel" => Some(self.background_panel),
            "background_element" | "backgroundElement" => Some(self.background_element),
            "background_menu" | "backgroundMenu" => Some(self.background_menu),
            // Border colors
            "border" => Some(self.border),
            "border_active" | "borderActive" => Some(self.border_active),
            "border_subtle" | "borderSubtle" => Some(self.border_subtle),
            _ => None,
        }
    }
}
