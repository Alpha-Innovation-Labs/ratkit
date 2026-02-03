use crate::widgets::file_system_tree::{FileSystemTree, FileSystemTreeConfig};

impl<'a> FileSystemTree<'a> {
    pub fn with_dir_style(mut self, style: ratatui::style::Style) -> Self {
        self.config.dir_style = style;
        self
    }
}

#[allow(dead_code)]
pub fn with_dir_style(tree: FileSystemTree, style: ratatui::style::Style) -> FileSystemTree {
    FileSystemTree {
        config: FileSystemTreeConfig {
            dir_style: style,
            ..tree.config
        },
        ..tree
    }
}
