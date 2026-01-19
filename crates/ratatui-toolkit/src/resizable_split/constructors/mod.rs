use crate::resizable_split::{ResizableSplit, SplitDirection};

impl ResizableSplit {
    pub fn new(initial_percent: u16) -> Self {
        Self::new_with_direction(initial_percent, SplitDirection::Vertical)
    }

    pub fn new_with_direction(initial_percent: u16, direction: SplitDirection) -> Self {
        Self {
            split_percent: initial_percent.clamp(5, 95),
            min_percent: 10,
            max_percent: 90,
            is_dragging: false,
            is_hovering: false,
            direction,
            divider_pos: 0,
        }
    }

    pub fn start_drag(&mut self) {
        self.is_dragging = true;
    }

    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }

    pub fn right_percent(&self) -> u16 {
        100 - self.split_percent
    }

    pub fn bottom_percent(&self) -> u16 {
        self.right_percent()
    }
}
