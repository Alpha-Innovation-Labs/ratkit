//! Parsed cache getter for CacheState.

use crate::widgets::markdown_widget::state::cache_state::{CacheState, ParsedCache};

impl CacheState {
    /// Get a reference to the parsed cache if it exists.
    pub fn parsed_cache(&self) -> Option<&ParsedCache> {
        self.parsed.as_ref()
    }
}
