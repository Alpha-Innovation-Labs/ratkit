use crate::dialog::{Dialog, DialogType};

impl<'a> Dialog<'a> {
    pub fn confirm(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message)
            .dialog_type(DialogType::Confirm)
            .buttons(vec!["Yes", "No"])
    }
}
