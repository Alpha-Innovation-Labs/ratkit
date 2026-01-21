//! Constructor for building a tree from a list of file paths and statuses.

use std::collections::HashMap;

use crate::primitives::tree_view::TreeNode;
use crate::widgets::code_diff::diff_file_tree::DiffFileEntry;
use crate::widgets::code_diff::diff_file_tree::DiffFileTree;
use crate::widgets::code_diff::diff_file_tree::FileStatus;

impl DiffFileTree {
    /// Creates a `DiffFileTree` from a list of (path, status) pairs.
    ///
    /// Paths are parsed to build a hierarchical directory structure.
    /// Intermediate directories are created automatically.
    ///
    /// # Arguments
    ///
    /// * `paths` - A slice of (path, status) pairs
    ///
    /// # Returns
    ///
    /// A new `DiffFileTree` with the file hierarchy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::widgets::code_diff::diff_file_tree::{DiffFileTree, FileStatus};
    ///
    /// let files = vec![
    ///     ("src/lib.rs", FileStatus::Modified),
    ///     ("src/utils/helper.rs", FileStatus::Added),
    /// ];
    ///
    /// let tree = DiffFileTree::from_paths(&files);
    /// ```
    #[must_use]
    pub fn from_paths<S: AsRef<str>>(paths: &[(S, FileStatus)]) -> Self {
        let mut tree = Self::new();

        if paths.is_empty() {
            return tree;
        }

        // Build a temporary structure to organize files by directory
        let mut dir_map: HashMap<String, Vec<(String, FileStatus)>> = HashMap::new();

        for (path, status) in paths {
            let path = path.as_ref();
            let parts: Vec<&str> = path.split('/').collect();

            if parts.len() == 1 {
                // Root-level file
                dir_map
                    .entry(String::new())
                    .or_default()
                    .push((path.to_string(), *status));
            } else {
                // File in a subdirectory - add to root level directory
                let root_dir = parts[0].to_string();
                dir_map
                    .entry(root_dir)
                    .or_default()
                    .push((path.to_string(), *status));
            }
        }

        // Build tree nodes
        let mut nodes = Vec::new();

        // First add root-level files
        if let Some(root_files) = dir_map.get("") {
            for (path, status) in root_files {
                let name = path.split('/').last().unwrap_or(path);
                let entry = DiffFileEntry::file(name, path, *status);
                nodes.push(TreeNode::new(entry));
            }
        }

        // Then add directories with their contents
        let mut dir_names: Vec<_> = dir_map.keys().filter(|k| !k.is_empty()).collect();
        dir_names.sort();

        for dir_name in dir_names {
            if let Some(files) = dir_map.get(dir_name) {
                let dir_node = build_directory_node(dir_name, files);
                nodes.push(dir_node);
            }
        }

        // Sort: directories first, then alphabetically
        nodes.sort_by(|a, b| {
            let a_is_dir = a.data.is_dir;
            let b_is_dir = b.data.is_dir;
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
            }
        });

        tree.nodes = nodes;

        // Select first item if available
        if !tree.nodes.is_empty() {
            tree.state.select(vec![0]);
            // Expand root-level directories by default
            for i in 0..tree.nodes.len() {
                if tree.nodes[i].expandable {
                    tree.state.expand(vec![i]);
                }
            }
        }

        tree
    }
}

/// Builds a directory node with all its children from a flat list of paths.
fn build_directory_node(dir_name: &str, files: &[(String, FileStatus)]) -> TreeNode<DiffFileEntry> {
    let entry = DiffFileEntry::directory(dir_name, dir_name);

    // Group files by their next path component
    let mut subdirs: HashMap<String, Vec<(String, FileStatus)>> = HashMap::new();
    let mut direct_files: Vec<(String, FileStatus)> = Vec::new();

    for (path, status) in files {
        let relative = path.strip_prefix(dir_name).unwrap_or(path);
        let relative = relative.strip_prefix('/').unwrap_or(relative);
        let parts: Vec<&str> = relative.split('/').collect();

        if parts.len() == 1 {
            // Direct child file
            direct_files.push((path.clone(), *status));
        } else {
            // Nested in subdirectory
            let subdir = parts[0].to_string();
            subdirs
                .entry(subdir)
                .or_default()
                .push((path.clone(), *status));
        }
    }

    // Build children
    let mut children = Vec::new();

    // Add subdirectories
    let mut subdir_names: Vec<_> = subdirs.keys().collect();
    subdir_names.sort();

    for subdir_name in subdir_names {
        if let Some(subdir_files) = subdirs.get(subdir_name) {
            let subdir_path = format!("{}/{}", dir_name, subdir_name);
            let subdir_node = build_subdirectory_node(&subdir_path, subdir_name, subdir_files);
            children.push(subdir_node);
        }
    }

    // Add direct files
    direct_files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    for (path, status) in direct_files {
        let name = path.split('/').last().unwrap_or(&path);
        let entry = DiffFileEntry::file(name, &path, status);
        children.push(TreeNode::new(entry));
    }

    // Sort: directories first, then alphabetically
    children.sort_by(|a, b| {
        let a_is_dir = a.data.is_dir;
        let b_is_dir = b.data.is_dir;
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        }
    });

    TreeNode::with_children(entry, children)
}

/// Builds a subdirectory node recursively.
fn build_subdirectory_node(
    full_path: &str,
    name: &str,
    files: &[(String, FileStatus)],
) -> TreeNode<DiffFileEntry> {
    let entry = DiffFileEntry::directory(name, full_path);

    // Group files by their next path component relative to this directory
    let mut subdirs: HashMap<String, Vec<(String, FileStatus)>> = HashMap::new();
    let mut direct_files: Vec<(String, FileStatus)> = Vec::new();

    for (path, status) in files {
        let relative = path.strip_prefix(full_path).unwrap_or(path);
        let relative = relative.strip_prefix('/').unwrap_or(relative);
        let parts: Vec<&str> = relative.split('/').collect();

        if parts.len() == 1 {
            // Direct child file
            direct_files.push((path.clone(), *status));
        } else {
            // Nested in subdirectory
            let subdir = parts[0].to_string();
            subdirs
                .entry(subdir)
                .or_default()
                .push((path.clone(), *status));
        }
    }

    // Build children
    let mut children = Vec::new();

    // Add subdirectories
    let mut subdir_names: Vec<_> = subdirs.keys().collect();
    subdir_names.sort();

    for subdir_name in subdir_names {
        if let Some(subdir_files) = subdirs.get(subdir_name) {
            let subdir_full_path = format!("{}/{}", full_path, subdir_name);
            let subdir_node = build_subdirectory_node(&subdir_full_path, subdir_name, subdir_files);
            children.push(subdir_node);
        }
    }

    // Add direct files
    direct_files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    for (path, status) in direct_files {
        let name = path.split('/').last().unwrap_or(&path);
        let entry = DiffFileEntry::file(name, &path, status);
        children.push(TreeNode::new(entry));
    }

    // Sort: directories first, then alphabetically
    children.sort_by(|a, b| {
        let a_is_dir = a.data.is_dir;
        let b_is_dir = b.data.is_dir;
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        }
    });

    TreeNode::with_children(entry, children)
}
