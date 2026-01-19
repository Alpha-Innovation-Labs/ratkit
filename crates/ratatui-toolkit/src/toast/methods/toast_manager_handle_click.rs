use crate::toast::ToastManager;
use ratatui::layout::Rect;

impl ToastManager {
    /// Handle a mouse click and dismiss any toast that was clicked.
    ///
    /// Returns `true` if a toast was dismissed, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the click
    /// * `y` - The y coordinate of the click
    /// * `frame_area` - The total frame area (used to calculate toast positions)
    pub fn handle_click(&mut self, x: u16, y: u16, frame_area: Rect) -> bool {
        // Use same constants as render_toasts
        const TOAST_WIDTH: u16 = 40;
        const TOAST_HEIGHT: u16 = 3;
        const TOAST_MARGIN: u16 = 2;
        const TOAST_SPACING: u16 = 1;

        let active_count = self.toasts.iter().filter(|t| !t.is_expired()).count();
        if active_count == 0 {
            return false;
        }

        let mut y_offset = frame_area.height.saturating_sub(TOAST_MARGIN);

        // Iterate through toasts in reverse order (same as render_toasts)
        // to find which one (if any) was clicked
        for i in (0..self.toasts.len()).rev() {
            if self.toasts[i].is_expired() {
                continue;
            }

            let toast_y = y_offset.saturating_sub(TOAST_HEIGHT);
            let toast_x = frame_area.width.saturating_sub(TOAST_WIDTH + TOAST_MARGIN);

            // Check if click is within this toast's bounds
            if x >= toast_x
                && x < toast_x + TOAST_WIDTH
                && y >= toast_y
                && y < toast_y + TOAST_HEIGHT
            {
                self.toasts.remove(i);
                return true;
            }

            y_offset = toast_y.saturating_sub(TOAST_SPACING);

            if toast_y == 0 || toast_x == 0 {
                break;
            }
        }

        false
    }
}
