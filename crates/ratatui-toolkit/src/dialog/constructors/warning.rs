use crate::dialog::{Dialog, DialogType};

impl<'a> Dialog<'a> {
    pub fn warning(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Warning)
    }
}
