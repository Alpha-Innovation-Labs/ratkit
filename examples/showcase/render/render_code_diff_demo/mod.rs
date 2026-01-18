//! Render the code diff demo tab.

use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::app::App;

/// Render the code diff demo.
///
/// The CodeDiff widget handles everything internally:
/// - File tree sidebar (toggle with `[`)
/// - Diff view
/// - Focus switching (h/l or Tab)
/// - Navigation (j/k)
/// - Resize (H/L or </>)
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
pub fn render_code_diff_demo(frame: &mut ratatui::Frame, area: Rect, app: &App) {
    (&app.code_diff).render(area, frame.buffer_mut());
}
