//! Default trait implementation for CacheState.

use crate::widgets::markdown_widget::state::cache_state::CacheState;

impl Default for CacheState {
    fn default() -> Self {
        Self::new()
    }
}
