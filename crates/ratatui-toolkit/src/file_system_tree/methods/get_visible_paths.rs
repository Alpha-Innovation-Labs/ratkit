use crate::file_system_tree::FileSystemTree;
use crate::tree_view::{get_visible_paths, TreeViewState};

impl<'a> FileSystemTree<'a> {
    pub fn get_visible_paths(&self, state: &TreeViewState) -> Vec<Vec<usize>> {
        get_visible_paths(&self.nodes, state)
    }
}
