//! Checkbox state for task lists.

/// Checkbox state for task lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    /// Unchecked: [ ]
    Unchecked,
    /// Checked: [x] or [X]
    Checked,
    /// Todo/In Progress: [-]
    Todo,
}
