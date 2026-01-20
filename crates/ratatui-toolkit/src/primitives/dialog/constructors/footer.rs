use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn footer(mut self, footer: &'a str) -> Self {
        self.footer = Some(footer);
        self
    }
}
