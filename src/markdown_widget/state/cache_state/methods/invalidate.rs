//! Invalidate cache method for CacheState.

use crate::markdown_widget::state::cache_state::CacheState;

impl CacheState {
    /// Invalidate both parsed and render caches.
    ///
    /// Call this when content changes.
    pub fn invalidate(&mut self) {
        self.parsed = None;
        self.render = None;
    }
}
