use crate::primitives::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn dialog_type(mut self, dialog_type: crate::primitives::dialog::DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }
}
