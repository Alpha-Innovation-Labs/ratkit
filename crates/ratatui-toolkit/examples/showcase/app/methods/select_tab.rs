//! Select tab method.

use super::super::App;
use crate::demo_tab::DemoTab;

impl App {
    /// Select a demo tab and update menu bar state.
    pub fn select_tab(&mut self, tab: DemoTab) {
        self.current_tab = tab;
        let idx = DemoTab::all().iter().position(|t| *t == tab).unwrap_or(0);
        for (i, item) in self.menu_bar.items.iter_mut().enumerate() {
            item.selected = i == idx;
        }
    }
}
