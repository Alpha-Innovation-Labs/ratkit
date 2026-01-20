use ratatui::style::Style;

use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn footer_style(mut self, style: Style) -> Self {
        self.footer_style = style;
        self
    }
}
