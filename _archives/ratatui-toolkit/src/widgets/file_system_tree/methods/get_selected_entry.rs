use crate::primitives::tree_view::TreeViewState;
use crate::widgets::file_system_tree::{FileSystemEntry, FileSystemTree};

impl<'a> FileSystemTree<'a> {
    pub fn get_selected_entry(&self, state: &TreeViewState) -> Option<&FileSystemEntry> {
        state
            .selected_path
            .as_ref()
            .and_then(|path| self.get_entry_at_path(path))
    }
}
