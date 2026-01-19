use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn buttons(mut self, buttons: Vec<&'a str>) -> Self {
        self.buttons = buttons;
        self
    }
}
