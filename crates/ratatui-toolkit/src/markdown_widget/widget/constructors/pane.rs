//! Constructor for pane configuration.

use crate::markdown_widget::widget::MarkdownWidget;
use crate::primitives::pane::Pane;

impl<'a> MarkdownWidget<'a> {
    /// Configure the Pane that wraps the widget.
    ///
    /// When `has_pane` is true (default), the widget is wrapped in this Pane.
    /// Use this to customize the pane's title, icon, padding, border style, etc.
    ///
    /// # Arguments
    ///
    /// * `pane` - The Pane configuration to use
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane(mut self, pane: Pane<'a>) -> Self {
        self.pane = Some(pane);
        self
    }
}
