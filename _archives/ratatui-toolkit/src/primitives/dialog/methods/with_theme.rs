//! Method to apply an AppTheme to the Dialog.

use crate::primitives::dialog::Dialog;
use crate::services::theme::AppTheme;
use ratatui::style::{Modifier, Style};

impl<'a> Dialog<'a> {
    /// Applies theme colors to the dialog.
    ///
    /// This method configures the dialog's border colors for each dialog type,
    /// as well as button styles, based on the provided theme.
    ///
    /// # Theme Mapping
    ///
    /// - Info dialogs use `theme.info`
    /// - Success dialogs use `theme.success`
    /// - Warning dialogs use `theme.warning`
    /// - Error dialogs use `theme.error`
    /// - Confirm dialogs use `theme.primary`
    /// - Dialog background uses `theme.background_panel`
    /// - Dialog text uses `theme.text`
    /// - Selected button uses `theme.selected_text` on `theme.primary` background
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
    /// use ratatui_toolkit::{Dialog, DialogType, AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// let dialog = Dialog::new("Info", "This is a message")
    ///     .dialog_type(DialogType::Info)
    ///     .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        // Set dialog type colors
        self.theme_info_color = Some(theme.info);
        self.theme_success_color = Some(theme.success);
        self.theme_warning_color = Some(theme.warning);
        self.theme_error_color = Some(theme.error);
        self.theme_confirm_color = Some(theme.primary);

        // Set dialog background and text style
        self.style = Style::default().bg(theme.background_panel).fg(theme.text);

        // Set button styles
        self.button_selected_style = Style::default()
            .fg(theme.selected_text)
            .bg(theme.primary)
            .add_modifier(Modifier::BOLD);
        self.button_style = Style::default().fg(theme.text);

        self
    }
}
