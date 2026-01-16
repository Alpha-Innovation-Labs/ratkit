//! Fuzzy finder component
//!
//! Provides a PTY-based fuzzy finder widget that spawns interactive fuzzy search tools.

pub mod constructors;
pub mod functions;
pub mod methods;
pub mod traits;

use crate::termtui::Parser;
use portable_pty::{Child, MasterPty};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

/// Terminal state for fuzzy finder
pub struct FuzzyFinderTerminal {
    parser: Arc<Mutex<Parser>>,
    _master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    child: Arc<Mutex<Box<dyn Child + Send + Sync>>>,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

/// A generic fuzzy finder widget that spawns a PTY-based fuzzy finder (like fzf)
#[allow(dead_code)]
pub struct FuzzyFinder {
    /// PTY terminal state
    terminal: Option<FuzzyFinderTerminal>,
    /// Popup size (percentage of screen)
    size_percent: (u16, u16),
    /// Title for the popup
    title: String,
    /// Loading message (before terminal spawns)
    loading_message: String,
}
