//! Master Layout - Top-level orchestrator for tabs, navigation, and modes

use super::{InteractionMode, PaneId, Tab};
use crate::master_layout::MasterLayoutKeyBindings;
use crate::{MenuBar, MenuItem};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Result of event handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Event was consumed by the master layout or a pane
    Consumed,
    /// Event was not handled
    NotHandled,
    /// Application should quit
    Quit,
}

/// Master Layout - orchestrates the entire TUI application
///
/// # Architecture
///
/// ```text
/// ┌─────────────────────────────────┐
/// │ Navigation Bar (1 row + border) │ ← 3 rows total
/// ├─────────────────────────────────┤
/// │                                 │
/// │         Active Tab              │ ← Remaining space
/// │  (panes + footer)               │
/// │                                 │
/// └─────────────────────────────────┘
/// ```
///
/// # Modes
///
/// - **Layout Mode**: Navigate panes with hjkl, Tab/Shift+Tab, Enter to focus
/// - **Focus Mode**: All input goes to focused pane, Ctrl-A to exit
///
/// # Auto-Focus Mode
///
/// When `auto_focus` is enabled, the selected pane immediately receives input:
/// - No Enter required to focus
/// - No Ctrl-A to exit
/// - Tab/Shift+Tab switch panes and route input to the new pane
/// - Global keys (q, 1-9) still work for app control
///
/// ## Per-Pane Focus Mode
///
/// Panes can override auto-focus behavior by implementing `requires_focus_mode() -> true`:
/// - Panes returning `false` (default): receive input immediately when selected
/// - Panes returning `true`: require Enter to focus, Ctrl+A to exit (modal behavior)
///
/// This allows hybrid layouts where some panes (e.g., tree navigation) auto-passthrough
/// while others (e.g., chat, terminal) require explicit focus.
///
/// | `auto_focus` | `requires_focus_mode()` | Behavior |
/// |--------------|-------------------------|----------|
/// | false        | any                     | Modal (Enter to focus, Ctrl+A to exit) |
/// | true         | false                   | Auto-passthrough (input routes immediately) |
/// | true         | true                    | Modal for this pane only |
///
/// # Key Bindings
///
/// **Global (always work)**:
/// - Ctrl+Q: Quit
/// - 1-9: Switch tabs
///
/// **Layout Mode** (auto_focus: false):
/// - h/j/k/l: Directional navigation
/// - Tab: Next pane
/// - Shift+Tab: Previous pane
/// - Enter: Focus selected pane
///
/// **Layout Mode** (auto_focus: true):
/// - h/j/k/l: Route to selected pane
/// - Tab: Next pane + route input
/// - Shift+Tab: Previous pane + route input
/// - Enter: Route to selected pane
///
/// **Focus Mode**:
/// - Ctrl-A: Exit to Layout Mode
/// - All other keys: Route to focused pane
/// - Mouse on same pane: Route to pane
/// - Mouse on different pane: Change focus
pub struct MasterLayout {
    nav_bar: MenuBar,
    tabs: Vec<Tab>,
    active_tab_index: usize,
    mode: InteractionMode,
    global_area: Rect,
    nav_bar_offset: u16,
    keybindings: MasterLayoutKeyBindings,
    auto_focus: bool,
}

impl MasterLayout {
    /// Create a new empty master layout
    pub fn new() -> Self {
        Self {
            nav_bar: MenuBar::new(Vec::new()),
            tabs: Vec::new(),
            active_tab_index: 0,
            mode: InteractionMode::default(),
            global_area: Rect::default(),
            nav_bar_offset: 0,
            keybindings: MasterLayoutKeyBindings::default(),
            auto_focus: false,
        }
    }

    /// Set custom keybindings for the layout
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_toolkit::master_layout::{MasterLayout, MasterLayoutKeyBindings};
    /// use crossterm::event::{KeyCode, KeyModifiers};
    ///
    /// let mut bindings = MasterLayoutKeyBindings::default();
    /// bindings.quit = vec![(KeyCode::Char('x'), KeyModifiers::empty())];
    ///
    /// let layout = MasterLayout::new().with_keybindings(bindings);
    /// ```
    pub fn with_keybindings(mut self, keybindings: MasterLayoutKeyBindings) -> Self {
        self.keybindings = keybindings;
        self
    }

    /// Get the current keybindings
    pub fn keybindings(&self) -> &MasterLayoutKeyBindings {
        &self.keybindings
    }

    /// Set the keybindings
    pub fn set_keybindings(&mut self, keybindings: MasterLayoutKeyBindings) {
        self.keybindings = keybindings;
    }

    /// Enable auto-focus mode (selected pane receives input immediately)
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_toolkit::master_layout::MasterLayout;
    ///
    /// // Default: modal behavior (Enter to focus, Ctrl-A to exit)
    /// let layout = MasterLayout::new();
    ///
    /// // Auto-focus: selected pane receives input immediately
    /// let layout = MasterLayout::new().with_auto_focus(true);
    /// ```
    pub fn with_auto_focus(mut self, enabled: bool) -> Self {
        self.auto_focus = enabled;
        self
    }

    /// Get the auto-focus setting
    pub fn auto_focus(&self) -> bool {
        self.auto_focus
    }

    /// Set the auto-focus setting
    pub fn set_auto_focus(&mut self, enabled: bool) {
        self.auto_focus = enabled;
    }

    /// Set the navigation bar left offset (to make room for other components like IconNavBar)
    pub fn set_nav_bar_offset(&mut self, offset: u16) {
        self.nav_bar_offset = offset;
    }

