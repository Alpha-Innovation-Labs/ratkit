use crate::menu_bar::{MenuBar, MenuItem};

impl Default for MenuBar {
    fn default() -> Self {
        Self::new(vec![MenuItem::new("Menu Item", 0)])
    }
}
