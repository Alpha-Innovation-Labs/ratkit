//! Render the AI chat demo tab.

use ratatui::layout::Rect;
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Render the AI chat demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
/// * `theme` - The application theme.
pub fn render_ai_chat_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
    let widget =
        ratatui_toolkit::AIChat::new_ai_chat(&mut app.ai_chat_messages, &mut app.ai_chat_input);

    widget.render(frame, area);
}
