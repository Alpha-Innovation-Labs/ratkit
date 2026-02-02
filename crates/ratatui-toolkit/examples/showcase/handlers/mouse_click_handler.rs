//! Mouse click handler for menu bar interactions.

use crate::app::App;
use crate::demo_tab::DemoTab;

pub fn handle_mouse_click(app: &mut App, column: u16, row: u16) {
    if let Some(idx) = app.menu_bar.handle_click(column, row) {
        if idx == 6 {
            app.select_tab(DemoTab::AiChat);
        } else if idx == 7 {
            app.select_tab(DemoTab::Primitives);
        } else if idx == 8 {
            app.original_theme = Some(app.current_theme.clone());
            app.show_theme_picker = true;
            app.theme_filter.clear();
            app.theme_picker_index = 0;
        } else if idx < DemoTab::all().len() {
            app.select_tab(DemoTab::all()[idx]);
        }
    }
}
