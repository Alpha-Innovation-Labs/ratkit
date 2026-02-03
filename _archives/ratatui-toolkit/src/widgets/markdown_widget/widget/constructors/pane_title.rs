//! Constructor for pane title.

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the title for the Pane that wraps the widget.
    ///
    /// This is typically the filename being displayed.
    /// Only used when `has_pane` is true (default).
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display in the pane's title bar
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane_title(mut self, title: impl Into<String>) -> Self {
        self.pane_title = Some(title.into());
        self
    }
}
