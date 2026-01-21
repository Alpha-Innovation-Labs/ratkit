//! SplitLayout widget with integrated hover and mouse handling.
//!
//! This widget wraps the SplitLayout primitive and provides:
//! - Mouse hover detection on dividers
//! - Drag-to-resize functionality
//! - Optional styling for dividers and panes
//! - Rendering support for pane borders and overlays

use crate::primitives::split_layout::{PaneLayout, SplitAxis, SplitDividerLayout, SplitLayout};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
    Frame,
};

/// State for SplitLayoutWidget interactions.
///
/// This can be stored in app state to preserve hover and drag
/// information across frames.
#[derive(Debug, Clone, Copy, Default)]
pub struct SplitLayoutWidgetState {
    /// Index of the divider currently being hovered
    pub hovered_divider: Option<usize>,
    /// Index of the divider currently being dragged
    pub dragging_divider: Option<usize>,
}

/// A widget that wraps SplitLayout with mouse interaction support.
///
/// This widget manages hover state, drag state, and handles mouse events
/// to resize dividers. It also provides styling options for visual feedback
/// during interactions.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::widgets::split_layout::{SplitLayoutWidget, SplitLayoutWidgetState};
/// use ratatui_toolkit::primitives::split_layout::SplitLayout;
///
/// let mut layout = SplitLayout::new(0);
/// layout.split_pane_vertically(0);
/// let mut state = SplitLayoutWidgetState::default();
///
/// let widget = SplitLayoutWidget::new(&mut layout)
///     .with_divider_width(1)
///     .with_hit_threshold(2);
/// ```
#[derive(Debug)]
pub struct SplitLayoutWidget<'a> {
    /// Reference to the underlying SplitLayout
    layout: &'a mut SplitLayout,
    /// State for hover and drag interactions
    state: SplitLayoutWidgetState,
    /// Width of divider lines in columns
    divider_width: u16,
    /// Hit detection threshold in columns/rows
    hit_threshold: u16,
    /// Style for hovered dividers
    hover_style: Style,
    /// Style for dragging dividers
    drag_style: Style,
    /// Style for normal dividers
    divider_style: Style,
    /// Optional block to render around the entire widget
    block: Option<Block<'a>>,
    /// Whether to show pane borders
    show_pane_borders: bool,
}

impl<'a> SplitLayoutWidget<'a> {
    /// Create a new SplitLayoutWidget wrapping a SplitLayout.
    pub fn new(layout: &'a mut SplitLayout) -> Self {
        Self {
            layout,
            state: SplitLayoutWidgetState::default(),
            divider_width: 1,
            hit_threshold: 2,
            hover_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            drag_style: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            divider_style: Style::default(),
            block: None,
            show_pane_borders: true,
        }
    }

    /// Set the widget state (for preserving hover/drag across frames).
    pub fn with_state(mut self, state: SplitLayoutWidgetState) -> Self {
        self.state = state;
        self
    }

    /// Get the current widget state (for saving after frame).
    pub fn state(&self) -> SplitLayoutWidgetState {
        self.state
    }

    /// Get the recommended event poll duration based on interaction state.
    ///
    /// Returns 8ms (~120fps) when dragging for smooth resizing,
    /// otherwise returns 50ms (normal rate).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let poll_timeout = widget.optimal_poll_duration();
    /// if event::poll(poll_timeout)? { ... }
    /// ```
    pub fn optimal_poll_duration(&self) -> std::time::Duration {
        if self.state.dragging_divider.is_some() {
            std::time::Duration::from_millis(8)
        } else {
            std::time::Duration::from_millis(50)
        }
    }

    /// Get a reference to the underlying layout.
    pub fn layout(&self) -> &SplitLayout {
        self.layout
    }

    /// Get a mutable reference to the underlying layout.
    pub fn layout_mut(&mut self) -> &mut SplitLayout {
        self.layout
    }

    /// Set the width of divider lines.
    pub fn with_divider_width(mut self, width: u16) -> Self {
        self.divider_width = width.max(1);
        self
    }

    /// Set the hit detection threshold for dividers.
    pub fn with_hit_threshold(mut self, threshold: u16) -> Self {
        self.hit_threshold = threshold.max(1);
        self
    }

    /// Set the style for hovered dividers.
    pub fn with_hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    /// Set the style for dragging dividers.
    pub fn with_drag_style(mut self, style: Style) -> Self {
        self.drag_style = style;
        self
    }

    /// Set the style for normal dividers.
    pub fn with_divider_style(mut self, style: Style) -> Self {
        self.divider_style = style;
        self
    }

    /// Set the block to render around the widget.
    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Enable or disable pane borders.
    pub fn with_pane_borders(mut self, show: bool) -> Self {
        self.show_pane_borders = show;
        self
    }

