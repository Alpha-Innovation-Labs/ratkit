//! Constructor for Minimap.

use super::super::{Minimap, MinimapConfig};

impl<'a> Minimap<'a> {
    /// Create a new Minimap with the given content.
    ///
    /// # Arguments
    ///
    /// * `content` - The text content to render as a minimap
    ///
    /// # Returns
    ///
    /// A new `Minimap` instance with default configuration.
    pub fn new(content: &'a str) -> Self {
        let total_lines = content.lines().count();
        Self {
            content,
            width: 10,
            viewport_start: 0,
            viewport_end: 0,
            total_lines,
            config: MinimapConfig::default(),
        }
    }
}