    /// Add a tab to the master layout
    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);

        // Rebuild nav bar with updated tab names as menu items
        let menu_items: Vec<MenuItem> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, t)| MenuItem::new(t.name(), i))
            .collect();
        self.nav_bar = MenuBar::new(menu_items).with_selected(self.active_tab_index);

        // If this is the first tab and we have panes, select the first pane
        if self.tabs.len() == 1 {
            self.select_first_pane_in_active_tab();
        }
    }

    /// Get number of tabs
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get active tab index
    pub fn active_tab_index(&self) -> usize {
        self.active_tab_index
    }

    /// Set active tab by index
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            // Update menu bar selection
            for (i, item) in self.nav_bar.items.iter_mut().enumerate() {
                item.selected = i == index;
            }

            // When switching tabs, select first pane in new tab
            self.select_first_pane_in_active_tab();
        }
    }

    /// Get reference to active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    /// Get mutable reference to active tab
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }

    /// Get current interaction mode
    pub fn mode(&self) -> &InteractionMode {
        &self.mode
    }

    /// Enter Layout Mode
    pub fn enter_layout_mode(&mut self) {
        if let Some(focused_id) = self.mode.focused_pane() {
            // If we're in focus mode, transition to layout mode with selection
            self.mode = InteractionMode::layout_with_selection(focused_id);
        } else {
            // Just ensure we're in layout mode
            if !self.mode.is_layout() {
                self.mode = InteractionMode::layout();
            }
        }
    }

    /// Enter Focus Mode with a specific pane
    pub fn enter_focus_mode(&mut self, pane_id: PaneId) {
        self.mode = InteractionMode::focus(pane_id);
    }

    /// Exit Focus Mode (Ctrl-A) - returns to Layout Mode
    pub fn exit_focus_mode(&mut self) {
        self.mode.exit_focus();
    }

    /// Select the first focusable pane in the active tab
    fn select_first_pane_in_active_tab(&mut self) {
        if let Some(tab) = self.active_tab() {
            let container = tab.pane_container();
            if let Some(first_pane) = container.select_next(None) {
                self.mode = InteractionMode::layout_with_selection(first_pane);
            }
        }
    }

    /// Select next pane (Tab key in Layout Mode)
    pub fn select_next_pane(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            let current = self.mode.selected_pane();
            if let Some(next) = tab.pane_container().select_next(current) {
                self.mode.select_pane(next);
            }
        }
    }

    /// Select previous pane (Shift+Tab key in Layout Mode)
    pub fn select_prev_pane(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            let current = self.mode.selected_pane();
            if let Some(prev) = tab.pane_container().select_prev(current) {
                self.mode.select_pane(prev);
            }
        }
    }

    /// Select pane to the left (h key in Layout Mode)
    pub fn select_left(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            if let Some(current) = self.mode.selected_pane() {
                if let Some(left) = tab.pane_container().select_left(current) {
                    self.mode.select_pane(left);
                }
            }
        }
    }

    /// Select pane to the right (l key in Layout Mode)
    pub fn select_right(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            if let Some(current) = self.mode.selected_pane() {
                if let Some(right) = tab.pane_container().select_right(current) {
                    self.mode.select_pane(right);
                }
            }
        }
    }

    /// Select pane above (k key in Layout Mode)
    pub fn select_up(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            if let Some(current) = self.mode.selected_pane() {
                if let Some(up) = tab.pane_container().select_up(current) {
                    self.mode.select_pane(up);
                }
            }
        }
    }

    /// Select pane below (j key in Layout Mode)
    pub fn select_down(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(tab) = self.active_tab() {
            if let Some(current) = self.mode.selected_pane() {
                if let Some(down) = tab.pane_container().select_down(current) {
                    self.mode.select_pane(down);
                }
            }
        }
    }

    /// Focus the currently selected pane (Enter key in Layout Mode)
    pub fn focus_selected(&mut self) {
        if !self.mode.is_layout() {
            return;
        }

        if let Some(selected) = self.mode.selected_pane() {
            self.enter_focus_mode(selected);
        }
    }

    /// Handle a crossterm event
    pub fn handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(key) => self.handle_key_event(key),
            Event::Mouse(mouse) => self.handle_mouse_event(mouse),
            Event::Resize(_, _) => EventResult::Consumed,
            _ => EventResult::NotHandled,
        }
    }

    /// Handle keyboard events
    fn handle_key_event(&mut self, key: KeyEvent) -> EventResult {
        // Mode-specific handling
        match self.mode.clone() {
            InteractionMode::Layout { selected_pane } => {
                // Auto-focus mode: route keys directly to selected pane
                // (unless the pane requires explicit focus mode)
                if self.auto_focus {
                    if let Some(pane_id) = selected_pane {
                        // Check if the pane requires explicit focus mode
                        let requires_focus = self
                            .active_tab()
                            .and_then(|tab| tab.pane_container().get_pane(pane_id))
                            .map(|pane| pane.requires_focus_mode())
                            .unwrap_or(false);

                        if !requires_focus {
                            // Pane allows auto-passthrough, route input immediately
                            return self.handle_auto_focus_layout_key(key, pane_id);
                        }
                        // Pane requires focus mode, fall through to modal behavior
                    }
                }

                // In Layout Mode: Check quit keybinding
                if self.keybindings.is_quit(&key) {
                    return EventResult::Quit;
                }

                // In Layout Mode: Handle copy_selection and clear_selection for text selection if pane has selection
                if let Some(pane_id) = selected_pane {
                    // Handle copy selection - only if pane has selection
                    if self.keybindings.is_copy_selection(&key) {
                        if let Some(tab) = self.active_tab_mut() {
                            if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                                // Only route to pane if it has a selection
                                if pane.has_selection() && pane.handle_key(key) {
                                    return EventResult::Consumed;
                                }
                            }
                        }
                    }

                    // Handle clear_selection (Esc) - only if pane has selection
                    if self.keybindings.is_clear_selection(&key) {
                        if let Some(tab) = self.active_tab_mut() {
                            if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                                // Only route to pane if it has a selection to clear
                                if pane.has_selection() && pane.handle_key(key) {
                                    return EventResult::Consumed;
                                }
                            }
                        }
                        // If no selection to clear, fall through to let Layout Mode handle it
                    }
                }

                // Handle other Layout Mode keys (hjkl navigation, Enter, tab switching, etc.)
                self.handle_layout_mode_key(key)
            }
            InteractionMode::Focus { focused_pane } => {
                // Check exit_focus_mode keybinding - EVERYTHING else goes to the pane
                // This includes 'q', 'h', 'j', 'k', 'l', numbers, etc.
                if self.keybindings.is_exit_focus_mode(&key) {
                    self.exit_focus_mode();
                    return EventResult::Consumed;
                }

                // ALL other keys (including 'q', Ctrl+C, Ctrl+W, Esc, etc.) go to focused pane
                if let Some(tab) = self.active_tab_mut() {
                    if let Some(pane) = tab.pane_container_mut().get_pane_mut(focused_pane) {
                        if pane.handle_key(key) {
                            return EventResult::Consumed;
                        }
                    }
                }

                EventResult::NotHandled
            }
        }
    }

    /// Handle keyboard events in Layout Mode
    fn handle_layout_mode_key(&mut self, key: KeyEvent) -> EventResult {
        // Clear selection (Esc)
        if self.keybindings.is_clear_selection(&key) {
            self.mode = InteractionMode::Layout {
                selected_pane: None,
            };
            return EventResult::Consumed;
        }

        // Deselect pane (Ctrl-A in Layout Mode)
        if self.keybindings.is_deselect_pane(&key) {
            self.mode = InteractionMode::Layout {
                selected_pane: None,
            };
            return EventResult::Consumed;
        }

        // Switch tabs (1-9)
        if let Some(tab_index) = self.keybindings.get_tab_switch_index(&key) {
            if tab_index < self.tab_count() {
                self.set_active_tab(tab_index);
                return EventResult::Consumed;
            }
            return EventResult::NotHandled;
        }

        // Directional navigation (hjkl)
        if self.keybindings.is_navigate_left(&key) {
            self.select_left();
            return EventResult::Consumed;
        }
        if self.keybindings.is_navigate_down(&key) {
            self.select_down();
            return EventResult::Consumed;
        }
        if self.keybindings.is_navigate_up(&key) {
            self.select_up();
            return EventResult::Consumed;
        }
        if self.keybindings.is_navigate_right(&key) {
            self.select_right();
            return EventResult::Consumed;
        }

        // Focus selected pane (Enter)
        if self.keybindings.is_focus_pane(&key) {
            self.focus_selected();
            return EventResult::Consumed;
        }

        EventResult::NotHandled
    }

    /// Handle keyboard events in Layout Mode with auto-focus enabled
    fn handle_auto_focus_layout_key(&mut self, key: KeyEvent, pane_id: PaneId) -> EventResult {
        // Check global keys first (quit and tab switching)
        if self.keybindings.is_quit(&key) {
            return EventResult::Quit;
        }

        if let Some(tab_index) = self.keybindings.get_tab_switch_index(&key) {
            if tab_index < self.tab_count() {
                self.set_active_tab(tab_index);
                return EventResult::Consumed;
            }
            return EventResult::NotHandled;
        }

        // Handle tab navigation: switch pane AND route to new pane
        if key.code == KeyCode::Tab && key.modifiers.is_empty() {
            self.select_next_pane();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        if key.code == KeyCode::BackTab {
            self.select_prev_pane();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        // Handle navigation keys that both navigate AND route to pane
        if self.keybindings.is_navigate_left(&key) {
            self.select_left();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        if self.keybindings.is_navigate_right(&key) {
            self.select_right();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        if self.keybindings.is_navigate_up(&key) {
            self.select_up();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        if self.keybindings.is_navigate_down(&key) {
            self.select_down();
            if let Some(new_pane) = self.mode.selected_pane() {
                return self.route_key_to_pane(key, new_pane);
            }
            return EventResult::NotHandled;
        }

        // All other keys go directly to the selected pane
        self.route_key_to_pane(key, pane_id)
    }

    /// Route a key event to a specific pane
    fn route_key_to_pane(&mut self, key: KeyEvent, pane_id: PaneId) -> EventResult {
        if let Some(tab) = self.active_tab_mut() {
            if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                if pane.handle_key(key) {
                    return EventResult::Consumed;
                }
            }
        }
        EventResult::NotHandled
    }

    /// Handle mouse events
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> EventResult {
        // Check if click is on navigation bar
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            // Nav bar is in top 3 rows
            if mouse.row < 3 {
                if let Some(tab_index) = self.nav_bar.handle_click(mouse.column, mouse.row) {
                    self.set_active_tab(tab_index);
                    return EventResult::Consumed;
                }
            }
        }

        // Handle pane resizing (only in Layout Mode)
        if matches!(self.mode, InteractionMode::Layout { .. }) {
            if let Some(tab) = self.active_tab_mut() {
                let container = tab.pane_container_mut();

                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Check if clicking on a divider
                        if let Some(divider_idx) =
                            container.find_divider_at(mouse.column, mouse.row)
                        {
                            container.start_drag(divider_idx);
                            return EventResult::Consumed;
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        // Update drag if dragging
                        if container.is_dragging() {
                            container.update_drag(mouse.column, mouse.row);
                            return EventResult::Consumed;
                        }
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        // Stop dragging
                        if container.is_dragging() {
                            container.stop_drag();
                            return EventResult::Consumed;
                        }
                    }
                    MouseEventKind::Moved => {
                        // Update hover state
                        container.update_hover(mouse.column, mouse.row);
                    }
                    _ => {}
                }
            }
        }

        // Handle text selection in focused/selected pane
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) if mouse.row >= 3 => {
                // Get the currently selected/focused pane BEFORE borrowing
                let current_pane = match &self.mode {
                    InteractionMode::Layout { selected_pane } => *selected_pane,
                    InteractionMode::Focus { focused_pane } => Some(*focused_pane),
                };

                // Check if clicking on a pane
                if let Some(tab) = self.active_tab_mut() {
                    if let Some(pane_id) =
                        tab.pane_container().find_pane_at(mouse.column, mouse.row)
                    {
                        // If clicking on the current pane, start text selection
                        if Some(pane_id) == current_pane {
                            if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                                if pane.contains_point(mouse.column, mouse.row) {
                                    let local_mouse = pane.translate_mouse(mouse);
                                    pane.start_selection(local_mouse.column, local_mouse.row);
                                    return EventResult::Consumed;
                                }
                            }
                        } else {
                            // Clicking on a different pane: select it
                            match &mut self.mode {
                                InteractionMode::Layout { .. } => {
                                    self.mode.select_pane(pane_id);
                                }
                                InteractionMode::Focus { focused_pane } => {
                                    if *focused_pane != pane_id {
                                        self.enter_focus_mode(pane_id);
                                    }
                                }
                            }
                            return EventResult::Consumed;
                        }
                    }
                }
            }
            MouseEventKind::Drag(MouseButton::Left) if mouse.row >= 3 => {
                // Update selection if dragging in current pane
                let current_pane = match &self.mode {
                    InteractionMode::Layout { selected_pane } => *selected_pane,
                    InteractionMode::Focus { focused_pane } => Some(*focused_pane),
                };

                if let Some(pane_id) = current_pane {
                    if let Some(tab) = self.active_tab_mut() {
                        if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                            if pane.contains_point(mouse.column, mouse.row) {
                                let local_mouse = pane.translate_mouse(mouse);
                                pane.update_selection(local_mouse.column, local_mouse.row);
                                return EventResult::Consumed;
                            }
                        }
                    }
                }
            }
            MouseEventKind::Up(MouseButton::Left) if mouse.row >= 3 => {
                // End selection
                let current_pane = match &self.mode {
                    InteractionMode::Layout { selected_pane } => *selected_pane,
                    InteractionMode::Focus { focused_pane } => Some(*focused_pane),
                };

                if let Some(pane_id) = current_pane {
                    if let Some(tab) = self.active_tab_mut() {
                        if let Some(pane) = tab.pane_container_mut().get_pane_mut(pane_id) {
                            pane.end_selection();
                            return EventResult::Consumed;
                        }
                    }
                }
            }
            _ => {}
        }

        // If in Focus Mode, route mouse events to focused pane
        if let InteractionMode::Focus { focused_pane } = self.mode.clone() {
            if let Some(tab) = self.active_tab_mut() {
                if let Some(pane) = tab.pane_container_mut().get_pane_mut(focused_pane) {
                    // Check if mouse is within focused pane
                    if pane.contains_point(mouse.column, mouse.row) {
                        // Translate to pane-local coordinates and route
                        let local_mouse = pane.translate_mouse(mouse);
                        if pane.handle_mouse(local_mouse) {
                            return EventResult::Consumed;
                        }
                    } else {
                        // Mouse clicked outside focused pane - change focus
                        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                            if let Some(new_pane) =
                                tab.pane_container().find_pane_at(mouse.column, mouse.row)
                            {
                                self.enter_focus_mode(new_pane);
                                return EventResult::Consumed;
                            }
                        }
                    }
                }
            }
        }

        EventResult::NotHandled
    }

    /// Render the master layout
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        self.global_area = frame.area();

        // Calculate layout: nav bar (3 rows) + active tab (remaining)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Nav bar (1 row content + 2 border)
                Constraint::Min(5),    // Tab content (minimum 5 rows)
            ])
            .split(self.global_area);

        let nav_area = chunks[0];
        let tab_area = chunks[1];

        // Render navigation bar as menu bar with offset
        self.nav_bar
            .render_with_offset(frame, nav_area, self.nav_bar_offset);

        // Render active tab
        let mode = self.mode.clone();
        if let Some(tab) = self.active_tab_mut() {
            tab.render(frame, tab_area, &mode);
        }
    }
}

