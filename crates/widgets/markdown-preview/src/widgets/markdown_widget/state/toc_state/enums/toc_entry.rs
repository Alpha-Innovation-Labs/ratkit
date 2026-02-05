//! Table of Contents entry.

/// A single entry in the Table of Contents.
#[derive(Debug, Clone, Default)]
pub struct TocEntry {
    /// The text content of the heading.
    pub text: String,
    /// The heading level (1-6).
    pub level: u8,
    /// The line number in the source document.
    pub line_number: usize,
    /// A unique identifier for the section.
    pub section_id: String,
}
