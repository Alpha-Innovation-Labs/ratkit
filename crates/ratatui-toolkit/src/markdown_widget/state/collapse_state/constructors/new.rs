//! Constructor for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;
use std::collections::HashMap;

impl CollapseState {
    /// Create a new collapse state with no collapsed sections.
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            hierarchy: HashMap::new(),
        }
    }
}
