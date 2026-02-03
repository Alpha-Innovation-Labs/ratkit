//! Invalidate render cache method for CacheState.

use crate::widgets::markdown_widget::state::cache_state::CacheState;

impl CacheState {
    /// Invalidate only the render cache.
    ///
    /// Call this when width changes but content is the same.
    pub fn invalidate_render(&mut self) {
        self.render = None;
    }
}
