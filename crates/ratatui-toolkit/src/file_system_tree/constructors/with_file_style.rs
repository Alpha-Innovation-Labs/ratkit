use crate::file_system_tree::FileSystemTree;

impl<'a> FileSystemTree<'a> {
    pub fn with_file_style(mut self, style: ratatui::style::Style) -> Self {
        self.config.file_style = style;
        self
    }
}
