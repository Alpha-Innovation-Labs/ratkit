use crate::Hotkey;
use crate::HotkeyScope;
use crossterm::event::KeyCode;

/// Trait for handling hotkey actions.
///
/// Widgets that implement this trait can respond to hotkey actions
/// when they are triggered. This provides a clean separation between
/// hotkey declaration (via `HasHotkeys`) and hotkey handling.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyScope, HotkeyHandler};
/// use crossterm::event::KeyCode;
///
/// struct MyWidget;
///
/// impl HotkeyHandler for MyWidget {
///     fn handle_hotkey(&mut self, key: &KeyCode, scope: &HotkeyScope) -> bool {
///         match key {
///             KeyCode::Char('j') => {
///                 // Handle down action
///                 true
///             }
///             KeyCode::Char('k') => {
///                 // Handle up action
///                 true
///             }
///             _ => false,
///         }
///     }
/// }
/// ```
pub trait HotkeyHandler {
    /// Handle a hotkey action.
    ///
    /// # Arguments
    ///
    /// * `key` - The key code that was pressed
    /// * `scope` - The current active scope
    ///
    /// # Returns
    ///
    /// `true` if the hotkey was handled, `false` otherwise.
    fn handle_hotkey(&mut self, key: &KeyCode, scope: &HotkeyScope) -> bool;

    /// Check if this handler can handle the given hotkey.
    ///
    /// # Arguments
    ///
    /// * `hotkey` - The hotkey to check
    /// * `key` - The key code that was pressed
    /// * `scope` - The current active scope
    ///
    /// # Returns
    ///
    /// `true` if this handler can handle the hotkey.
    fn can_handle(&self, hotkey: &Hotkey, key: &KeyCode, scope: &HotkeyScope) -> bool {
        let hotkey_key = hotkey.key.to_lowercase();
        let matches_key = match key {
            KeyCode::Char(c) => hotkey_key == c.to_string().to_lowercase(),
            KeyCode::Tab => hotkey_key == "tab",
            KeyCode::Enter => hotkey_key == "enter",
            KeyCode::Esc => hotkey_key == "escape" || hotkey_key == "esc",
            KeyCode::Up => hotkey_key == "up",
            KeyCode::Down => hotkey_key == "down",
            KeyCode::Left => hotkey_key == "left",
            KeyCode::Right => hotkey_key == "right",
            KeyCode::Backspace => hotkey_key == "backspace",
            _ => false,
        };

        if !matches_key {
            return false;
        }

        match &hotkey.scope {
            HotkeyScope::Global => true,
            HotkeyScope::Modal(m) => matches!(scope, HotkeyScope::Modal(s) if s == m),
            HotkeyScope::Tab(t) => matches!(scope, HotkeyScope::Tab(s) if s == t),
            HotkeyScope::Custom(c) => matches!(scope, HotkeyScope::Custom(s) if s == c),
        }
    }
}
