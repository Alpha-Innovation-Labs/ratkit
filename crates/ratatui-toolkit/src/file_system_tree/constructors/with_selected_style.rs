use crate::file_system_tree::FileSystemTree;

impl<'a> FileSystemTree<'a> {
    pub fn with_selected_style(mut self, style: ratatui::style::Style) -> Self {
        self.config.selected_style = style;
        self
    }
}
