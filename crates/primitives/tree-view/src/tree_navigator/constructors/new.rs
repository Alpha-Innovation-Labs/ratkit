//! TreeNavigator::new constructor.

use crate::keybindings::TreeKeyBindings;
use crate::tree_navigator::TreeNavigator;

impl TreeNavigator {
    /// Creates a new tree navigator with default keybindings.
    ///
    /// # Returns
    ///
    /// A new `TreeNavigator` with default keybindings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeNavigator;
    ///
    /// let navigator = TreeNavigator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            keybindings: TreeKeyBindings::default(),
        }
    }
}
