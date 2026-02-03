use ratatui::style::Color;

use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}
