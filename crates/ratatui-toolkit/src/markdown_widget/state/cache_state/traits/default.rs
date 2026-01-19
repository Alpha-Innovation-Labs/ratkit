//! Default trait implementation for CacheState.

use crate::markdown_widget::state::cache_state::CacheState;

impl Default for CacheState {
    fn default() -> Self {
        Self::new()
    }
}
