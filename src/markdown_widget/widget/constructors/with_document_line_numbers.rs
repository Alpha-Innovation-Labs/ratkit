//! Enable or disable document-wide line numbers.

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable document-wide line numbers.
    ///
    /// When enabled, line numbers will be displayed on the left side of
    /// every line in the document (not just code blocks). The line numbers
    /// use a fixed width of 6 characters: "  1 │ " to "999 │ ".
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show document-wide line numbers
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let widget = MarkdownWidget::new(content, &mut scroll, &mut selection, &mut double_click)
    ///     .with_document_line_numbers(true);
    /// ```
    pub fn with_document_line_numbers(self, show: bool) -> Self {
        self.scroll.show_document_line_numbers = show;
        self
    }
}
