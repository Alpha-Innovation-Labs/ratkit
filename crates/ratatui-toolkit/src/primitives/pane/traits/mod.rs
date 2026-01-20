use crate::primitives::pane::Pane;

impl<'a> Default for Pane<'a> {
    fn default() -> Self {
        Self::new("Pane")
    }
}
