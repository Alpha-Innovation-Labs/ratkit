//! Kind of table border.

#[derive(Debug, Clone)]
pub enum TableBorderKind {
    Top(Vec<usize>),
    HeaderSeparator(Vec<usize>),
    Bottom(Vec<usize>),
}
