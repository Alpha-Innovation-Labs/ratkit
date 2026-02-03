//! Method for getting the path of the selected item.

use crate::widgets::code_diff::diff_file_tree::DiffFileTree;

impl DiffFileTree {
    /// Returns the full path of the currently selected item.
    ///
    /// # Returns
    ///
    /// `Some(path)` if an item is selected, `None` if the tree is empty.
    #[must_use]
    pub fn selected_path(&self) -> Option<String> {
        let path = self.state.selected_path.as_ref()?;
        let node = self.get_node_at_path(path)?;
        Some(node.data.full_path.clone())
    }
}
