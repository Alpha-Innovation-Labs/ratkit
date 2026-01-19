use ratatui::layout::Rect;

use crate::resizable_split::{ResizableSplit, SplitDirection};

impl ResizableSplit {
    pub fn update_divider_position(&mut self, area: Rect) {
        match self.direction {
            SplitDirection::Vertical => {
                let left_width = (area.width as u32 * self.split_percent as u32 / 100) as u16;
                self.divider_pos = area.x + left_width;
            }
            SplitDirection::Horizontal => {
                let top_height = (area.height as u32 * self.split_percent as u32 / 100) as u16;
                self.divider_pos = area.y + top_height;
            }
        }
    }

    pub fn is_on_divider(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> bool {
        match self.direction {
            SplitDirection::Vertical => {
                let divider_start = self.divider_pos.saturating_sub(1);
                let divider_end = (self.divider_pos + 1).min(area.x + area.width.saturating_sub(1));
                mouse_column >= divider_start && mouse_column <= divider_end
            }
            SplitDirection::Horizontal => {
                let divider_start = self.divider_pos.saturating_sub(1);
                let divider_end =
                    (self.divider_pos + 1).min(area.y + area.height.saturating_sub(1));
                mouse_row >= divider_start && mouse_row <= divider_end
            }
        }
    }

    pub fn update_from_mouse(&mut self, mouse_column: u16, mouse_row: u16, area: Rect) {
        if !self.is_dragging {
            return;
        }

        let new_percent = match self.direction {
            SplitDirection::Vertical => {
                let relative_column = mouse_column.saturating_sub(area.x);

                if area.width > 0 {
                    ((relative_column as u32 * 100) / area.width as u32) as u16
                } else {
                    self.split_percent
                }
            }
            SplitDirection::Horizontal => {
                let relative_row = mouse_row.saturating_sub(area.y);

                if area.height > 0 {
                    ((relative_row as u32 * 100) / area.height as u32) as u16
                } else {
                    self.split_percent
                }
            }
        };

        self.split_percent = new_percent.clamp(self.min_percent, self.max_percent);
    }

    pub fn render_divider_indicator(&self, _frame: &mut ratatui::Frame, _area: Rect) {
        // No separator rendering - border colors are changed on the panes themselves
    }
}
