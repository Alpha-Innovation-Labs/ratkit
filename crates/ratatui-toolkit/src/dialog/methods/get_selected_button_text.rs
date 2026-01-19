use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn get_selected_button_text(&self) -> Option<&str> {
        self.buttons.get(self.selected_button).copied()
    }
}
