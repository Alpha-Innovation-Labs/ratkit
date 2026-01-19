//! Render cache getter for CacheState.

use crate::markdown_widget::state::cache_state::{CacheState, RenderCache};

impl CacheState {
    /// Get a reference to the render cache if it exists.
    ///
    /// The render cache contains the rendered lines from the last render operation.
    /// This is useful for extracting text for copy operations.
    pub fn render_cache(&self) -> Option<&RenderCache> {
        self.render.as_ref()
    }
}
