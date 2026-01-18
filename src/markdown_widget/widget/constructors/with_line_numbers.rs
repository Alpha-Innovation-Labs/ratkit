//! Enable or disable line numbers in code blocks.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable line numbers in code blocks.
    ///
    /// When enabled, code blocks will display line numbers on the left side
    /// of each line of code.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers in code blocks
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let widget = MarkdownWidget::new(content, &mut scroll, &mut selection, &mut double_click)
    ///     .with_line_numbers(true);
    /// ```
    pub fn with_line_numbers(self, show: bool) -> Self {
        self.scroll.show_line_numbers = show;
        self
    }
}
