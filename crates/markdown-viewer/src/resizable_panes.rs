//! Resizable split panes component

use ratatui::layout::Rect;
use ratatui_toolkit::{ResizableSplit, SplitDirection};

/// State wrapper for ResizableSplit to maintain ViewerState API compatibility
#[derive(Debug, Clone)]
pub struct ResizablePanesState {
    /// Current split ratio (0.0-1.0)
    pub split_ratio: f32,
}

impl Default for ResizablePanesState {
    fn default() -> Self {
        Self::new()
    }
}

impl ResizablePanesState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            split_ratio: 0.25,
        }
    }

    /// Create with custom split ratio
    pub fn with_split_ratio(ratio: f32) -> Self {
        let mut state = Self::new();
        state.split_ratio = ratio.clamp(0.1, 0.9);
        state
    }

    /// Start dragging
    pub fn start_drag(&mut self) {
        // Handled by ResizableSplit internally
    }

    /// Stop dragging
    pub fn stop_drag(&mut self) {
        // Handled by ResizableSplit internally
    }

    /// Update drag position (handled by ResizableSplit internally)
    pub fn update_drag(&mut self, _current_x: u16, _area_width: u16, _min_width: u16) {
        // Handled by ResizableSplit internally
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        // Handled by ResizableSplit internally
        false
    }

    /// Get current split ratio
    pub fn split_ratio(&self) -> f32 {
        self.split_ratio
    }

    /// Set split ratio directly
    pub fn set_split_ratio(&mut self, ratio: f32) {
        self.split_ratio = ratio.clamp(0.1, 0.9);
    }
}

impl ResizablePanesState {
    /// Calculate areas for resizable split using ratatui_toolkit::ResizableSplit
    pub fn calculate_areas(&self, area: Rect, split: &mut ResizableSplit) -> (Rect, Rect, Rect) {
        // Update divider position based on current area
        split.update_divider_position(area);

        let (left_area, right_area) = if split.direction == SplitDirection::Vertical {
            let divider_x = split.divider_pos;
            let left_width = divider_x - area.x;
            let right_x = divider_x + 1;

            let left = Rect {
                x: area.x,
                y: area.y,
                width: left_width,
                height: area.height,
            };

            let right = Rect {
                x: right_x,
                y: area.y,
                width: area.width - left_width - 1,
                height: area.height,
            };

            // Divider area (1 pixel wide)
            let divider = Rect {
                x: divider_x,
                y: area.y,
                width: 1,
                height: area.height,
            };

            (left, right, divider)
        } else {
            // Horizontal split (not used in markdown viewer)
            let divider_y = split.divider_pos;
            let top_height = divider_y - area.y;
            let bottom_y = divider_y + 1;

            let top = Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: top_height,
            };

            let bottom = Rect {
                x: area.x,
                y: bottom_y,
                width: area.width,
                height: area.height - top_height - 1,
            };

            // Divider area (1 pixel tall)
            let divider = Rect {
                x: area.x,
                y: divider_y,
                width: area.width,
                height: 1,
            };

            (top, bottom, divider)
        }
    }
}

/// Widget for rendering the divider
pub struct DividerWidget {
    symbol: String,
}

impl Default for DividerWidget {
    fn default() -> Self {
        Self {
            symbol: "│".to_string(),
        }
    }
}

impl DividerWidget {
    /// Create with custom symbol
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl ratatui::widgets::Widget for DividerWidget {
    fn render(self, area: Rect, buf: &mut ratatui::backend::CrosstermBackend)
    where
        Self: Sized,
    {
        let span = ratatui::text::Span::styled(
            self.symbol.as_str(),
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
        );

        for y in area.top()..area.bottom() {
            let line = ratatui::text::Line::from(vec![span.clone()]);
            line.render(ratatui::layout::Rect::new(area.x, y, area.width, 1), buf);
        }
    }
}
