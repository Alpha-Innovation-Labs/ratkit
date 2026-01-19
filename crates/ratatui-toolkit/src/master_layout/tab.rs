//! Tab component containing panes and footer

use super::{Footer, InteractionMode, Pane, PaneContainer, PaneLayout};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// A tab containing panes and a footer
pub struct Tab {
    name: String,
    pane_container: PaneContainer,
    footer: Footer,
}

impl Tab {
    /// Create a new tab with given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pane_container: PaneContainer::default(),
            footer: Footer::with_mode(),
        }
    }

    /// Create a tab with a specific layout
    pub fn with_layout(name: impl Into<String>, layout: PaneLayout) -> Self {
        Self {
            name: name.into(),
            pane_container: PaneContainer::new(layout),
            footer: Footer::with_mode(),
        }
    }

    /// Get the tab name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the tab name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Get reference to pane container
    pub fn pane_container(&self) -> &PaneContainer {
        &self.pane_container
    }

    /// Get mutable reference to pane container
    pub fn pane_container_mut(&mut self) -> &mut PaneContainer {
        &mut self.pane_container
    }

    /// Get reference to footer
    pub fn footer(&self) -> &Footer {
        &self.footer
    }

    /// Get mutable reference to footer
    pub fn footer_mut(&mut self) -> &mut Footer {
        &mut self.footer
    }

    /// Add a pane to this tab
    pub fn add_pane(&mut self, pane: Pane) {
        self.pane_container.add_pane(pane);
    }

    /// Set the layout for panes in this tab
    pub fn set_layout(&mut self, layout: PaneLayout) {
        // Create new container with new layout, transfer panes
        let new_container = PaneContainer::new(layout);

        // We can't easily move panes out, so this is a limitation
        // In real usage, layout should be set before adding panes
        // For now, just create new container (existing panes will be lost)
        self.pane_container = new_container;
    }

    /// Get number of panes in this tab
    pub fn pane_count(&self) -> usize {
        self.pane_container.pane_count()
    }

    /// Render the tab (panes + footer)
    pub fn render(&mut self, frame: &mut ratatui::Frame, area: Rect, mode: &InteractionMode) {
        // Calculate layout: panes take most space, footer takes 1 row
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    // Panes area (minimum 3 rows)
                Constraint::Length(3), // Footer (3 rows with border)
            ])
            .split(area);

        let pane_area = chunks[0];
        let footer_area = chunks[1];

        // Update pane layout
        self.pane_container.update_layout(pane_area);

        // Render panes
        self.pane_container.render(frame, mode);

        // Render footer
        self.footer
            .render_with_mode(footer_area, frame.buffer_mut(), mode);
    }
}

#[cfg(test)]
mod tests {
    use super::super::PaneId;
    use super::*;
    use crossterm::event::{KeyEvent, MouseEvent};
    use ratatui::{buffer::Buffer, widgets::Widget};

    // Mock PaneContent for testing
    struct MockContent {
        title: String,
    }

    impl MockContent {
        fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
            }
        }
    }

    impl Widget for MockContent {
        fn render(self, _area: Rect, _buf: &mut Buffer) {}
    }

    impl super::super::pane::PaneContent for MockContent {
        fn handle_key(&mut self, _key: KeyEvent) -> bool {
            true
        }
        fn handle_mouse(&mut self, _mouse: MouseEvent) -> bool {
            true
        }
        fn title(&self) -> String {
            self.title.clone()
        }
        fn render_content(&mut self, _area: Rect, _frame: &mut ratatui::Frame) {
            // Mock implementation - do nothing
        }
    }

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("Test Tab");
        assert_eq!(tab.name(), "Test Tab");
        assert_eq!(tab.pane_count(), 0);
    }

    #[test]
    fn test_tab_with_layout() {
        let layout = PaneLayout::Horizontal(vec![50, 50]);
        let tab = Tab::with_layout("Test Tab", layout);
        assert_eq!(tab.name(), "Test Tab");
        assert_eq!(tab.pane_count(), 0);
    }

    #[test]
    fn test_set_name() {
        let mut tab = Tab::new("Original");
        assert_eq!(tab.name(), "Original");

        tab.set_name("Updated");
        assert_eq!(tab.name(), "Updated");
    }

    #[test]
    fn test_add_pane() {
        let mut tab = Tab::new("Test Tab");
        let pane_id = PaneId::new("pane1");
        let pane = Pane::new(pane_id, Box::new(MockContent::new("Pane 1")));

        tab.add_pane(pane);
        assert_eq!(tab.pane_count(), 1);
    }

    #[test]
    fn test_add_multiple_panes() {
        let mut tab = Tab::new("Test Tab");

        tab.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("Pane 1")),
        ));
        tab.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("Pane 2")),
        ));
        tab.add_pane(Pane::new(
            PaneId::new("p3"),
            Box::new(MockContent::new("Pane 3")),
        ));

        assert_eq!(tab.pane_count(), 3);
    }

    #[test]
    fn test_pane_container_access() {
        let mut tab = Tab::new("Test Tab");
        let pane_id = PaneId::new("pane1");

        tab.add_pane(Pane::new(pane_id, Box::new(MockContent::new("Pane 1"))));

        // Immutable access
        assert_eq!(tab.pane_container().pane_count(), 1);

        // Mutable access
        let pane_id2 = PaneId::new("pane2");
        tab.pane_container_mut()
            .add_pane(Pane::new(pane_id2, Box::new(MockContent::new("Pane 2"))));
        assert_eq!(tab.pane_count(), 2);
    }

    #[test]
    fn test_footer_access() {
        let mut tab = Tab::new("Test Tab");

        // Footer should exist
        assert!(!tab.footer().items.is_empty()); // Has mode indicator

        // Can add items
        tab.footer_mut().add_static("Test item");
        assert!(tab.footer().items.len() > 1);
    }

    #[test]
    fn test_set_layout() {
        let mut tab = Tab::new("Test Tab");

        // Add a pane with default layout
        tab.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("Pane 1")),
        ));
        assert_eq!(tab.pane_count(), 1);

        // Change layout (note: this creates new container, panes are lost)
        tab.set_layout(PaneLayout::Vertical(vec![30, 70]));

        // Panes are lost due to current implementation limitation
        assert_eq!(tab.pane_count(), 0);
    }

    #[test]
    fn test_render_does_not_panic() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;

        let mut tab = Tab::new("Test Tab");
        tab.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("Pane 1")),
        ));

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                tab.render(frame, area, &InteractionMode::default());
            })
            .unwrap();
    }

    #[test]
    fn test_empty_tab_renders() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;

        let mut tab = Tab::new("Empty Tab");

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                tab.render(frame, area, &InteractionMode::default());
            })
            .unwrap();
    }
}
