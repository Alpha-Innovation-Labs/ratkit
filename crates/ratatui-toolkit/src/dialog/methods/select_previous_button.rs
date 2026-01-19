use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn select_previous_button(&mut self) {
        if self.selected_button > 0 {
            self.selected_button -= 1;
        }
    }
}