    /// Check if currently hovering over any divider.
    pub fn is_hovering(&self) -> bool {
        self.state.hovered_divider.is_some()
    }

    /// Check if currently dragging any divider.
    pub fn is_dragging(&self) -> bool {
        self.state.dragging_divider.is_some()
    }

    /// Get the currently hovered divider index, if any.
    pub fn hovered_divider(&self) -> Option<usize> {
        self.state.hovered_divider
    }

    /// Get the currently dragging divider index, if any.
    pub fn dragging_divider(&self) -> Option<usize> {
        self.state.dragging_divider
    }

    /// Handle a mouse event.
    ///
    /// This method processes mouse events and updates the widget's state:
    /// - Mouse move: Update hover state
    /// - Mouse down: Start dragging if on a divider
    /// - Mouse drag: Resize the divider
    /// - Mouse up: Stop dragging
    ///
    /// # Arguments
    ///
    /// * `mouse` - The mouse event to handle
    /// * `area` - The area the widget is rendered in
    pub fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) {
        match mouse.kind {
            MouseEventKind::Moved => {
                if self.state.dragging_divider.is_none() {
                    self.state.hovered_divider =
                        self.find_divider_at(mouse.column, mouse.row, area);
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(pane_id) = self.find_divider_at(mouse.column, mouse.row, area) {
                    self.state.dragging_divider = Some(pane_id);
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(pane_id) = self.state.dragging_divider {
                    self.resize_divider(pane_id, mouse.column, mouse.row, area);
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.state.dragging_divider = None;
                self.state.hovered_divider = self.find_divider_at(mouse.column, mouse.row, area);
            }
            _ => {}
        }
    }

    /// Find which split divider the mouse is over.
    ///
    /// Returns the split index if mouse is near a divider, or None otherwise.
    fn find_divider_at(&self, column: u16, row: u16, area: Rect) -> Option<usize> {
        let layouts = self.layout.layout_dividers(area);
        let threshold = self.hit_threshold;
        let mut best_match: Option<(usize, u16, u32)> = None;

        for divider in &layouts {
            let rect = divider.area();
            match divider.axis() {
                SplitAxis::Vertical => {
                    let divider_x = rect.x.saturating_add(
                        ((rect.width as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    let distance = divider_x.abs_diff(column);
                    if distance <= threshold
                        && column <= divider_x.saturating_add(threshold)
                        && row >= rect.y
                        && row <= rect.y.saturating_add(rect.height)
                    {
                        let area_size = rect.width as u32 * rect.height as u32;
                        if best_match
                            .map(|(_, best_distance, best_area)| {
                                distance < best_distance
                                    || (distance == best_distance && area_size < best_area)
                            })
                            .unwrap_or(true)
                        {
                            best_match = Some((divider.split_index(), distance, area_size));
                        }
                    }
                }
                SplitAxis::Horizontal => {
                    let divider_y = rect.y.saturating_add(
                        ((rect.height as u32 * divider.ratio() as u32) / 100) as u16,
                    );
                    let distance = divider_y.abs_diff(row);
                    if distance <= threshold
                        && row <= divider_y.saturating_add(threshold)
                        && column >= rect.x
                        && column <= rect.x.saturating_add(rect.width)
                    {
                        let area_size = rect.width as u32 * rect.height as u32;
                        if best_match
                            .map(|(_, best_distance, best_area)| {
                                distance < best_distance
                                    || (distance == best_distance && area_size < best_area)
                            })
                            .unwrap_or(true)
                        {
                            best_match = Some((divider.split_index(), distance, area_size));
                        }
                    }
                }
            }
        }

        best_match.map(|(split_index, _, _)| split_index)
    }

    /// Resize a divider based on mouse position.
    ///
    /// Calculates new split percentage based on mouse position and calls
    /// resize_divider on the SplitLayout.
    fn resize_divider(&mut self, split_index: usize, column: u16, row: u16, area: Rect) {
        let layouts = self.layout.layout_dividers(area);
        let divider_layout = layouts
            .iter()
            .find(|divider| divider.split_index() == split_index);

        if let Some(divider) = divider_layout {
            let rect = divider.area();
            match divider.axis() {
                SplitAxis::Vertical => {
                    let content_width = rect.width;
                    if content_width > 0 {
                        let relative_x = column.saturating_sub(rect.x);
                        let percent = ((relative_x as u32 * 100) / content_width as u32) as u16;
                        let _ = self.layout.resize_split(split_index, percent);
                    }
                }
                SplitAxis::Horizontal => {
                    let content_height = rect.height;
                    if content_height > 0 {
                        let relative_y = row.saturating_sub(rect.y);
                        let percent = ((relative_y as u32 * 100) / content_height as u32) as u16;
                        let _ = self.layout.resize_split(split_index, percent);
                    }
                }
            }
        }
    }

    /// Get the layout rectangles for all panes.
    ///
    /// This allows callers to render pane contents after the widget
    /// has drawn borders and overlays.
    pub fn pane_layouts(&self, area: Rect) -> Vec<PaneLayout> {
        self.layout.layout_panes(area)
    }
}

impl<'a> Widget for SplitLayoutWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut render_area = area;

        // Render outer block if provided
        if let Some(ref block) = self.block {
            let block = block.clone();
            render_area = block.inner(area);
            block.render(area, buf);
        }

        let pane_layouts = self.layout.layout_panes(render_area);
        let divider_layouts = self.layout.layout_dividers(render_area);

        // Render each pane with borders
        for pane_layout in &pane_layouts {
            let pane_id = pane_layout.pane_id();
            let pane_area = pane_layout.area();

            let border_style = self.divider_style;

            // Render pane border
            if self.show_pane_borders {
                let pane_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style)
                    .title(Line::from(format!(" {}", pane_id)));

                pane_block.render(pane_area, buf);
            }

            // Render divider overlay when hovered/dragging
        }

        for divider in &divider_layouts {
            let divider_style = if self.state.dragging_divider == Some(divider.split_index()) {
                self.drag_style
            } else if self.state.hovered_divider == Some(divider.split_index()) {
                self.hover_style
            } else {
                continue;
            };

            self.render_divider_overlay(divider, divider_style, buf);
        }
    }
}

impl<'a> SplitLayoutWidget<'a> {
    /// Render a visual overlay on the divider to indicate it's active.
    fn render_divider_overlay(
        &self,
        divider: &SplitDividerLayout,
        style: Style,
        buf: &mut ratatui::buffer::Buffer,
    ) {
        let width = self.divider_width;
        let rect = divider.area();

        match divider.axis() {
            SplitAxis::Vertical => {
                let divider_x = rect
                    .x
                    .saturating_add(((rect.width as u32 * divider.ratio() as u32) / 100) as u16);
                for y in rect.top()..rect.bottom() {
                    for dx in 0..width {
                        let x = divider_x.saturating_sub(dx);
                        let cell = buf.get_mut(x, y);
                        cell.set_style(style);
                        cell.set_char('│');
                    }
                }
            }
            SplitAxis::Horizontal => {
                let divider_y = rect
                    .y
                    .saturating_add(((rect.height as u32 * divider.ratio() as u32) / 100) as u16);
                for x in rect.left()..rect.right() {
                    for dy in 0..width {
                        let y = divider_y.saturating_sub(dy);
                        let cell = buf.get_mut(x, y);
                        cell.set_style(style);
                        cell.set_char('─');
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

    #[test]
    fn test_widget_creation() {
        let mut layout = SplitLayout::new(0);
        let widget = SplitLayoutWidget::new(&mut layout);
        assert!(!widget.is_hovering());
        assert!(!widget.is_dragging());
    }

    #[test]
    fn test_hover_on_vertical_divider() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_horizontally(0).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Test hovering near right edge of pane 0 (vertical divider at ~33%)
        let mouse = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 26, // Near divider at 33% of 80 = ~26
            row: 5,
            modifiers: KeyModifiers::empty(),
        };

        widget.handle_mouse(mouse, area);

        // Should detect hover on pane 0's divider
        assert!(widget.is_hovering());
        assert_eq!(widget.hovered_divider(), Some(0));
    }

    #[test]
    fn test_drag_vertical_divider() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_horizontally(0).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Start drag on divider
        let mouse_down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 26, // Near divider at 33%
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_down, area);

        // Should be dragging
        assert!(widget.is_dragging());
        assert_eq!(widget.dragging_divider(), Some(0));

        // Drag to new position (50%)
        let mouse_drag = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 40, // 50% of 80
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_drag, area);

        // Layout should be resized
        let layouts = layout.layout_panes(area);
        let pane_0_area = layouts.iter().find(|p| p.pane_id() == 0).unwrap().area();

        // Pane 0 should be approximately 50% width
        assert!(pane_0_area.width >= 38 && pane_0_area.width <= 42);
    }

    #[test]
    fn test_hover_on_horizontal_divider() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_vertically(0).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Test hovering near bottom edge of pane 0 (horizontal divider at ~50%)
        let mouse = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 40,
            row: 11, // Near 50% of 24 = ~12
            modifiers: KeyModifiers::empty(),
        };

        widget.handle_mouse(mouse, area);

        // Should detect hover on pane 0's divider
        assert!(widget.is_hovering());
        assert_eq!(widget.hovered_divider(), Some(0));
    }

    #[test]
    fn test_no_hover_in_middle_of_pane() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_horizontally(0).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Test mouse in middle of pane, not near edges
        let mouse = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 13, // Middle of left pane
            row: 12,
            modifiers: KeyModifiers::empty(),
        };

        widget.handle_mouse(mouse, area);

        // Should NOT detect hover
        assert!(!widget.is_hovering());
        assert!(widget.hovered_divider().is_none());
    }

    #[test]
    fn test_drag_stops_on_mouse_up() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_horizontally(0).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Start drag
        let mouse_down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 26,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_down, area);
        assert!(widget.is_dragging());

        // Stop drag
        let mouse_up = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 40,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_up, area);
        assert!(!widget.is_dragging());
    }

    #[test]
    fn test_divider_hit_threshold() {
        let mut layout = SplitLayout::new(0);
        let _pane_2 = layout.split_pane_horizontally(0).unwrap();

        let widget = SplitLayoutWidget::new(&mut layout).with_hit_threshold(5);

        assert_eq!(widget.hit_threshold, 5);
    }

    #[test]
    fn test_with_styling_methods() {
        let mut layout = SplitLayout::new(0);
        let hover_style = Style::default().fg(Color::Red);
        let drag_style = Style::default().fg(Color::Blue);
        let divider_style = Style::default().fg(Color::Green);

        let widget = SplitLayoutWidget::new(&mut layout)
            .with_hover_style(hover_style)
            .with_drag_style(drag_style)
            .with_divider_style(divider_style);

        // Styles should be set
        assert_eq!(widget.hover_style.fg, Some(Color::Red));
        assert_eq!(widget.drag_style.fg, Some(Color::Blue));
        assert_eq!(widget.divider_style.fg, Some(Color::Green));
    }

    #[test]
    fn test_complex_grid_layout_hover() {
        // Test showcase layout: 5 panes in a grid
        let mut layout = SplitLayout::new(0);
        let pane_2 = layout.split_pane_horizontally(0).unwrap();
        let _pane_3 = layout.split_pane_vertically(pane_2).unwrap();
        let _pane_4 = layout.split_pane_vertically(1).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);
        let dividers = layout.layout_dividers(area);
        let divider = dividers
            .iter()
            .find(|divider| divider.axis() == SplitAxis::Vertical)
            .unwrap();
        let divider_area = divider.area();
        let divider_x = divider_area
            .x
            .saturating_add(((divider_area.width as u32 * divider.ratio() as u32) / 100) as u16);

        // Hover over pane 2's divider
        let mouse = MouseEvent {
            kind: MouseEventKind::Moved,
            column: divider_x.saturating_sub(1),
            row: divider_area.y + divider_area.height / 2,
            modifiers: KeyModifiers::empty(),
        };

        widget.handle_mouse(mouse, area);

        // Should detect hover on the divider
        assert_eq!(widget.hovered_divider(), Some(divider.split_index()));
    }

    #[test]
    fn test_drag_resize_multiple_panes() {
        // Test that dragging affects correct pane in multi-pane layout
        let mut layout = SplitLayout::new(0);
        let pane_2 = layout.split_pane_horizontally(0).unwrap();
        let _pane_3 = layout.split_pane_vertically(pane_2).unwrap();

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        // Get initial layout
        let initial_layouts = layout.layout_panes(area);
        let initial_pane_0_width = initial_layouts
            .iter()
            .find(|p| p.pane_id() == 0)
            .unwrap()
            .area()
            .width;

        // Start drag on pane 0's divider
        let mouse_down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 26,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_down, area);

        // Drag to 60%
        let mouse_drag = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 48, // 60% of 80
            row: 5,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_drag, area);

        // Pane 0 should be wider now
        let final_layouts = layout.layout_panes(area);
        let final_pane_0_width = final_layouts
            .iter()
            .find(|p| p.pane_id() == 0)
            .unwrap()
            .area()
            .width;

        assert!(final_pane_0_width > initial_pane_0_width);
    }

    #[test]
    fn test_no_drag_on_single_pane() {
        // Single pane has no dividers, so dragging should not work
        let mut layout = SplitLayout::new(0);

        let mut widget = SplitLayoutWidget::new(&mut layout);

        let area = Rect::new(0, 0, 80, 24);

        let mouse_down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 40,
            row: 10,
            modifiers: KeyModifiers::empty(),
        };
        widget.handle_mouse(mouse_down, area);

        // Should NOT be dragging since there's no divider
        assert!(!widget.is_dragging());
        assert!(widget.dragging_divider().is_none());
    }
}
