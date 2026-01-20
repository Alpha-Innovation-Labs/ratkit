use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn title_inside(mut self, title_inside: bool) -> Self {
        self.title_inside = title_inside;
        self
    }
}
