//! TOC state constructor for MarkdownWidget.

use crate::widgets::markdown_widget::state::toc_state::TocState;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC state for the widget.
    ///
    /// When a TOC state is provided, the widget can use it for TOC rendering
    /// and navigation.
    ///
    /// # Arguments
    ///
    /// * `toc_state` - The TOC state containing entries and hover information
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn with_toc_state(mut self, toc_state: TocState) -> Self {
        self.toc_state = Some(toc_state);
        self
    }
}
