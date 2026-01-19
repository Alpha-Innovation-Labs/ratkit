//! Pane component and PaneContent trait

use super::PaneId;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Widget},
};

/// Trait that pane content must implement
pub trait PaneContent {
    /// Handle keyboard input when pane is focused
    /// Returns true if the event was consumed, false otherwise
    fn handle_key(&mut self, key: KeyEvent) -> bool;

    /// Handle mouse input when pane is focused
    /// Returns true if the event was consumed, false otherwise
    fn handle_mouse(&mut self, mouse: MouseEvent) -> bool;

    /// Get the pane's title
    fn title(&self) -> String;

    /// Render the pane content
    /// This is called instead of Widget::render to allow mutable access
    /// Receives Frame so panes can use either Frame or Buffer rendering
    fn render_content(&mut self, area: Rect, frame: &mut ratatui::Frame);

    /// Check if this pane can receive focus
    /// Returns false for display-only panes (status displays, etc.)
    fn is_focusable(&self) -> bool {
        true
    }

    /// Start text selection at the given coordinates (relative to content area)
    fn start_selection(&mut self, _x: u16, _y: u16) {
        // Default: no-op (pane doesn't support selection)
    }

    /// Update text selection to the given coordinates (relative to content area)
    fn update_selection(&mut self, _x: u16, _y: u16) {
        // Default: no-op
    }

    /// End text selection
    fn end_selection(&mut self) {
        // Default: no-op
    }

    /// Get the currently selected text, if any
    fn get_selected_text(&self) -> Option<String> {
        None // Default: no selection
    }

    /// Clear the current selection
    fn clear_selection(&mut self) {
        // Default: no-op
    }

    /// Check if there is an active selection
    fn has_selection(&self) -> bool {
        false // Default: no selection
    }

    /// Notify the pane about focus change
    /// Called by the master layout when focus is gained or lost
    fn set_focused(&mut self, _focused: bool) {
        // Default: no-op
    }

    /// Whether this pane requires explicit focus mode (Enter to focus, Ctrl+A to exit).
    ///
    /// When `auto_focus` is enabled on `MasterLayout`:
    /// - Panes returning `false` (default): receive input immediately when selected
    /// - Panes returning `true`: require Enter to focus, Ctrl+A to exit (modal behavior)
    ///
    /// When `auto_focus` is disabled, this method has no effect (all panes are modal).
    ///
    /// Use cases for returning `true`:
    /// - Chat/message input panes that need Enter for sending messages
    /// - Terminal emulators that capture all keyboard input
    /// - Text editors that use navigation keys for cursor movement
    fn requires_focus_mode(&self) -> bool {
        false
    }

    /// Icon displayed when this pane is focused.
    ///
    /// Override to customize the focus indicator in the title bar.
    /// Default: "█"
    fn focus_icon(&self) -> &str {
        "█"
    }

    /// Icon displayed when this pane is selected (but not focused).
    ///
    /// Override to customize the selection indicator in the title bar.
    /// Default: "●"
    fn selected_icon(&self) -> &str {
        "●"
    }

    /// Get custom border style for this pane
    /// Default implementation uses standard focus/selection colors
    fn border_style(&self, is_selected: bool, is_focused: bool) -> Style {
        if is_focused {
            // Focused: Cyan bold
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if is_selected {
            // Selected: Yellow bold
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            // Inactive: Dark gray
            Style::default().fg(Color::DarkGray)
        }
    }

    /// Get title with focus/selection indicator
    ///
    /// Shows different indicators based on state:
    /// - Focused: `{focus_icon} Title (Focused)`
    /// - Selected + requires_focus_mode: `{selected_icon} Title (Press Enter)`
    /// - Selected (normal): `{selected_icon} Title (Selected)`
    /// - Inactive: `Title`
    ///
    /// Override `focus_icon()` and `selected_icon()` to customize the icons.
    fn title_with_indicator(&self, is_selected: bool, is_focused: bool) -> String {
        let title = self.title();
        if is_focused {
            format!("{} {} (Focused)", self.focus_icon(), title)
        } else if is_selected {
            if self.requires_focus_mode() {
                format!("{} {} (Press Enter)", self.selected_icon(), title)
            } else {
                format!("{} {} (Selected)", self.selected_icon(), title)
            }
        } else {
            title
        }
    }
}

/// A pane within a tab
pub struct Pane {
    id: PaneId,
    content: Box<dyn PaneContent>,
    area: Rect,

