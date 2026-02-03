//! Theme application constructor for MarkdownWidget.

use crate::services::theme::AppTheme;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Applies an application theme to the widget.
    ///
    /// When a theme is applied, the widget will use theme colors for:
    /// - Statusline (mode colors, background, text)
    /// - TOC (text, active, hover, background, border colors)
    /// - Selection highlighting
    ///
    /// If no theme is set, the widget falls back to default hardcoded colors.
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to use for styling
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::{MarkdownWidget, theme::AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// // let widget = MarkdownWidget::new(content, scroll, selection, double_click)
    /// //     .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.app_theme = Some(theme.clone());
        // Apply theme colors to TOC config
        self.toc_config = self.toc_config.with_theme(theme);
        self
    }
}
