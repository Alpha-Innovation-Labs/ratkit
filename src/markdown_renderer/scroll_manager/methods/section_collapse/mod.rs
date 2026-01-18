//! Section collapse/expand methods for MarkdownScrollManager.

mod clear_section_hierarchy;
mod collapse_all_sections;
mod collapse_section;
mod expand_all_sections;
mod expand_section;
mod is_section_collapsed;
mod register_section;
mod set_section_collapsed;
mod toggle_section_collapse;

pub use clear_section_hierarchy::*;
pub use collapse_all_sections::*;
pub use collapse_section::*;
pub use expand_all_sections::*;
pub use expand_section::*;
pub use is_section_collapsed::*;
pub use register_section::*;
pub use set_section_collapsed::*;
pub use toggle_section_collapse::*;
