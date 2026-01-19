use crate::pane::Pane;

impl<'a> Default for Pane<'a> {
    fn default() -> Self {
        Self::new("Pane")
    }
}
