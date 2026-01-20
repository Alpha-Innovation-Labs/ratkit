use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn overlay(mut self, overlay: bool) -> Self {
        self.overlay = overlay;
        self
    }
}
