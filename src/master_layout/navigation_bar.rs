//! Navigation bar for tab management

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Widget},
};

/// A button in the navigation bar representing a tab
#[derive(Debug, Clone)]
pub struct TabButton {
    pub label: String,
    pub area: Rect,
}

impl TabButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            area: Rect::default(),
        }
    }

    /// Check if coordinates are within this button
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
    }
}

/// Navigation bar component for displaying tabs
pub struct NavigationBar {
    buttons: Vec<TabButton>,
    active_index: usize,
}

impl NavigationBar {
    /// Create a new navigation bar with tab labels
    pub fn new(labels: Vec<String>) -> Self {
        let buttons = labels.into_iter().map(TabButton::new).collect();
        Self {
            buttons,
            active_index: 0,
        }
    }

    /// Set the active tab index
    pub fn set_active(&mut self, index: usize) {
        if index < self.buttons.len() {
            self.active_index = index;
        }
    }

    /// Get the active tab index
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Get number of tabs
    pub fn tab_count(&self) -> usize {
        self.buttons.len()
    }

    /// Handle a mouse click, returns the clicked tab index if any
    pub fn handle_click(&self, x: u16, y: u16) -> Option<usize> {
        self.buttons.iter().position(|button| button.contains(x, y))
    }

    /// Render the navigation bar with explicit active index
    pub fn render_with_active(&mut self, area: Rect, buf: &mut Buffer, active_index: usize) {
        self.render_with_active_and_offset(area, buf, active_index, 0);
    }

    /// Render the navigation bar with explicit active index and left offset
    pub fn render_with_active_and_offset(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        active_index: usize,
        left_offset: u16,
    ) {
        // Update active index
        self.active_index = active_index;

        // Calculate button areas
        let button_count = self.buttons.len();
        let available_width = area.width.saturating_sub(2).saturating_sub(left_offset);
        let button_width = if button_count > 0 {
            available_width / button_count as u16
        } else {
            0
        };

        let mut current_x = area.x + 1 + left_offset;

        for (i, button) in self.buttons.iter_mut().enumerate() {
            // Calculate button area
            let width = if i == button_count - 1 {
                // Last button takes remaining space
                area.width
                    .saturating_sub(current_x - area.x)
                    .saturating_sub(1)
            } else {
                button_width
            };

            button.area = Rect::new(current_x, area.y, width, 1);

            // Render button
            let is_active = i == self.active_index;
            let style = if is_active {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let label = format!(" {} ", button.label);
            let span = Span::styled(label, style);

            // Render directly to buffer
            let mut x = button.area.x;
            for grapheme in span.content.chars() {
                if x < button.area.x + button.area.width {
                    buf[(x, button.area.y)]
                        .set_char(grapheme)
                        .set_style(span.style);
                    x += 1;
                }
            }

            current_x += width;
        }

        // Draw border around the nav bar
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        block.render(area, buf);
    }
}

impl Widget for NavigationBar {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let active_index = self.active_index;
        self.render_with_active(area, buf, active_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_button_creation() {
        let button = TabButton::new("Test");
        assert_eq!(button.label, "Test");
        assert_eq!(button.area, Rect::default());
    }

    #[test]
    fn test_tab_button_contains() {
        let mut button = TabButton::new("Test");
        button.area = Rect::new(10, 0, 20, 1);

        assert!(button.contains(15, 0));
        assert!(button.contains(10, 0)); // Left edge
        assert!(button.contains(29, 0)); // Right edge - 1

        assert!(!button.contains(5, 0)); // Before
        assert!(!button.contains(30, 0)); // After
        assert!(!button.contains(15, 1)); // Different row
    }

    #[test]
    fn test_navigation_bar_creation() {
        let labels = vec!["Tab 1".to_string(), "Tab 2".to_string()];
        let nav_bar = NavigationBar::new(labels);

        assert_eq!(nav_bar.tab_count(), 2);
        assert_eq!(nav_bar.active_index(), 0);
    }

    #[test]
    fn test_set_active() {
        let labels = vec![
            "Tab 1".to_string(),
            "Tab 2".to_string(),
            "Tab 3".to_string(),
        ];
        let mut nav_bar = NavigationBar::new(labels);

        nav_bar.set_active(1);
        assert_eq!(nav_bar.active_index(), 1);

        nav_bar.set_active(2);
        assert_eq!(nav_bar.active_index(), 2);

        // Invalid index should be ignored
        nav_bar.set_active(10);
        assert_eq!(nav_bar.active_index(), 2);
    }

    #[test]
    fn test_handle_click() {
        let labels = vec!["Tab 1".to_string(), "Tab 2".to_string()];
        let mut nav_bar = NavigationBar::new(labels);

        // Setup button areas
        nav_bar.buttons[0].area = Rect::new(1, 0, 10, 1);
        nav_bar.buttons[1].area = Rect::new(11, 0, 10, 1);

        assert_eq!(nav_bar.handle_click(5, 0), Some(0));
        assert_eq!(nav_bar.handle_click(15, 0), Some(1));
        assert_eq!(nav_bar.handle_click(25, 0), None);
    }

    #[test]
    fn test_render_calculates_button_areas() {
        let labels = vec!["Tab 1".to_string(), "Tab 2".to_string()];
        let mut nav_bar = NavigationBar::new(labels);

        let area = Rect::new(0, 0, 80, 1);
        let mut buffer = Buffer::empty(area);

        nav_bar.render_with_active(area, &mut buffer, 0);

        // Check that button areas were calculated
        assert_ne!(nav_bar.buttons[0].area, Rect::default());
        assert_ne!(nav_bar.buttons[1].area, Rect::default());

        // Buttons should span the width
        let total_width: u16 = nav_bar.buttons.iter().map(|b| b.area.width).sum();
        assert!(total_width <= area.width);
    }

    #[test]
    fn test_empty_navigation_bar() {
        let nav_bar = NavigationBar::new(vec![]);
        assert_eq!(nav_bar.tab_count(), 0);
        assert_eq!(nav_bar.handle_click(10, 0), None);
    }

    #[test]
    fn test_single_tab() {
        let labels = vec!["Only Tab".to_string()];
        let nav_bar = NavigationBar::new(labels);
        assert_eq!(nav_bar.tab_count(), 1);
        assert_eq!(nav_bar.active_index(), 0);
    }

    #[test]
    fn test_many_tabs() {
        let labels = (1..=10).map(|i| format!("Tab {}", i)).collect();
        let nav_bar = NavigationBar::new(labels);
        assert_eq!(nav_bar.tab_count(), 10);
    }
}
