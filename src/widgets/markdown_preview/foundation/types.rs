//! Types module.

#[derive(Debug, Clone, Default)]
pub struct GitStats {
    pub additions: usize,
    pub deletions: usize,
    pub changed_files: usize,
}

#[derive(Debug, Clone)]
pub struct SelectionPos {
    pub start: (usize, usize),
    pub end: (usize, usize),
}
