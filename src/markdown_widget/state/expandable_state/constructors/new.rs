//! Constructor for ExpandableState.

use crate::markdown_widget::state::expandable_state::ExpandableState;
use std::collections::HashMap;

impl ExpandableState {
    /// Create a new expandable state with defaults.
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            default_max_lines: 3,
        }
    }
}
