use crate::Hotkey;
use crate::HotkeyScope;

/// Trait for widgets that declare their hotkeys.
///
/// Implement this trait to provide a standardized way for widgets
/// to declare what hotkeys they support and in what scope they are active.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::services::hotkey::{Hotkey, HotkeyScope, HasHotkeys};
///
/// struct MyWidget;
///
/// impl HasHotkeys for MyWidget {
///     fn hotkeys(&self) -> Vec<Hotkey> {
///         vec![
///             Hotkey::new("j", "Move down").scope(HotkeyScope::Global),
///             Hotkey::new("k", "Move up").scope(HotkeyScope::Global),
///         ]
///     }
/// }
/// ```
pub trait HasHotkeys {
    /// Get the hotkeys declared by this widget.
    ///
    /// # Returns
    ///
    /// A vector of hotkeys that this widget responds to.
    fn hotkeys(&self) -> Vec<Hotkey>;

    /// Get hotkeys filtered by scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope to filter by
    ///
    /// # Returns
    ///
    /// Hotkeys that are active in the given scope.
    fn hotkeys_for_scope(&self, scope: &HotkeyScope) -> Vec<Hotkey> {
        self.hotkeys()
            .into_iter()
            .filter(|h| match scope {
                HotkeyScope::Global => matches!(h.scope, HotkeyScope::Global),
                HotkeyScope::Modal(m) => {
                    matches!(&h.scope, HotkeyScope::Modal(s) if *s == *m)
                }
                HotkeyScope::Tab(t) => matches!(&h.scope, HotkeyScope::Tab(s) if *s == *t),
                HotkeyScope::Custom(c) => {
                    matches!(&h.scope, HotkeyScope::Custom(s) if *s == *c)
                }
            })
            .collect()
    }
}
