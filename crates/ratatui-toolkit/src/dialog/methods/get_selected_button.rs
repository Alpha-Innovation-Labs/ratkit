use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn get_selected_button(&self) -> usize {
        self.selected_button
    }
}
