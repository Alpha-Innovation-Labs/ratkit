//! Demo vim-like mode for statusline demonstration.

/// Vim-like mode states for statusline demo.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum DemoMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}
