use crate::primitives::statusline::{StatusLineStacked, StyledStatusLine};

impl<'a> Default for StatusLineStacked<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for StyledStatusLine<'a> {
    fn default() -> Self {
        Self::new()
    }
}
