use std::collections::HashSet;

use crate::tree_node::FileSystemTreeNode;

#[derive(Debug, Clone, Default)]
pub struct FileSystemTreeState {
    pub selected_path: Option<Vec<usize>>,
    pub expanded: HashSet<Vec<usize>>,
    pub offset: usize,
    pub filter: Option<String>,
    pub filter_mode: bool,
}

impl FileSystemTreeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, path: Vec<usize>) {
        self.selected_path = Some(path);
    }

    pub fn clear_selection(&mut self) {
        self.selected_path = None;
    }

    pub fn is_expanded(&self, path: &[usize]) -> bool {
        self.expanded.contains(path)
    }

    pub fn expand(&mut self, path: Vec<usize>) {
        self.expanded.insert(path);
    }

    pub fn collapse(&mut self, path: Vec<usize>) {
        self.expanded.remove(&path);
    }

    pub fn toggle_expansion(&mut self, path: Vec<usize>) {
        if self.expanded.contains(&path) {
            self.expanded.remove(&path);
        } else {
            self.expanded.insert(path);
        }
    }

    pub fn expand_all(&mut self, nodes: &[FileSystemTreeNode], current_path: &mut Vec<usize>) {
        for (idx, node) in nodes.iter().enumerate() {
            current_path.push(idx);
            if node.is_dir() {
                self.expanded.insert(current_path.clone());
                if !node.children.is_empty() {
                    self.expand_all(&node.children, current_path);
                }
            }
            current_path.pop();
        }
    }

    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }

    pub fn enter_filter_mode(&mut self) {
        self.filter_mode = true;
        self.filter = Some(String::new());
    }

    pub fn exit_filter_mode(&mut self) {
        self.filter_mode = false;
        self.filter = None;
    }

    pub fn is_filter_mode(&self) -> bool {
        self.filter_mode
    }

    pub fn filter_text(&self) -> Option<&str> {
        self.filter.as_deref()
    }

    pub fn set_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }

    pub fn push_filter(&mut self, c: char) {
        if let Some(filter) = &mut self.filter {
            filter.push(c);
        }
    }

    pub fn pop_filter(&mut self) {
        if let Some(filter) = &mut self.filter {
            filter.pop();
        }
    }

    pub fn clear_filter(&mut self) {
        self.filter = None;
        self.filter_mode = false;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
