//! Demo tab enum for showcase navigation.

mod methods;

/// Available demo tabs in the showcase.
#[derive(Clone, Copy, PartialEq)]
pub enum DemoTab {
    Markdown,
    CodeDiff,
    Tree,
    Dialogs,
    StatusLine,
    Terminal,
}
