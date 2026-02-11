use crate::widgets::file_system_tree::entry::FileSystemEntry;

#[derive(Debug, Clone)]
pub struct FileSystemTreeNode {
    pub data: FileSystemEntry,
    pub children: Vec<FileSystemTreeNode>,
    pub expandable: bool,
}

impl FileSystemTreeNode {
    pub fn new(data: FileSystemEntry) -> Self {
        Self {
            data,
            children: Vec::new(),
            expandable: false,
        }
    }

    pub fn with_children(data: FileSystemEntry, children: Vec<FileSystemTreeNode>) -> Self {
        let expandable = !children.is_empty();
        Self {
            data,
            children,
            expandable,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.data.is_dir
    }
}
