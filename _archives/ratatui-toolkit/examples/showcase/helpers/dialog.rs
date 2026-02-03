//! Helper functions for dialog rendering.

use crate::app::App;
use ratatui::Frame;

pub fn render_dialog(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if !app.show_dialog {
        return;
    }
    let (title, message) = match app.dialog_type {
        ratatui_toolkit::DialogType::Info => (
            "Information",
            "Button pressed.\n\nPress Enter or Esc to close.",
        ),
        ratatui_toolkit::DialogType::Success => ("Success!", "Operation completed successfully!"),
        ratatui_toolkit::DialogType::Warning => ("Warning", "This action may have consequences."),
        ratatui_toolkit::DialogType::Error => ("Error", "Something went wrong!"),
        ratatui_toolkit::DialogType::Confirm => ("Confirm", "Do you want to proceed?"),
    };
    let mut dialog = ratatui_toolkit::Dialog::new(title, message)
        .dialog_type(app.dialog_type)
        .width_percent(0.5)
        .height_percent(0.35)
        .with_theme(&app.current_theme);
    let dialog_widget = ratatui_toolkit::DialogWidget::new(&mut dialog);
    frame.render_widget(dialog_widget, area);
}
