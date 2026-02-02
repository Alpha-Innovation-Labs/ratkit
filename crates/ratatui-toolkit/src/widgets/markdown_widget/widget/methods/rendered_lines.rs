use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    pub fn rendered_lines(&self) -> &Vec<ratatui::text::Line<'static>> {
        &self.rendered_lines
    }
}
