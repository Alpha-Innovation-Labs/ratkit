//! State persistence for the viewer

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// Persistence manager for viewer state
#[derive(Debug, Clone)]
pub struct Persistence {
    config_dir: PathBuf,
    state_file: PathBuf,
}

impl Default for Persistence {
    fn default() -> Self {
        Self::new()
    }
}

impl Persistence {
    /// Create a new persistence manager
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("markdown-viewer");

        let state_file = config_dir.join("state.json");

        Self {
            config_dir,
            state_file,
        }
    }

    /// Create with custom config directory
    pub fn with_config_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.config_dir = dir.as_ref().to_path_buf();
        self.state_file = self.config_dir.join("state.json");
        self
    }

    /// Ensure config directory exists
    fn ensure_config_dir(&self) -> Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir).context("Failed to create config directory")?;
        }
        Ok(())
    }

    /// Save state to file
    pub fn save_state(&self, state: serde_json::Value) -> Result<()> {
        self.ensure_config_dir()?;

        let json = serde_json::to_string_pretty(&state).context("Failed to serialize state")?;

        fs::write(&self.state_file, json).context("Failed to write state file")?;

        Ok(())
    }

    /// Load state from file
    pub fn load_state(&self) -> Result<serde_json::Value> {
        if !self.state_file.exists() {
            return Ok(serde_json::json!({}));
        }

        let json = fs::read_to_string(&self.state_file).context("Failed to read state file")?;

        let state: serde_json::Value =
            serde_json::from_str(&json).context("Failed to parse state file")?;

        Ok(state)
    }

    /// Clear saved state
    pub fn clear_state(&self) -> Result<()> {
        if self.state_file.exists() {
            fs::remove_file(&self.state_file).context("Failed to remove state file")?;
        }
        Ok(())
    }

    /// Get state file path
    pub fn state_file(&self) -> &Path {
        &self.state_file
    }
}
