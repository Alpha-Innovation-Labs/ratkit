use ratatui::style::Style;

use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn button_selected_style(mut self, style: Style) -> Self {
        self.button_selected_style = style;
        self
    }
}
