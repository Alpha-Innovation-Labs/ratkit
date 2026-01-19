use ratatui::style::Style;

use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}
