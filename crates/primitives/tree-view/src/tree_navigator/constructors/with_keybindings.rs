//! TreeNavigator::with_keybindings constructor.

use crate::keybindings::TreeKeyBindings;
use crate::tree_navigator::TreeNavigator;

impl TreeNavigator {
    /// Creates a tree navigator with custom keybindings.
    ///
    /// # Arguments
    ///
    /// * `keybindings` - The custom keybindings to use.
    ///
    /// # Returns
    ///
    /// A new `TreeNavigator` with the specified keybindings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeKeyBindings, TreeNavigator};
    ///
    /// let keybindings = TreeKeyBindings::default();
    /// let navigator = TreeNavigator::with_keybindings(keybindings);
    /// ```
    pub fn with_keybindings(keybindings: TreeKeyBindings) -> Self {
        Self { keybindings }
    }
}
