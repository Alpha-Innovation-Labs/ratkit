//! Config builder method for Minimap.

use crate::markdown_widget::extensions::minimap::enums::MinimapConfig;
use crate::markdown_widget::extensions::minimap::Minimap;

impl<'a> Minimap<'a> {
    /// Set the configuration for the minimap.
    ///
    /// # Arguments
    ///
    /// * `config` - The minimap configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: MinimapConfig) -> Self {
        self.config = config;
        self
    }
}
