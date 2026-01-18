//! Enum types for markdown elements.
//!
//! Contains all enumeration types used in markdown rendering.

pub mod checkbox_state;
pub mod code_block_border_kind;
pub mod column_alignment;
pub mod element_kind;
pub mod table_border_kind;
pub mod text_segment;

pub use checkbox_state::CheckboxState;
pub use code_block_border_kind::CodeBlockBorderKind;
pub use column_alignment::ColumnAlignment;
pub use element_kind::ElementKind;
pub use table_border_kind::TableBorderKind;
pub use text_segment::TextSegment;
