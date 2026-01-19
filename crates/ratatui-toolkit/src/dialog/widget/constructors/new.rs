use crate::dialog::{Dialog, DialogWidget};

impl<'a> DialogWidget<'a> {
    pub fn new(dialog: &'a mut Dialog<'a>) -> Self {
        Self { dialog }
    }
}
