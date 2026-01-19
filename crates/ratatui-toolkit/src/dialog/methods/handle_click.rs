use crate::dialog::Dialog;

impl<'a> Dialog<'a> {
    pub fn handle_click(&self, column: u16, row: u16) -> Option<usize> {
        for (idx, area) in self.button_areas.iter().enumerate() {
            if column >= area.x
                && column < area.x + area.width
                && row >= area.y
                && row < area.y + area.height
            {
                return Some(idx);
            }
        }
        None
    }
}
