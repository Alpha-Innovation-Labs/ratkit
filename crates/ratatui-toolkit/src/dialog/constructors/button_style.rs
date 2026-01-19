use ratatui::style::Style;

use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn button_style(mut self, style: Style) -> Self {
        self.button_style = style;
        self
    }
}
