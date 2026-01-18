//! Constructors for FileWatcher.

mod for_directory;
mod for_file;
mod new;
mod with_config;

pub use for_directory::*;
pub use for_file::*;
pub use new::*;
pub use with_config::*;
