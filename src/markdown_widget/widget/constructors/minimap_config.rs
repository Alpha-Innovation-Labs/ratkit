//! Set the minimap configuration.

use crate::markdown_widget::extensions::minimap::MinimapConfig;
use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Set the minimap configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The minimap configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn minimap_config(mut self, config: MinimapConfig) -> Self {
        self.minimap_config = config;
        self
    }
}
