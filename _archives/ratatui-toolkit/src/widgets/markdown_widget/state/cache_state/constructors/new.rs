//! Constructor for CacheState.

use crate::widgets::markdown_widget::state::cache_state::CacheState;

impl CacheState {
    /// Create a new cache state with empty caches.
    pub fn new() -> Self {
        Self {
            parsed: None,
            render: None,
        }
    }
}
