//! Set render cache method for CacheState.

use crate::markdown_widget::state::cache_state::{CacheState, RenderCache};

impl CacheState {
    /// Set the render cache.
    pub fn set_render(&mut self, cache: RenderCache) {
        self.render = Some(cache);
    }
}
