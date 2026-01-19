//! Set parsed cache method for CacheState.

use crate::markdown_widget::state::cache_state::{CacheState, ParsedCache};

impl CacheState {
    /// Set the parsed cache.
    pub fn set_parsed(&mut self, cache: ParsedCache) {
        self.parsed = Some(cache);
    }
}