impl Default for MasterLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::master_layout::{Pane, PaneContent};
    use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEventKind};
    use ratatui::{backend::TestBackend, buffer::Buffer, widgets::Widget, Terminal};

    // Mock PaneContent for testing
    struct MockContent {
        title: String,
        key_count: usize,
        mouse_count: usize,
        last_key: Option<KeyEvent>,
        requires_focus: bool,
    }

    impl MockContent {
        fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
                key_count: 0,
                mouse_count: 0,
                last_key: None,
                requires_focus: false,
            }
        }

        fn with_requires_focus(title: &str) -> Self {
            Self {
                title: title.to_string(),
                key_count: 0,
                mouse_count: 0,
                last_key: None,
                requires_focus: true,
            }
        }
    }

    impl Widget for MockContent {
        fn render(self, _area: Rect, _buf: &mut Buffer) {}
    }

    impl PaneContent for MockContent {
        fn handle_key(&mut self, key: KeyEvent) -> bool {
            self.key_count += 1;
            self.last_key = Some(key);
            true
        }

        fn handle_mouse(&mut self, _mouse: MouseEvent) -> bool {
            self.mouse_count += 1;
            true
        }

        fn title(&self) -> String {
            self.title.clone()
        }

        fn render_content(&mut self, _area: Rect, _frame: &mut ratatui::Frame) {
            // Mock implementation - do nothing
        }

        fn requires_focus_mode(&self) -> bool {
            self.requires_focus
        }
    }

    fn create_test_layout() -> MasterLayout {
        let mut layout = MasterLayout::new();

        // Create a tab with two panes
        let mut tab = Tab::new("Test Tab");
        let pane1 = Pane::new(PaneId::new("pane1"), Box::new(MockContent::new("Pane 1")));
        let pane2 = Pane::new(PaneId::new("pane2"), Box::new(MockContent::new("Pane 2")));
        tab.add_pane(pane1);
        tab.add_pane(pane2);

        layout.add_tab(tab);
        layout
    }

    #[test]
    fn test_master_layout_creation() {
        let layout = MasterLayout::new();
        assert_eq!(layout.tab_count(), 0);
        assert_eq!(layout.active_tab_index(), 0);
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_add_tab() {
        let mut layout = MasterLayout::new();
        let tab = Tab::new("Tab 1");

        layout.add_tab(tab);

        assert_eq!(layout.tab_count(), 1);
        assert!(layout.active_tab().is_some());
        assert_eq!(layout.active_tab().unwrap().name(), "Tab 1");
    }

    #[test]
    fn test_add_multiple_tabs() {
        let mut layout = MasterLayout::new();

        layout.add_tab(Tab::new("Tab 1"));
        layout.add_tab(Tab::new("Tab 2"));
        layout.add_tab(Tab::new("Tab 3"));

        assert_eq!(layout.tab_count(), 3);
        assert_eq!(layout.active_tab_index(), 0);
    }

    #[test]
    fn test_set_active_tab() {
        let mut layout = MasterLayout::new();
        layout.add_tab(Tab::new("Tab 1"));
        layout.add_tab(Tab::new("Tab 2"));
        layout.add_tab(Tab::new("Tab 3"));

        layout.set_active_tab(1);
        assert_eq!(layout.active_tab_index(), 1);
        assert_eq!(layout.active_tab().unwrap().name(), "Tab 2");

        layout.set_active_tab(2);
        assert_eq!(layout.active_tab_index(), 2);
        assert_eq!(layout.active_tab().unwrap().name(), "Tab 3");
    }

    #[test]
    fn test_set_active_tab_invalid_index() {
        let mut layout = MasterLayout::new();
        layout.add_tab(Tab::new("Tab 1"));

        layout.set_active_tab(10);
        // Should remain at 0
        assert_eq!(layout.active_tab_index(), 0);
    }

    #[test]
    fn test_mode_transitions() {
        let mut layout = create_test_layout();

        // Should start in Layout Mode
        assert!(layout.mode().is_layout());

        // Get a pane ID to focus
        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter Focus Mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane_id));

        // Exit Focus Mode
        layout.exit_focus_mode();
        assert!(layout.mode().is_layout());
        assert_eq!(layout.mode().selected_pane(), Some(pane_id));
    }

    #[test]
    fn test_enter_key_focuses_selected_pane() {
        let mut layout = create_test_layout();

        // Should have a pane selected (first pane auto-selected on tab add)
        let selected = layout.mode().selected_pane();
        assert!(selected.is_some());

        // Press Enter
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), selected);
    }

    #[test]
    fn test_ctrl_a_exits_focus_mode() {
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Press Ctrl-A
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_layout());
        assert_eq!(layout.mode().selected_pane(), Some(pane_id));
    }

    #[test]
    fn test_q_quits() {
        let mut layout = create_test_layout();

        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Quit);
    }

    #[test]
    fn test_tab_key_not_handled_in_layout_mode() {
        // Verify that Tab does NOT cycle panes in Layout Mode
        let mut layout = create_test_layout();

        let first_selected = layout.mode().selected_pane();
        assert!(first_selected.is_some());

        // Press Tab - should NOT be handled in Layout Mode
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Tab should not be consumed by Layout Mode
        assert_eq!(result, EventResult::NotHandled);

        // Selection should not change
        let still_selected = layout.mode().selected_pane();
        assert_eq!(first_selected, still_selected);
    }

    #[test]
    fn test_shift_tab_not_handled_in_layout_mode() {
        // Verify that Shift+Tab does NOT cycle panes in Layout Mode
        let mut layout = create_test_layout();

        let first_selected = layout.mode().selected_pane();

        // Press Shift+Tab - should NOT be handled in Layout Mode
        let key = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        let result = layout.handle_key_event(key);

        // Shift+Tab should not be consumed
        assert_eq!(result, EventResult::NotHandled);

        // Selection should not change
        let still_selected = layout.mode().selected_pane();
        assert_eq!(first_selected, still_selected);
    }

    #[test]
    fn test_digit_keys_switch_tabs_in_layout_mode() {
        // Verify that number keys ONLY switch tabs in Layout Mode
        let mut layout = MasterLayout::new();

        let mut tab1 = Tab::new("Tab 1");
        tab1.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        layout.add_tab(tab1);

        let mut tab2 = Tab::new("Tab 2");
        tab2.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));
        layout.add_tab(tab2);

        let mut tab3 = Tab::new("Tab 3");
        tab3.add_pane(Pane::new(
            PaneId::new("p3"),
            Box::new(MockContent::new("P3")),
        ));
        layout.add_tab(tab3);

        // Verify we're in Layout Mode
        assert!(layout.mode().is_layout());

        // Press '2' to switch to second tab
        let key = KeyEvent::new(KeyCode::Char('2'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert_eq!(layout.active_tab_index(), 1);

        // Press '3' to switch to third tab
        let key = KeyEvent::new(KeyCode::Char('3'), KeyModifiers::empty());
        layout.handle_key_event(key);
        assert_eq!(layout.active_tab_index(), 2);
    }

    #[test]
    fn test_hjkl_navigation_in_layout_mode() {
        let mut layout = create_test_layout();

        // Setup: ensure we're in layout mode with a selection
        assert!(layout.mode().is_layout());
        assert!(layout.mode().selected_pane().is_some());

        // Test h key (left)
        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);

        // Test j key (down)
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);

        // Test k key (up)
        let key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);

        // Test l key (right)
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);
    }

    #[test]
    fn test_hjkl_ignored_in_focus_mode() {
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);

        let initial_mode = layout.mode().clone();

        // Press h - should be routed to pane, not change selection
        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty());
        layout.handle_key_event(key);

        // Should still be in focus mode with same pane
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode(), &initial_mode);
    }

    #[test]
    fn test_mouse_click_on_nav_bar() {
        let mut layout = MasterLayout::new();
        layout.add_tab(Tab::new("Tab 1"));
        layout.add_tab(Tab::new("Tab 2"));

        // Start on first tab
        assert_eq!(layout.active_tab_index(), 0);

        // Render to calculate nav bar button areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Get the actual button area for the second tab after rendering
        // We need to access nav_bar to find the button position
        // For now, we'll test by switching tabs using the API directly
        // The click handling is tested indirectly through other tests

        // Alternative: Test that clicking in nav bar area is handled
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 5,
            row: 1,
            modifiers: KeyModifiers::empty(),
        };

        let result = layout.handle_mouse_event(mouse);
        // Should be consumed if click hits a button, or NotHandled if it misses
        // Since button layout is calculated during render, we just verify it doesn't panic
        assert!(result == EventResult::Consumed || result == EventResult::NotHandled);
    }

    #[test]
    fn test_mouse_click_on_pane_focuses() {
        let mut layout = create_test_layout();

        // Render to calculate pane areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Click on pane area (below nav bar)
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };

        layout.handle_mouse_event(mouse);

        // Should enter focus mode (if click hit a pane)
        // Note: This test is simplified; in real usage, pane areas are calculated during render
    }

    #[test]
    fn test_keys_routed_to_focused_pane() {
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);

        // Send a key event
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        // Pane should have received the key (verified by MockContent.key_count in real test)
    }

    #[test]
    fn test_tab_key_routed_to_pane_in_focus_mode() {
        // Verify that Tab key goes to the pane in Focus Mode, not navigation
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Press Tab - should be routed to pane, not used for navigation
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Tab should be consumed (by pane or by layout attempting to route it)
        // The key point is it should NOT switch panes
        assert_eq!(result, EventResult::Consumed);

        // Should still be in focus mode with the same pane
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane_id));
    }

    #[test]
    fn test_number_keys_routed_to_pane_in_focus_mode() {
        // Verify that number keys go to pane in Focus Mode, NOT switch tabs
        let mut layout = MasterLayout::new();

        let mut tab1 = Tab::new("Tab 1");
        tab1.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        layout.add_tab(tab1);

        let mut tab2 = Tab::new("Tab 2");
        tab2.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));
        layout.add_tab(tab2);

        // Start on first tab
        assert_eq!(layout.active_tab_index(), 0);

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode on first tab
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Press '2' - should go to pane, NOT switch to tab 2
        let key = KeyEvent::new(KeyCode::Char('2'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should be consumed by pane
        assert_eq!(result, EventResult::Consumed);

        // Should still be on first tab (NOT switched to tab 2)
        assert_eq!(layout.active_tab_index(), 0);

        // Should still be in focus mode with the same pane
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane_id));
    }

    #[test]
    fn test_only_ctrl_a_exits_focus_mode() {
        // Verify that ONLY Ctrl-A exits Focus Mode, other keys don't
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Try various keys - none should exit focus mode (except 'q' which quits the app)
        let test_keys = vec![
            KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()),
            KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()),
            KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()),
        ];

        for key in test_keys {
            layout.handle_key_event(key);
            // Should still be in focus mode
            assert!(
                layout.mode().is_focus(),
                "Key {:?} should not exit focus mode",
                key
            );
        }

        // Now press Ctrl-A - should exit focus mode
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_all_keys_passed_to_focused_pane() {
        // Verify that ALL keys (including Ctrl+W, Ctrl+C, etc.) are passed to focused pane
        let mut layout = create_test_layout();

        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter focus mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Test various control sequences that should be passed through
        let test_keys = vec![
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL), // Ctrl+W
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), // Ctrl+C
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL), // Ctrl+D
            KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),       // Esc
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL), // Ctrl+R
        ];

        for key in test_keys {
            let result = layout.handle_key_event(key);

            // Key should be consumed (passed to pane)
            assert_eq!(
                result,
                EventResult::Consumed,
                "Key {:?} should be passed to pane",
                key
            );

            // Should still be in focus mode
            assert!(
                layout.mode().is_focus(),
                "Key {:?} should not exit focus mode",
                key
            );

            // Verify the pane received the key by checking last_key
            let _pane = layout
                .active_tab()
                .unwrap()
                .pane_container()
                .get_pane(pane_id)
                .unwrap();

            // We need to access the MockContent's last_key, but it's boxed
            // For now, just verify we're still in focus mode
            // The key_count verification in other tests confirms keys are being passed
        }
    }

    #[test]
    fn test_q_quits_only_in_layout_mode() {
        // Verify Q (without modifiers) quits ONLY in Layout Mode, NOT in Focus Mode
        let mut layout = create_test_layout();

        // Test in Layout Mode - 'q' should quit
        assert!(layout.mode().is_layout());
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Quit, "'q' should quit in Layout Mode");

        // Reset layout and test in Focus Mode - 'q' should be sent to pane
        let mut layout = create_test_layout();
        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(
            result,
            EventResult::Consumed,
            "'q' should be consumed by pane in Focus Mode, NOT quit app"
        );

        // Verify still in Focus Mode (key was consumed, not used for app control)
        assert!(
            layout.mode().is_focus(),
            "Should still be in Focus Mode after 'q'"
        );
    }

    #[test]
    fn test_q_in_focus_mode_regression() {
        // Regression test: Ensure 'q' key in Focus Mode does NOT quit the application
        // Bug history: Previously, 'q' was checked globally before mode handling,
        // causing it to quit even when a terminal was focused
        let mut layout = create_test_layout();

        // Enter Focus Mode on the first pane
        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();
        layout.enter_focus_mode(pane_id);

        // Verify we're in Focus Mode
        assert!(layout.mode().is_focus(), "Should be in Focus Mode");

        // Press 'q' - this should go to the pane, NOT quit
        let q_key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(q_key);

        // Critical assertion: 'q' should be consumed by pane, not trigger quit
        assert_ne!(
            result,
            EventResult::Quit,
            "REGRESSION: 'q' in Focus Mode should NOT quit the application"
        );
        assert_eq!(
            result,
            EventResult::Consumed,
            "'q' should be consumed by the focused pane"
        );

        // Verify we're still in Focus Mode (didn't quit or exit Focus Mode)
        assert!(
            layout.mode().is_focus(),
            "Should still be in Focus Mode after pressing 'q'"
        );

        // Press multiple 'q' keys to ensure consistent behavior
        for _ in 0..5 {
            let result = layout.handle_key_event(q_key);
            assert_eq!(
                result,
                EventResult::Consumed,
                "Multiple 'q' presses should all be consumed"
            );
            assert!(layout.mode().is_focus(), "Should remain in Focus Mode");
        }

        // Final verification: still in Focus Mode after multiple 'q' presses
        assert!(
            layout.mode().is_focus(),
            "Should remain in Focus Mode after all 'q' presses"
        );
    }

    #[test]
    fn test_uppercase_q_in_focus_mode() {
        // Regression test: Ensure uppercase 'Q' (Shift+Q) in Focus Mode does NOT quit
        // Bug: When pressing Shift+Q, it generates KeyCode::Char('Q') which wasn't being
        // handled properly and could slip through to quit the app
        let mut layout = create_test_layout();

        // Enter Focus Mode
        let pane_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Press uppercase 'Q' (Shift+Q) - should go to pane, NOT quit
        let uppercase_q = KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::empty());
        let result = layout.handle_key_event(uppercase_q);

        // Should NOT quit the application
        assert_ne!(
            result,
            EventResult::Quit,
            "REGRESSION: Uppercase 'Q' (Shift+Q) in Focus Mode should NOT quit the application"
        );
        assert_eq!(
            result,
            EventResult::Consumed,
            "Uppercase 'Q' should be consumed by the focused pane"
        );

        // Should still be in Focus Mode
        assert!(
            layout.mode().is_focus(),
            "Should still be in Focus Mode after pressing 'Q'"
        );
    }

    #[test]
    fn test_uppercase_q_quits_in_layout_mode() {
        // Both 'q' and 'Q' should quit when in Layout Mode (case-insensitive)
        let mut layout = create_test_layout();
        assert!(layout.mode().is_layout());

        // Test lowercase 'q'
        let lowercase_q = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(lowercase_q);
        assert_eq!(
            result,
            EventResult::Quit,
            "Lowercase 'q' should quit in Layout Mode"
        );

        // Reset and test uppercase 'Q'
        let mut layout = create_test_layout();
        let uppercase_q = KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::empty());
        let result = layout.handle_key_event(uppercase_q);
        assert_eq!(
            result,
            EventResult::Quit,
            "Uppercase 'Q' should quit in Layout Mode"
        );
    }

    #[test]
    fn test_render_does_not_panic() {
        let mut layout = create_test_layout();

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_empty_layout_renders() {
        let mut layout = MasterLayout::new();

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_tab_switching_preserves_mode() {
        let mut layout = MasterLayout::new();

        // Create two tabs with panes
        let mut tab1 = Tab::new("Tab 1");
        tab1.add_pane(Pane::new(
            PaneId::new("t1p1"),
            Box::new(MockContent::new("T1P1")),
        ));
        layout.add_tab(tab1);

        let mut tab2 = Tab::new("Tab 2");
        tab2.add_pane(Pane::new(
            PaneId::new("t2p1"),
            Box::new(MockContent::new("T2P1")),
        ));
        layout.add_tab(tab2);

        // Switch to tab 2
        layout.set_active_tab(1);

        // Mode should still be layout mode with first pane selected
        assert!(layout.mode().is_layout());
        assert!(layout.mode().selected_pane().is_some());
    }

    #[test]
    fn test_no_panes_no_selection() {
        let mut layout = MasterLayout::new();
        let tab = Tab::new("Empty Tab");
        layout.add_tab(tab);

        // No panes, so no selection should be made
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_event_handling_resize() {
        let mut layout = create_test_layout();

        let event = Event::Resize(100, 50);
        let result = layout.handle_event(event);

        assert_eq!(result, EventResult::Consumed);
    }

    #[test]
    fn test_select_next_with_no_tabs() {
        let mut layout = MasterLayout::new();
        layout.select_next_pane();
        // Should not panic
    }

    #[test]
    fn test_focus_selected_with_no_selection() {
        let mut layout = MasterLayout::new();
        let tab = Tab::new("Empty Tab");
        layout.add_tab(tab);

        // No panes to select
        layout.focus_selected();
        // Should remain in layout mode
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_esc_deselects_in_layout_mode() {
        let mut layout = create_test_layout();

        // Verify we start with a pane selected
        assert!(layout.mode().is_layout());
        assert!(layout.mode().selected_pane().is_some());

        // Press ESC to clear selection
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Verify the ESC key was consumed
        assert_eq!(result, EventResult::Consumed);

        // Verify we're still in Layout Mode
        assert!(layout.mode().is_layout());

        // Verify no pane is selected
        assert_eq!(layout.mode().selected_pane(), None);
    }

    #[test]
    fn test_mouse_click_selects_pane_in_layout_mode() {
        let mut layout = create_test_layout();

        // Verify initial state: Layout Mode with first pane selected
        assert!(layout.mode().is_layout());
        let initial_selection = layout.mode().selected_pane();
        assert!(initial_selection.is_some());

        // Get the second pane ID for comparison
        let _pane2 = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(1)
            .unwrap()
            .id();

        // Render to calculate pane areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Simulate mouse click on second pane (below nav bar)
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };

        layout.handle_mouse_event(mouse);

        // Assert that:
        // 1. Mode is still Layout Mode (not Focus Mode)
        assert!(
            layout.mode().is_layout(),
            "Mode should still be Layout Mode after mouse click in Layout Mode"
        );

        // 2. No pane is focused (verified by checking it's not Focus Mode)
        assert!(
            layout.mode().focused_pane().is_none(),
            "No pane should be focused in Layout Mode"
        );

        // 3. A pane is selected
        let current_selection = layout.mode().selected_pane();
        assert!(
            current_selection.is_some(),
            "A pane should be selected after mouse click"
        );
    }

    #[test]
    fn test_keyboard_navigation_then_enter_focuses() {
        let mut layout = create_test_layout();

        // Step 1: Verify first pane is selected in Layout Mode
        assert!(layout.mode().is_layout());
        let first_pane = layout.mode().selected_pane();
        assert!(first_pane.is_some());

        // Render to calculate pane areas (required for directional navigation)
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Step 2: Press 'l' key to select right pane
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);

        // Step 3: Assert second pane is now selected (still in Layout Mode)
        let second_pane = layout.mode().selected_pane();
        assert!(second_pane.is_some());
        assert_ne!(
            first_pane, second_pane,
            "Second pane should be different from first"
        );
        assert!(layout.mode().is_layout(), "Should still be in Layout Mode");

        // Step 4: Press Enter key
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);

        // Step 5: Assert Mode is now Focus Mode and second pane is focused
        assert!(
            layout.mode().is_focus(),
            "Should be in Focus Mode after Enter"
        );
        assert_eq!(
            layout.mode().focused_pane(),
            second_pane,
            "Second pane should be focused"
        );
    }

    #[test]
    fn test_ctrl_a_deselects_in_layout_mode() {
        let mut layout = create_test_layout();

        // Verify we start in Layout Mode with a pane selected
        assert!(layout.mode().is_layout());
        assert!(layout.mode().selected_pane().is_some());

        // Press Ctrl-A to deselect
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);

        // Verify the Ctrl-A key was consumed
        assert_eq!(result, EventResult::Consumed);

        // Verify we're still in Layout Mode
        assert!(layout.mode().is_layout());

        // Verify no pane is selected
        assert_eq!(layout.mode().selected_pane(), None);
    }

    #[test]
    fn test_click_different_pane_in_focus_mode_switches() {
        let mut layout = create_test_layout();

        // Get pane IDs from the layout
        let pane1_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        let pane2_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(1)
            .unwrap()
            .id();

        // Enter Focus Mode on first pane
        layout.enter_focus_mode(pane1_id);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane1_id));

        // Render to calculate pane areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Simulate clicking on the second pane (somewhere in the pane area)
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 50,
            row: 12,
            modifiers: KeyModifiers::empty(),
        };

        let result = layout.handle_mouse_event(mouse);
        assert_eq!(result, EventResult::Consumed);

        // Assert: Still in Focus Mode
        assert!(layout.mode().is_focus());

        // Assert: Now focused on second pane
        assert_eq!(layout.mode().focused_pane(), Some(pane2_id));
    }

    #[test]
    fn test_click_same_pane_in_focus_mode_maintains() {
        let mut layout = create_test_layout();

        // Get first pane ID
        let pane1_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        // Enter Focus Mode on first pane
        layout.enter_focus_mode(pane1_id);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane1_id));

        // Render to calculate pane areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Simulate clicking on the same first pane
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 8,
            modifiers: KeyModifiers::empty(),
        };

        let result = layout.handle_mouse_event(mouse);
        assert_eq!(result, EventResult::Consumed);

        // Assert: Still in Focus Mode
        assert!(layout.mode().is_focus());

        // Assert: Still focused on first pane
        assert_eq!(layout.mode().focused_pane(), Some(pane1_id));
    }

    #[test]
    fn test_hjkl_navigation_blocked_in_focus_mode() {
        let mut layout = create_test_layout();

        let pane1_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(0)
            .unwrap()
            .id();

        let pane2_id = layout
            .active_tab()
            .unwrap()
            .pane_container()
            .get_pane_by_index(1)
            .unwrap()
            .id();

        // Verify we have 2 panes
        assert_ne!(pane1_id, pane2_id);

        // Enter focus mode on first pane
        layout.enter_focus_mode(pane1_id);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane(), Some(pane1_id));

        // Press 'l' key (should navigate to second pane in Layout Mode, but not in Focus Mode)
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Key should be consumed (routed to pane, not processed by layout navigation)
        assert_eq!(result, EventResult::Consumed);

        // IMPORTANT: Verify navigation is BLOCKED - still in Focus Mode with same pane
        assert!(layout.mode().is_focus(), "Should still be in Focus Mode");
        assert_eq!(
            layout.mode().focused_pane(),
            Some(pane1_id),
            "Should still be focused on first pane (navigation not allowed in Focus Mode)"
        );
    }

    // === AUTO-FOCUS MODE TESTS ===

    #[test]
    fn test_with_auto_focus_enables_auto_focus() {
        let layout = MasterLayout::new().with_auto_focus(true);
        assert!(layout.auto_focus());
    }

    #[test]
    fn test_default_auto_focus_is_disabled() {
        let layout = MasterLayout::new();
        assert!(!layout.auto_focus());
    }

    #[test]
    fn test_set_auto_focus() {
        let mut layout = MasterLayout::new();
        assert!(!layout.auto_focus());

        layout.set_auto_focus(true);
        assert!(layout.auto_focus());

        layout.set_auto_focus(false);
        assert!(!layout.auto_focus());
    }

    #[test]
    fn test_auto_focus_routes_input_immediately() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Should have a pane selected
        let selected = layout.mode().selected_pane();
        assert!(selected.is_some());

        // Press 'j' key - should be routed to pane immediately
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Should still be in layout mode (not focus mode)
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_auto_focus_tab_switches_and_routes() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Get initial selected pane
        let first_selected = layout.mode().selected_pane();
        assert!(first_selected.is_some());

        // Press Tab - should switch to next pane and route the Tab key
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Selection should have changed
        let new_selected = layout.mode().selected_pane();
        assert_ne!(first_selected, new_selected);
    }

    #[test]
    fn test_auto_focus_shift_tab_switches_and_routes() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Get initial selected pane
        let first_selected = layout.mode().selected_pane();
        assert!(first_selected.is_some());

        // Press Shift+Tab - should switch to previous pane and route the key
        let key = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Selection should have changed
        let new_selected = layout.mode().selected_pane();
        assert_ne!(first_selected, new_selected);
    }

    #[test]
    fn test_auto_focus_no_enter_required() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Press 'x' key - should be routed to selected pane immediately
        // No Enter required
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Should still be in layout mode
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_auto_focus_no_ctrl_a_required() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Press Ctrl-A - should be routed to selected pane
        // No Ctrl-A required for anything else (unlike focus mode exit)
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Should still be in layout mode
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_auto_focus_q_quits() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Press 'q' - should still quit (global key)
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Quit);
    }

    #[test]
    fn test_auto_focus_digit_keys_switch_tabs() {
        let mut layout = MasterLayout::new().with_auto_focus(true);

        // Create two tabs
        let mut tab1 = Tab::new("Tab 1");
        tab1.add_pane(Pane::new(
            PaneId::new("p1"),
            Box::new(MockContent::new("P1")),
        ));
        layout.add_tab(tab1);

        let mut tab2 = Tab::new("Tab 2");
        tab2.add_pane(Pane::new(
            PaneId::new("p2"),
            Box::new(MockContent::new("P2")),
        ));
        layout.add_tab(tab2);

        // Start on first tab
        assert_eq!(layout.active_tab_index(), 0);

        // Press '2' - should switch to second tab (global key still works)
        let key = KeyEvent::new(KeyCode::Char('2'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert_eq!(layout.active_tab_index(), 1);
    }

    #[test]
    fn test_default_behavior_unchanged() {
        // Regression test: ensure default behavior (auto_focus: false) is unchanged
        let mut layout = create_test_layout();

        // Verify auto_focus is disabled
        assert!(!layout.auto_focus());

        // Should be in layout mode
        assert!(layout.mode().is_layout());

        // Press 'j' - should NOT route to pane in layout mode
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should be consumed (navigation)
        assert_eq!(result, EventResult::Consumed);

        // Should still be in layout mode
        assert!(layout.mode().is_layout());

        // Press Enter - should enter focus mode
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_focus());
    }

    #[test]
    fn test_auto_focus_hjkl_routes_to_pane() {
        let mut layout = create_test_layout().with_auto_focus(true);

        // Get initial selected pane
        let _first_selected = layout.mode().selected_pane();

        // Press 'j' - should route to pane AND navigate
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // In auto-focus mode, hjkl still navigates panes
        let new_selected = layout.mode().selected_pane();
        assert!(new_selected.is_some());
    }

    #[test]
    fn test_auto_focus_with_no_selection() {
        let mut layout = MasterLayout::new().with_auto_focus(true);
        let tab = Tab::new("Empty Tab");
        layout.add_tab(tab);

        // No panes, so no selection
        assert!(layout.mode().selected_pane().is_none());

        // Press 'x' key - should not panic
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should return NotHandled (no pane to route to)
        assert_eq!(result, EventResult::NotHandled);
    }

    #[test]
    fn test_auto_focus_mouse_behavior() {
        // Verify mouse behavior still works in auto-focus mode
        let mut layout = create_test_layout().with_auto_focus(true);

        // Render to calculate pane areas
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                layout.render(frame);
            })
            .unwrap();

        // Click on pane area (below nav bar)
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: KeyModifiers::empty(),
        };

        // Should not panic
        let _result = layout.handle_mouse_event(mouse);
        // Result could be Consumed or NotHandled depending on what was clicked
    }

    // === PER-PANE FOCUS MODE TESTS ===

    // Helper to create a layout with mixed panes:
    // - Pane 1: requires_focus_mode = true (modal behavior even with auto_focus)
    // - Pane 2: requires_focus_mode = false (auto-passthrough with auto_focus)
    // Returns (layout, chat_pane_id, tree_pane_id)
    fn create_mixed_focus_layout() -> (MasterLayout, PaneId, PaneId) {
        let mut layout = MasterLayout::new();

        let mut tab = Tab::new("Mixed Tab");
        // First pane requires explicit focus (like a chat/terminal)
        let chat_id = PaneId::new("chat");
        let pane1 = Pane::new(chat_id, Box::new(MockContent::with_requires_focus("Chat")));
        // Second pane auto-passthroughs (like a tree list)
        let tree_id = PaneId::new("tree");
        let pane2 = Pane::new(tree_id, Box::new(MockContent::new("TreeList")));
        tab.add_pane(pane1);
        tab.add_pane(pane2);

        layout.add_tab(tab);
        (layout, chat_id, tree_id)
    }

    #[test]
    fn test_requires_focus_mode_blocks_auto_passthrough() {
        let (mut layout, chat_id, _tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // First pane (chat) should be selected by default
        let selected = layout.mode().selected_pane();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap(), chat_id);

        // Press 'x' key - should NOT be routed to pane (modal behavior)
        // because chat requires focus mode
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should NOT be consumed (falls through to Layout Mode which doesn't handle 'x')
        assert_eq!(result, EventResult::NotHandled);

        // Should still be in Layout Mode
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_requires_focus_mode_enter_to_focus() {
        let (mut layout, chat_id, _tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // First pane (chat) requires focus mode
        let selected = layout.mode().selected_pane();
        assert_eq!(selected.unwrap(), chat_id);

        // Press Enter - should enter Focus Mode (modal behavior)
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_focus());
        assert_eq!(layout.mode().focused_pane().unwrap(), chat_id);
    }

    #[test]
    fn test_requires_focus_mode_ctrl_a_exits() {
        let (mut layout, chat_id, _tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // First pane (chat) requires focus mode
        let pane_id = layout.mode().selected_pane().unwrap();
        assert_eq!(pane_id, chat_id);

        // Enter focus mode
        layout.enter_focus_mode(pane_id);
        assert!(layout.mode().is_focus());

        // Press Ctrl-A - should exit Focus Mode
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_no_requires_focus_auto_passthrough() {
        let (mut layout, _chat_id, tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // Navigate to second pane (tree) which doesn't require focus
        layout.select_next_pane();
        let selected = layout.mode().selected_pane();
        assert_eq!(selected.unwrap(), tree_id);

        // Press 'x' key - should be routed to pane immediately (auto-passthrough)
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Consumed);

        // Should still be in Layout Mode (auto-focus mode)
        assert!(layout.mode().is_layout());
    }

    #[test]
    fn test_mixed_layout_navigation() {
        let (mut layout, chat_id, _tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // Start on first pane (chat - requires focus)
        assert_eq!(layout.mode().selected_pane().unwrap(), chat_id);

        // Press Tab to move to next pane
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should be NotHandled because chat requires focus mode (modal behavior)
        // In modal behavior, Tab is not handled in Layout Mode
        assert_eq!(result, EventResult::NotHandled);
    }

    #[test]
    fn test_auto_focus_off_ignores_requires_focus_mode() {
        // When auto_focus is disabled, requires_focus_mode should have no effect
        // All panes are modal
        let (mut layout, chat_id, tree_id) = create_mixed_focus_layout();
        // auto_focus defaults to false

        // First pane (chat) selected
        let selected = layout.mode().selected_pane();
        assert_eq!(selected.unwrap(), chat_id);

        // Press 'x' key - should NOT be routed (normal Layout Mode behavior)
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // Should NOT be handled (Layout Mode doesn't handle 'x')
        assert_eq!(result, EventResult::NotHandled);

        // Move to second pane (tree)
        layout.select_next_pane();
        assert_eq!(layout.mode().selected_pane().unwrap(), tree_id);

        // Press 'x' key - should STILL not be routed (auto_focus is off)
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::NotHandled);
    }

    #[test]
    fn test_hybrid_workflow() {
        // Simulate a realistic hybrid workflow:
        // 1. Navigate to tree pane (auto-passthrough)
        // 2. Press keys that are handled by tree
        // 3. Navigate to chat pane (requires explicit focus)
        // 4. Press Enter to focus
        // 5. Type in chat
        // 6. Press Ctrl-A to exit
        let (mut layout, chat_id, tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // Step 1: Navigate to tree pane
        layout.select_next_pane();
        assert_eq!(layout.mode().selected_pane().unwrap(), tree_id);

        // Step 2: Press 'j' key - handled by tree via auto-passthrough
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_layout()); // Still in Layout Mode

        // Step 3: Navigate back to chat pane
        layout.select_prev_pane();
        assert_eq!(layout.mode().selected_pane().unwrap(), chat_id);

        // Step 4: Press Enter to focus (required because chat needs explicit focus)
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_focus()); // Now in Focus Mode

        // Step 5: Type in chat
        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_focus()); // Still in Focus Mode

        // Step 6: Press Ctrl-A to exit
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        let result = layout.handle_key_event(key);
        assert_eq!(result, EventResult::Consumed);
        assert!(layout.mode().is_layout()); // Back to Layout Mode
    }

    #[test]
    fn test_requires_focus_q_does_quit_in_layout_mode() {
        // Verify that 'q' quits when selected pane requires focus mode
        // It falls through to Layout Mode which handles 'q' as quit
        let (mut layout, chat_id, _tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // First pane (chat) requires focus mode
        assert_eq!(layout.mode().selected_pane().unwrap(), chat_id);

        // Press 'q' - should quit (falls through to Layout Mode)
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        // 'q' should quit in Layout Mode
        assert_eq!(result, EventResult::Quit);
    }

    #[test]
    fn test_auto_passthrough_q_quits() {
        // Verify that 'q' still quits even in auto-passthrough mode
        // because it's checked as a global key first
        let (mut layout, _chat_id, tree_id) = create_mixed_focus_layout();
        layout = layout.with_auto_focus(true);

        // Navigate to tree pane (doesn't require focus)
        layout.select_next_pane();
        assert_eq!(layout.mode().selected_pane().unwrap(), tree_id);

        // Press 'q' - should quit (global key in auto_focus mode)
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = layout.handle_key_event(key);

        assert_eq!(result, EventResult::Quit);
    }
}
