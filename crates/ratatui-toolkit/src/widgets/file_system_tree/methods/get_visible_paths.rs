use crate::primitives::tree_view::{get_visible_paths, TreeViewState};
use crate::widgets::file_system_tree::FileSystemTree;

impl<'a> FileSystemTree<'a> {
    pub fn get_visible_paths(&self, state: &TreeViewState) -> Vec<Vec<usize>> {
        get_visible_paths(&self.nodes, state)
    }
}