    // Visual enhancements
    /// Optional icon to display before the title
    icon: Option<String>,
    /// Padding around the content (top, right, bottom, left)
    padding: (u16, u16, u16, u16),
    /// Optional text footer displayed in the border
    text_footer: Option<String>,
    /// Border type (Rounded, Plain, Double, etc.)
    border_type: BorderType,
}

impl Pane {
    /// Create a new pane with given ID and content
    pub fn new(id: PaneId, content: Box<dyn PaneContent>) -> Self {
        Self {
            id,
            content,
            area: Rect::default(),
            icon: None,
            padding: (0, 0, 0, 0),
            text_footer: None,
            border_type: BorderType::Rounded,
        }
    }

    /// Set the icon for the title
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the padding (top, right, bottom, left)
    pub fn with_padding(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.padding = (top, right, bottom, left);
        self
    }

    /// Set uniform padding on all sides
    pub fn with_uniform_padding(mut self, padding: u16) -> Self {
        self.padding = (padding, padding, padding, padding);
        self
    }

    /// Set a text footer (displayed in the border)
    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.text_footer = Some(footer.into());
        self
    }

    /// Set the border type
    pub fn with_border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Get the pane's ID
    pub fn id(&self) -> PaneId {
        self.id
    }

    /// Get the pane's current area
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Set the pane's area (called during layout)
    pub fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    /// Get the pane's title
    pub fn title(&self) -> String {
        self.content.title()
    }

    /// Check if pane is focusable
    pub fn is_focusable(&self) -> bool {
        self.content.is_focusable()
    }

    /// Check if pane requires explicit focus mode when auto_focus is enabled
    ///
    /// When `auto_focus` is enabled on `MasterLayout`:
    /// - Returns `false`: pane receives input immediately when selected
    /// - Returns `true`: pane requires Enter to focus, Ctrl+A to exit
    pub fn requires_focus_mode(&self) -> bool {
        self.content.requires_focus_mode()
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.content.handle_key(key)
    }

    /// Handle mouse input (coordinates already translated to pane-local)
    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        self.content.handle_mouse(mouse)
    }

    /// Start text selection at the given coordinates
    pub fn start_selection(&mut self, x: u16, y: u16) {
        self.content.start_selection(x, y);
    }

    /// Update text selection to the given coordinates
    pub fn update_selection(&mut self, x: u16, y: u16) {
        self.content.update_selection(x, y);
    }

    /// End text selection
    pub fn end_selection(&mut self) {
        self.content.end_selection();
    }

    /// Get the currently selected text
    pub fn get_selected_text(&self) -> Option<String> {
        self.content.get_selected_text()
    }

    /// Clear the current selection
    pub fn clear_selection(&mut self) {
        self.content.clear_selection();
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        self.content.has_selection()
    }

    /// Notify pane content about focus state change
    pub fn set_focused(&mut self, focused: bool) {
        self.content.set_focused(focused);
    }

    /// Translate global mouse coordinates to pane-local coordinates
    pub fn translate_mouse(&self, mouse: MouseEvent) -> MouseEvent {
        MouseEvent {
            kind: mouse.kind,
            column: mouse.column.saturating_sub(self.area.x),
            row: mouse.row.saturating_sub(self.area.y),
            modifiers: mouse.modifiers,
        }
    }

    /// Check if point is within pane bounds
    pub fn contains_point(&self, x: u16, y: u16) -> bool {
        x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
    }

    /// Build the title line with optional icon
    fn build_title(&self, is_selected: bool, is_focused: bool) -> Line<'static> {
        let base_title = self.content.title_with_indicator(is_selected, is_focused);

        if let Some(ref icon) = self.icon {
            let icon_str = icon.clone();
            let title_str = base_title;
            Line::from(vec![
                Span::raw(" "),
                Span::raw(icon_str),
                Span::raw(" "),
                Span::raw(title_str),
                Span::raw(" "),
            ])
        } else {
            Line::from(format!(" {} ", base_title))
        }
    }

    /// Get the padded area (after applying padding)
    fn get_padded_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + self.padding.3,                                        // left
            y: area.y + self.padding.0,                                        // top
            width: area.width.saturating_sub(self.padding.1 + self.padding.3), // right + left
            height: area.height.saturating_sub(self.padding.0 + self.padding.2), // top + bottom
        }
    }

    /// Render the pane with border
    pub fn render(&mut self, frame: &mut ratatui::Frame, is_selected: bool, is_focused: bool) {
        let area = self.area;

        // Get border style from content
        let border_style = self.content.border_style(is_selected, is_focused);
        let title = self.build_title(is_selected, is_focused);

        // Create block with border
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .border_style(border_style)
            .title(title);

        // Add footer if present
        if let Some(ref footer) = self.text_footer {
            block = block.title_bottom(Line::from(format!(" {} ", footer)));
        }

        // Apply padding to the area
        let padded_area = self.get_padded_area(area);

        // Calculate inner area (inside border)
        let inner_area = block.inner(padded_area);

        // Render border
        frame.render_widget(block, padded_area);

        // Render content inside border
        self.content.render_content(inner_area, frame);
    }
}

