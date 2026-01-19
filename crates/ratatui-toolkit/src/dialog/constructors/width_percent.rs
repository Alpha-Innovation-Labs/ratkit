use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn width_percent(mut self, width_percent: f32) -> Self {
        self.width_percent = width_percent;
        self
    }
}
