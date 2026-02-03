mod constructors;
mod methods;

use crossterm::event::KeyCode;

/// Configurable keybindings for tree navigation
#[derive(Debug, Clone)]
pub struct TreeKeyBindings {
    pub next: Vec<KeyCode>,
    pub previous: Vec<KeyCode>,
    pub expand: Vec<KeyCode>,
    pub collapse: Vec<KeyCode>,
    pub toggle: Vec<KeyCode>,
    pub goto_top: Vec<KeyCode>,
    pub goto_bottom: Vec<KeyCode>,
}
