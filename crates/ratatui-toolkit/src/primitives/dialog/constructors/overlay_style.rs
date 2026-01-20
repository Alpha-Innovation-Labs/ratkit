use ratatui::style::Style;

use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn overlay_style(mut self, style: Style) -> Self {
        self.overlay_style = style;
        self
    }
}
