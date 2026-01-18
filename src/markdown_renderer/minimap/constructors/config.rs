//! Config builder method for Minimap.

use super::super::{Minimap, MinimapConfig};

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
