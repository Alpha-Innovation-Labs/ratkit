use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn select_next_button(&mut self) {
        if !self.buttons.is_empty() && self.selected_button < self.buttons.len() - 1 {
            self.selected_button += 1;
        }
    }
}
