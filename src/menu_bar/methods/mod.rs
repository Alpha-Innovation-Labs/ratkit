use crate::menu_bar::functions::display_width;
use crate::menu_bar::MenuBar;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

impl MenuBar {
    pub fn update_hover(&mut self, column: u16, row: u16) {
        for item in &mut self.items {
            item.hovered = if let Some(area) = item.area {
                column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
            } else {
                false
            };
        }
    }

    pub fn handle_click(&mut self, column: u16, row: u16) -> Option<usize> {
        let clicked_index = self.items.iter().enumerate().find_map(|(i, item)| {
            if let Some(area) = item.area {
                if column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    return Some(i);
                }
            }
            None
        });

        if let Some(clicked) = clicked_index {
            for (i, item) in self.items.iter_mut().enumerate() {
                item.selected = i == clicked;
            }
        }

        clicked_index
    }

    pub fn selected(&self) -> Option<usize> {
        self.items.iter().position(|item| item.selected)
    }

    /// Render the menu bar as a connected series of items with rounded border
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.render_with_offset(frame, area, 0);
    }

    /// Render the menu bar with a left offset to make room for other components
    pub fn render_with_offset(&mut self, frame: &mut Frame, area: Rect, left_offset: u16) {
        if self.items.is_empty() {
            return;
        }

        // Calculate required width based on menu items using proper display width
        // Items have no padding, separators have spacing, 1 space at start and end
        let total_label_width: usize = self
            .items
            .iter()
            .map(|item| display_width(&item.display_label()))
            .sum();
        let separators = (self.items.len() - 1) * 3; // " │ " between items (1 space + separator + 1 space)
        let needed_width = (total_label_width + separators + 4) as u16; // +2 for borders + 2 for start/end spaces

        // Adjust area to account for left offset
        let available_width = area.width.saturating_sub(left_offset);

        // Create a fixed-width area for the button group, shifted by the offset
        let button_group_area = Rect {
            x: area.x + left_offset,
            y: area.y,
            width: needed_width.min(available_width),
            height: area.height,
        };

        self.area = Some(button_group_area);

        // Create a block with rounded border around the entire group
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(button_group_area);
        frame.render_widget(block, button_group_area);

        // Start with 1 space padding from the left border
        let mut x_offset = inner_area.x + 1;
        let button_count = self.items.len();

        for (i, item) in self.items.iter_mut().enumerate() {
            // No padding on menu item - use custom display width for Nerd Font icons
            let label = item.display_label();
            let item_width = display_width(&label) as u16;

            // Check if we have space left to render this item
            let available_width = (inner_area.x + inner_area.width).saturating_sub(x_offset);
            if available_width == 0 {
                break; // No space left
            }

            // Limit item width to available space
            let actual_item_width = item_width.min(available_width);

            let item_area = Rect {
                x: x_offset,
                y: inner_area.y,
                width: actual_item_width,
                height: inner_area.height,
            };

            item.area = Some(item_area);

            // Determine style based on state
            let style = match (item.selected, item.hovered) {
                (true, true) => self.selected_hover_style,
                (true, false) => self.selected_style,
                (false, true) => self.hover_style,
                (false, false) => self.normal_style,
            };

            // Create menu item text with no padding
            // Truncate label if needed to fit available space
            let display_label = if actual_item_width < item_width {
                // Truncate the label to fit
                label
                    .chars()
                    .take(actual_item_width as usize)
                    .collect::<String>()
            } else {
                label
            };
            let paragraph = Paragraph::new(display_label).style(style);
            frame.render_widget(paragraph, item_area);

            x_offset += actual_item_width;

            // Render separator after item (except for last item)
            // Check if there's enough space left before rendering
            if i < button_count - 1 && x_offset + 3 <= inner_area.x + inner_area.width {
                let separator_area = Rect {
                    x: x_offset,
                    y: inner_area.y,
                    width: 3, // " │ " (1 space + separator + 1 space)
                    height: inner_area.height,
                };
                let separator = Paragraph::new(" │ ");
                frame.render_widget(separator, separator_area);
                x_offset += 3;
            }
        }
    }

    /// Render with a centered layout (useful for menu bars)
    pub fn render_centered(&mut self, frame: &mut Frame, area: Rect) {
        // Calculate total width needed for all menu items
        let total_chars: usize = self
            .items
            .iter()
            .map(|item| display_width(&item.display_label()) + 4)
            .sum(); // +4 for borders/padding
        let needed_width = total_chars as u16;

        // Create a centered layout
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((area.width.saturating_sub(needed_width)) / 2),
                Constraint::Length(needed_width.min(area.width)),
                Constraint::Min(0),
            ])
            .split(area);

        self.render(frame, chunks[1]);
    }
}
