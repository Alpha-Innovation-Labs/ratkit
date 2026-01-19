//! Expand all sections method for CollapseState.

use crate::markdown_widget::state::collapse_state::CollapseState;

impl CollapseState {
    /// Expand all sections.
    pub fn expand_all(&mut self) {
        let section_ids: Vec<usize> = self.sections.keys().copied().collect();
        for section_id in section_ids {
            self.sections.insert(section_id, false);
        }
    }
}
