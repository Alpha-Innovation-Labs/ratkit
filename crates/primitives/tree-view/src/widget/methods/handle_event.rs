//! Handle keyboard and mouse events for TreeView.

use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::layout::Rect;

use crate::tree_navigator::TreeNavigator;
use crate::tree_view_state::TreeViewState;
use crate::widget::TreeView;
use ratkit_widget_event::WidgetEvent;

impl<'a, T> TreeView<'a, T> {
    /// Handle a keyboard event and return a WidgetEvent.
    ///
    /// This method processes keyboard input for the tree view, including:
    /// - Navigation keys (up/down/left/right)
    /// - Expansion/collapse keys (h/l)
    /// - Filter mode keys (/ to enter, Esc to exit, Enter to confirm)
    ///
    /// When filter mode is active, all keystrokes are routed to the filter input
    /// and the tree is automatically filtered.
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to handle
    /// * `navigator` - The tree navigator for navigation operations
    /// * `state` - The tree view state to modify
    ///
    /// # Returns
    ///
    /// A `WidgetEvent` indicating what action was taken.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView, TreeNavigator, TreeViewState};
    /// use crossterm::event::KeyCode;
    ///
    /// let nodes = vec![TreeNode::new("Root")];
    /// let mut tree = TreeView::new(nodes)
    ///     .render_fn(|data, state| {
    ///         ratatui::text::Line::from(*data)
    ///     });
    /// let mut state = TreeViewState::new();
    /// let navigator = TreeNavigator::new();
    ///
    /// match tree.handle_key_event(crossterm::event::KeyEvent::new(KeyCode::Down), &navigator, &mut state) {
    ///     WidgetEvent::Selected { path } => println!("Selected: {:?}", path),
    ///     WidgetEvent::Scrolled { offset, direction } => println!("Scrolled"),
    ///     _ => {}
    /// }
    /// ```
    pub fn handle_key_event(
        &mut self,
        key: KeyEvent,
        navigator: &TreeNavigator,
        state: &mut TreeViewState,
    ) -> WidgetEvent {
        if state.filter_mode {
            self.handle_filter_key(key, state)
        } else {
            navigator.handle_key_event(key, &self.nodes, state);
            WidgetEvent::Selected {
                path: state.selected_path.clone().unwrap_or_default(),
            }
        }
    }

    fn handle_filter_key(&mut self, key: KeyEvent, state: &mut TreeViewState) -> WidgetEvent {
        match key.code {
            KeyCode::Esc => {
                state.clear_filter();
                WidgetEvent::FilterModeExited {
                    path: state.selected_path.clone().unwrap_or_default(),
                }
            }
            KeyCode::Enter => {
                let path = state.selected_path.clone();
                state.exit_filter_mode();
                WidgetEvent::FilterModeExited {
                    path: path.unwrap_or_default(),
                }
            }
            KeyCode::Backspace => {
                state.backspace_filter();
                WidgetEvent::FilterModeChanged {
                    active: true,
                    filter: state.filter.clone().unwrap_or_default(),
                }
            }
            KeyCode::Char(c) => {
                state.append_to_filter(c);
                WidgetEvent::FilterModeChanged {
                    active: true,
                    filter: state.filter.clone().unwrap_or_default(),
                }
            }
            _ => WidgetEvent::None,
        }
    }

    /// Handle a mouse event and return a WidgetEvent.
    ///
    /// This method processes mouse input for the tree view, including:
    /// - Click to select items
    /// - Scroll wheel to scroll
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event to handle
    /// * `state` - The tree view state to modify
    /// * `area` - The area occupied by the tree view (including borders)
    ///
    /// # Returns
    ///
    /// A `WidgetEvent` indicating what action was taken.
    pub fn handle_mouse_event(
        &mut self,
        event: MouseEvent,
        state: &mut TreeViewState,
        area: Rect,
    ) -> WidgetEvent {
        let inner_area = match self.block {
            Some(ref block) => block.inner(area),
            None => area,
        };

        if inner_area.height == 0 {
            return WidgetEvent::None;
        }

        let y = event.row;
        if y < inner_area.y || y >= inner_area.y + inner_area.height {
            return WidgetEvent::None;
        }

        let row = (y - inner_area.y + state.offset as u16) as usize;
        let items = self.flatten_tree(state);

        if let Some((_, path)) = items.get(row) {
            state.selected_path = Some(path.clone());
            return WidgetEvent::Selected { path: path.clone() };
        }

        WidgetEvent::None
    }
}
