use crate::dialog::{Dialog, DialogType};

impl<'a> Dialog<'a> {
    pub fn info(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Info)
    }
}
