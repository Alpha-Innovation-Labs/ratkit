//! Configurable key bindings for MasterLayout

mod constructors;
mod methods;

use crossterm::event::{KeyCode, KeyModifiers};

/// Configurable key bindings for MasterLayout
///
/// This struct allows customization of all keyboard shortcuts used by MasterLayout.
/// Each field represents a specific action and contains a tuple of (KeyCode, KeyModifiers).
///
/// # Example
///
/// ```
/// use ratatui_toolkit::master_layout::MasterLayoutKeyBindings;
/// use crossterm::event::{KeyCode, KeyModifiers};
///
/// let mut bindings = MasterLayoutKeyBindings::default();
/// // Change quit key from 'q' to 'x'
/// bindings.quit = vec![(KeyCode::Char('x'), KeyModifiers::empty())];
/// ```
#[derive(Debug, Clone)]
pub struct MasterLayoutKeyBindings {
    /// Keys to quit the application (default: q, Q)
    pub quit: Vec<(KeyCode, KeyModifiers)>,
    /// Key to clear selection in Layout Mode (default: Esc)
    pub clear_selection: (KeyCode, KeyModifiers),
    /// Key to deselect pane in Layout Mode (default: Ctrl+A)
    pub deselect_pane: (KeyCode, KeyModifiers),
    /// Keys to switch tabs (default: 1-9)
    pub switch_tabs: Vec<(KeyCode, KeyModifiers)>,
    /// Key to navigate left (default: h)
    pub navigate_left: (KeyCode, KeyModifiers),
    /// Key to navigate right (default: l)
    pub navigate_right: (KeyCode, KeyModifiers),
    /// Key to navigate up (default: k)
    pub navigate_up: (KeyCode, KeyModifiers),
    /// Key to navigate down (default: j)
    pub navigate_down: (KeyCode, KeyModifiers),
    /// Key to focus the selected pane (default: Enter)
    pub focus_pane: (KeyCode, KeyModifiers),
    /// Key to exit focus mode (default: Ctrl+A)
    pub exit_focus_mode: (KeyCode, KeyModifiers),
    /// Key to copy selection (default: Ctrl+Shift+C)
    pub copy_selection: (KeyCode, KeyModifiers),
}