// Helper for rendering - the content itself
impl Pane {
    /// Get a reference to render the content widget
    /// This is used during the render phase
    pub fn render_content(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        is_selected: bool,
        is_focused: bool,
    ) {
        // Calculate inner area (inside border)
        let border_style = self.content.border_style(is_selected, is_focused);
        let title = self.build_title(is_selected, is_focused);

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .border_style(border_style)
            .title(title);

        // Add footer if present
        if let Some(ref footer) = self.text_footer {
            block = block.title_bottom(Line::from(format!(" {} ", footer)));
        }

        // Apply padding to the area
        let padded_area = self.get_padded_area(area);

        // Render border
        block.render(padded_area, buf);

        // Render content (need to work around Widget consuming self)
        // For now, we'll document that PaneContent needs special handling
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyModifiers, MouseButton, MouseEventKind};

    // Mock pane content for testing
    struct MockPaneContent {
        title: String,
        focusable: bool,
        focused: bool,
        requires_focus: bool,
        last_key: Option<KeyEvent>,
        last_mouse: Option<MouseEvent>,
    }

    impl MockPaneContent {
        fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
                focusable: true,
                focused: false,
                requires_focus: false,
                last_key: None,
                last_mouse: None,
            }
        }

        fn non_focusable(title: &str) -> Self {
            Self {
                title: title.to_string(),
                focusable: false,
                focused: false,
                requires_focus: false,
                last_key: None,
                last_mouse: None,
            }
        }

        fn with_requires_focus(title: &str) -> Self {
            Self {
                title: title.to_string(),
                focusable: true,
                focused: false,
                requires_focus: true,
                last_key: None,
                last_mouse: None,
            }
        }
    }

    impl Widget for MockPaneContent {
        fn render(self, _area: Rect, _buf: &mut Buffer) {
            // No-op for testing
        }
    }

    impl PaneContent for MockPaneContent {
        fn handle_key(&mut self, key: KeyEvent) -> bool {
            self.last_key = Some(key);
            true
        }

        fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
            self.last_mouse = Some(mouse);
            true
        }

        fn title(&self) -> String {
            self.title.clone()
        }

        fn render_content(&mut self, _area: Rect, _frame: &mut ratatui::Frame) {
            // Mock implementation - do nothing
        }

        fn is_focusable(&self) -> bool {
            self.focusable
        }

        fn set_focused(&mut self, focused: bool) {
            self.focused = focused;
        }

        fn requires_focus_mode(&self) -> bool {
            self.requires_focus
        }
    }

    #[test]
    fn test_pane_creation() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        assert_eq!(pane.id(), pane_id);
        assert_eq!(pane.title(), "Test Pane");
        assert!(pane.is_focusable());
    }

    #[test]
    fn test_pane_non_focusable() {
        let pane_id = PaneId::new("status");
        let content = Box::new(MockPaneContent::non_focusable("Status"));
        let pane = Pane::new(pane_id, content);

        assert!(!pane.is_focusable());
    }

    #[test]
    fn test_pane_area() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test"));
        let mut pane = Pane::new(pane_id, content);

        let area = Rect::new(10, 20, 30, 40);
        pane.set_area(area);

        assert_eq!(pane.area(), area);
    }

    #[test]
    fn test_pane_contains_point() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test"));
        let mut pane = Pane::new(pane_id, content);

        pane.set_area(Rect::new(10, 20, 30, 40));

        // Inside
        assert!(pane.contains_point(15, 25));
        assert!(pane.contains_point(10, 20)); // Top-left corner
        assert!(pane.contains_point(39, 59)); // Bottom-right corner - 1

        // Outside
        assert!(!pane.contains_point(5, 25)); // Left
        assert!(!pane.contains_point(45, 25)); // Right
        assert!(!pane.contains_point(15, 15)); // Above
        assert!(!pane.contains_point(15, 65)); // Below
    }

    #[test]
    fn test_pane_translate_mouse() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test"));
        let mut pane = Pane::new(pane_id, content);

        pane.set_area(Rect::new(10, 20, 30, 40));

        let global_mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 25,
            row: 35,
            modifiers: KeyModifiers::empty(),
        };

        let local_mouse = pane.translate_mouse(global_mouse);

        assert_eq!(local_mouse.column, 15); // 25 - 10
        assert_eq!(local_mouse.row, 15); // 35 - 20
        assert_eq!(local_mouse.kind, global_mouse.kind);
    }

    #[test]
    fn test_title_with_indicator_focused() {
        let content = MockPaneContent::new("Test");
        let title = content.title_with_indicator(false, true);
        assert_eq!(title, "█ Test (Focused)");
    }

    #[test]
    fn test_title_with_indicator_selected() {
        let content = MockPaneContent::new("Test");
        let title = content.title_with_indicator(true, false);
        assert_eq!(title, "● Test (Selected)");
    }

    #[test]
    fn test_title_with_indicator_inactive() {
        let content = MockPaneContent::new("Test");
        let title = content.title_with_indicator(false, false);
        assert_eq!(title, "Test");
    }

    #[test]
    fn test_border_style_focused() {
        let content = MockPaneContent::new("Test");
        let style = content.border_style(false, true);
        assert_eq!(style.fg, Some(Color::Cyan));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_border_style_selected() {
        let content = MockPaneContent::new("Test");
        let style = content.border_style(true, false);
        assert_eq!(style.fg, Some(Color::Yellow));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_border_style_inactive() {
        let content = MockPaneContent::new("Test");
        let style = content.border_style(false, false);
        assert_eq!(style.fg, Some(Color::DarkGray));
    }

    // Tests for new features

    #[test]
    fn test_pane_with_icon() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_icon("🔥");

        assert_eq!(pane.icon, Some("🔥".to_string()));
    }

    #[test]
    fn test_pane_without_icon() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        assert_eq!(pane.icon, None);
    }

    #[test]
    fn test_pane_with_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_padding(1, 2, 3, 4);

        assert_eq!(pane.padding, (1, 2, 3, 4));
    }

    #[test]
    fn test_pane_with_uniform_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_uniform_padding(2);

        assert_eq!(pane.padding, (2, 2, 2, 2));
    }

    #[test]
    fn test_pane_default_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        assert_eq!(pane.padding, (0, 0, 0, 0));
    }

    #[test]
    fn test_pane_with_footer() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_footer("Status: Connected");

        assert_eq!(pane.text_footer, Some("Status: Connected".to_string()));
    }

    #[test]
    fn test_pane_without_footer() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        assert_eq!(pane.text_footer, None);
    }

    #[test]
    fn test_pane_with_border_type() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_border_type(BorderType::Double);

        assert_eq!(pane.border_type, BorderType::Double);
    }

    #[test]
    fn test_pane_default_border_type() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        assert_eq!(pane.border_type, BorderType::Rounded);
    }

    #[test]
    fn test_pane_builder_chaining() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content)
            .with_icon("🚀")
            .with_padding(1, 2, 3, 4)
            .with_footer("Footer text")
            .with_border_type(BorderType::Thick);

        assert_eq!(pane.icon, Some("🚀".to_string()));
        assert_eq!(pane.padding, (1, 2, 3, 4));
        assert_eq!(pane.text_footer, Some("Footer text".to_string()));
        assert_eq!(pane.border_type, BorderType::Thick);
    }

    #[test]
    fn test_get_padded_area_no_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content);

        let area = Rect::new(10, 20, 100, 50);
        let padded = pane.get_padded_area(area);

        assert_eq!(padded, area);
    }

    #[test]
    fn test_get_padded_area_with_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_padding(1, 2, 3, 4); // top, right, bottom, left

        let area = Rect::new(10, 20, 100, 50);
        let padded = pane.get_padded_area(area);

        // x = 10 + 4 (left) = 14
        // y = 20 + 1 (top) = 21
        // width = 100 - 2 (right) - 4 (left) = 94
        // height = 50 - 1 (top) - 3 (bottom) = 46
        assert_eq!(padded, Rect::new(14, 21, 94, 46));
    }

    #[test]
    fn test_get_padded_area_uniform_padding() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let pane = Pane::new(pane_id, content).with_uniform_padding(2);

        let area = Rect::new(10, 20, 100, 50);
        let padded = pane.get_padded_area(area);

        // x = 10 + 2 = 12
        // y = 20 + 2 = 22
        // width = 100 - 2 - 2 = 96
        // height = 50 - 2 - 2 = 46
        assert_eq!(padded, Rect::new(12, 22, 96, 46));
    }

    #[test]
    fn test_build_title_with_icon() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("My Pane"));
        let pane = Pane::new(pane_id, content).with_icon("📁");

        let title = pane.build_title(false, false);

        // Check that the title contains the icon
        let title_text = title
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();

        assert!(title_text.contains("📁"));
        assert!(title_text.contains("My Pane"));
    }

    #[test]
    fn test_build_title_without_icon() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("My Pane"));
        let pane = Pane::new(pane_id, content);

        let title = pane.build_title(false, false);

        let title_text = title
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();

        assert!(title_text.contains("My Pane"));
        assert!(!title_text.contains("📁"));
    }

    #[test]
    fn test_pane_set_focused() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let mut pane = Pane::new(pane_id, content);

        // Should not panic when setting focus
        pane.set_focused(true);
        pane.set_focused(false);
        pane.set_focused(true);
    }

    #[test]
    fn test_pane_set_focused_multiple_times() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test Pane"));
        let mut pane = Pane::new(pane_id, content);

        // Toggle focus multiple times - should not panic
        pane.set_focused(true);
        pane.set_focused(true); // Should remain true
        pane.set_focused(false);
        pane.set_focused(false); // Should remain false
        pane.set_focused(true);

        // The test passes if no panic occurs
    }

    #[test]
    fn test_mock_pane_content_focus_tracking() {
        // Test the mock implementation directly
        let mut mock = MockPaneContent::new("Test");

        // Initially not focused
        assert!(!mock.focused);

        // Set focused via trait method
        PaneContent::set_focused(&mut mock, true);
        assert!(mock.focused);

        // Set unfocused
        PaneContent::set_focused(&mut mock, false);
        assert!(!mock.focused);
    }

    // === REQUIRES_FOCUS_MODE TESTS ===

    #[test]
    fn test_requires_focus_mode_default_is_false() {
        let content = MockPaneContent::new("Test");
        assert!(!content.requires_focus_mode());
    }

    #[test]
    fn test_requires_focus_mode_can_return_true() {
        let content = MockPaneContent::with_requires_focus("Chat");
        assert!(content.requires_focus_mode());
    }

    #[test]
    fn test_pane_requires_focus_mode_wrapper() {
        let pane_id = PaneId::new("test");
        let content = Box::new(MockPaneContent::new("Test"));
        let pane = Pane::new(pane_id, content);

        // Default content returns false
        assert!(!pane.requires_focus_mode());
    }

    #[test]
    fn test_pane_requires_focus_mode_wrapper_true() {
        let pane_id = PaneId::new("chat");
        let content = Box::new(MockPaneContent::with_requires_focus("Chat"));
        let pane = Pane::new(pane_id, content);

        // Content with requires_focus returns true
        assert!(pane.requires_focus_mode());
    }

    #[test]
    fn test_title_with_indicator_selected_requires_focus() {
        let content = MockPaneContent::with_requires_focus("Chat");
        let title = content.title_with_indicator(true, false);
        assert_eq!(title, "● Chat (Press Enter)");
    }

    #[test]
    fn test_title_with_indicator_selected_no_requires_focus() {
        let content = MockPaneContent::new("TreeList");
        let title = content.title_with_indicator(true, false);
        assert_eq!(title, "● TreeList (Selected)");
    }

    #[test]
    fn test_title_with_indicator_focused_requires_focus() {
        // When focused, the indicator should be the same regardless of requires_focus_mode
        let content = MockPaneContent::with_requires_focus("Chat");
        let title = content.title_with_indicator(false, true);
        assert_eq!(title, "█ Chat (Focused)");
    }

    #[test]
    fn test_title_with_indicator_inactive_requires_focus() {
        // When inactive, the indicator should be the same regardless of requires_focus_mode
        let content = MockPaneContent::with_requires_focus("Chat");
        let title = content.title_with_indicator(false, false);
        assert_eq!(title, "Chat");
    }
}
