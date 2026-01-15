//! Interaction mode state machine for Layout vs Focus modes

use super::PaneId;

/// Interaction mode - either Layout (navigation) or Focus (interaction)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionMode {
    /// Layout Mode (Command Mode)
    /// - Navigate panes with hjkl
    /// - Select pane (visual highlight only)
    /// - Press Enter to focus selected pane
    Layout { selected_pane: Option<PaneId> },

    /// Focus Mode (Insert Mode)
    /// - All input goes to focused pane
    /// - Press Ctrl-A to exit to Layout Mode
    Focus { focused_pane: PaneId },
}

impl InteractionMode {
    /// Create new Layout Mode with no selection
    pub fn layout() -> Self {
        Self::Layout {
            selected_pane: None,
        }
    }

    /// Create new Layout Mode with pane selected
    pub fn layout_with_selection(pane: PaneId) -> Self {
        Self::Layout {
            selected_pane: Some(pane),
        }
    }

    /// Create new Focus Mode with pane focused
    pub fn focus(pane: PaneId) -> Self {
        Self::Focus { focused_pane: pane }
    }

    /// Check if in Layout Mode
    pub fn is_layout(&self) -> bool {
        matches!(self, Self::Layout { .. })
    }

    /// Check if in Focus Mode
    pub fn is_focus(&self) -> bool {
        matches!(self, Self::Focus { .. })
    }

    /// Get selected pane if in Layout Mode
    pub fn selected_pane(&self) -> Option<PaneId> {
        match self {
            Self::Layout { selected_pane } => *selected_pane,
            _ => None,
        }
    }

    /// Get focused pane if in Focus Mode
    pub fn focused_pane(&self) -> Option<PaneId> {
        match self {
            Self::Focus { focused_pane } => Some(*focused_pane),
            _ => None,
        }
    }

    /// Transition from Layout to Focus mode
    pub fn enter_focus(&mut self, pane: PaneId) {
        *self = Self::Focus { focused_pane: pane };
    }

    /// Transition from Focus to Layout mode, selecting the previously focused pane
    pub fn exit_focus(&mut self) {
        if let Self::Focus { focused_pane } = self {
            *self = Self::Layout {
                selected_pane: Some(*focused_pane),
            };
        }
    }

    /// Update selected pane in Layout Mode
    pub fn select_pane(&mut self, pane: PaneId) {
        if let Self::Layout { selected_pane } = self {
            *selected_pane = Some(pane);
        }
    }
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::layout()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_layout_mode() {
        let mode = InteractionMode::default();
        assert!(mode.is_layout());
        assert_eq!(mode.selected_pane(), None);
    }

    #[test]
    fn test_layout_mode_creation() {
        let mode = InteractionMode::layout();
        assert!(mode.is_layout());
        assert_eq!(mode.selected_pane(), None);
    }

    #[test]
    fn test_layout_mode_with_selection() {
        let pane_id = PaneId::new("test");
        let mode = InteractionMode::layout_with_selection(pane_id);
        assert!(mode.is_layout());
        assert_eq!(mode.selected_pane(), Some(pane_id));
    }

    #[test]
    fn test_focus_mode_creation() {
        let pane_id = PaneId::new("test");
        let mode = InteractionMode::focus(pane_id);
        assert!(mode.is_focus());
        assert_eq!(mode.focused_pane(), Some(pane_id));
    }

    #[test]
    fn test_enter_focus_transition() {
        let mut mode = InteractionMode::layout();
        let pane_id = PaneId::new("test");

        mode.enter_focus(pane_id);

        assert!(mode.is_focus());
        assert_eq!(mode.focused_pane(), Some(pane_id));
    }

    #[test]
    fn test_exit_focus_transition() {
        let pane_id = PaneId::new("test");
        let mut mode = InteractionMode::focus(pane_id);

        mode.exit_focus();

        assert!(mode.is_layout());
        assert_eq!(mode.selected_pane(), Some(pane_id));
    }

    #[test]
    fn test_select_pane_in_layout_mode() {
        let mut mode = InteractionMode::layout();
        let pane_id = PaneId::new("test");

        mode.select_pane(pane_id);

        assert_eq!(mode.selected_pane(), Some(pane_id));
    }

    #[test]
    fn test_select_pane_in_focus_mode_does_nothing() {
        let pane1 = PaneId::new("pane1");
        let pane2 = PaneId::new("pane2");
        let mut mode = InteractionMode::focus(pane1);

        mode.select_pane(pane2);

        // Should still be in focus mode with pane1
        assert!(mode.is_focus());
        assert_eq!(mode.focused_pane(), Some(pane1));
    }
}
