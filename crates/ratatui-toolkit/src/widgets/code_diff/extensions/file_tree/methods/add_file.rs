//! Method for adding a file to an existing tree.

use crate::primitives::tree_view::TreeNode;
use crate::widgets::code_diff::diff_file_tree::DiffFileEntry;
use crate::widgets::code_diff::diff_file_tree::DiffFileTree;
use crate::widgets::code_diff::diff_file_tree::FileStatus;

impl DiffFileTree {
    /// Adds a file to the tree, creating intermediate directories as needed.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path (e.g., "src/utils/helper.rs")
    /// * `status` - The modification status
    pub fn add_file(&mut self, path: &str, status: FileStatus) {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return;
        }

        if parts.len() == 1 {
            // Root-level file
            let entry = DiffFileEntry::file(parts[0], path, status);
            self.nodes.push(TreeNode::new(entry));
        } else {
            // File in a subdirectory
            let root_dir = parts[0];

            // Find or create the root directory
            let dir_idx = self
                .nodes
                .iter()
                .position(|n| n.data.is_dir && n.data.name == root_dir);

            if let Some(idx) = dir_idx {
                // Add to existing directory
                add_to_directory(&mut self.nodes[idx], &parts[1..], path, status, root_dir);
            } else {
                // Create new directory tree
                let mut dir_node = TreeNode::with_children(
                    DiffFileEntry::directory(root_dir, root_dir),
                    Vec::new(),
                );
                add_to_directory(&mut dir_node, &parts[1..], path, status, root_dir);
                self.nodes.push(dir_node);
            }
        }

        // Re-sort: directories first, then alphabetically
        self.nodes.sort_by(|a, b| {
            let a_is_dir = a.data.is_dir;
            let b_is_dir = b.data.is_dir;
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
            }
        });
    }
}

/// Recursively adds a file to a directory node.
fn add_to_directory(
    node: &mut TreeNode<DiffFileEntry>,
    remaining_parts: &[&str],
    full_path: &str,
    status: FileStatus,
    current_path: &str,
) {
    if remaining_parts.is_empty() {
        return;
    }

    if remaining_parts.len() == 1 {
        // This is the file
        let file_name = remaining_parts[0];
        let entry = DiffFileEntry::file(file_name, full_path, status);
        node.children.push(TreeNode::new(entry));
        node.expandable = true;
    } else {
        // This is a subdirectory
        let subdir_name = remaining_parts[0];
        let subdir_path = format!("{}/{}", current_path, subdir_name);

        // Find or create the subdirectory
        let subdir_idx = node
            .children
            .iter()
            .position(|n| n.data.is_dir && n.data.name == subdir_name);

        if let Some(idx) = subdir_idx {
            add_to_directory(
                &mut node.children[idx],
                &remaining_parts[1..],
                full_path,
                status,
                &subdir_path,
            );
        } else {
            let mut subdir_node = TreeNode::with_children(
                DiffFileEntry::directory(subdir_name, &subdir_path),
                Vec::new(),
            );
            add_to_directory(
                &mut subdir_node,
                &remaining_parts[1..],
                full_path,
                status,
                &subdir_path,
            );
            node.children.push(subdir_node);
            node.expandable = true;
        }
    }

    // Sort children: directories first, then alphabetically
    node.children.sort_by(|a, b| {
        let a_is_dir = a.data.is_dir;
        let b_is_dir = b.data.is_dir;
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        }
    });
}
