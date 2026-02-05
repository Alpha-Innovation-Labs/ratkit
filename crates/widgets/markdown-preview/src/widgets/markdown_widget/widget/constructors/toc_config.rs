//! Set the TOC configuration.

use crate::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_config(mut self, config: TocConfig) -> Self {
        self.toc_config = config;
        self
    }
}
