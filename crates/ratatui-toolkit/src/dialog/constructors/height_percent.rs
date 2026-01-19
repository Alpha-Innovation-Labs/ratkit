use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn height_percent(mut self, height_percent: f32) -> Self {
        self.height_percent = height_percent;
        self
    }
}
