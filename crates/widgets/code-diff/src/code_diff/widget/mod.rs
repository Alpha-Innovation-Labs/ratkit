//! Code diff widget for displaying side-by-side diffs.
//!
//! The main widget that renders diff hunks in a side-by-side or unified view,
//! similar to VS Code's diff viewer.

use std::collections::HashMap;

use super::foundation::diff_config::DiffConfig;
use super::foundation::diff_hunk::DiffHunk;
use super::foundation::diff_line::DiffLine;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Debug, Clone, Default)]
pub struct CodeDiff {
    pub file_path: Option<String>,
    pub hunks: Vec<DiffHunk>,
    pub config: DiffConfig,
    pub scroll_offset: usize,
    pub file_diffs: HashMap<String, Vec<DiffHunk>>,
}

impl CodeDiff {
    pub fn new() -> Self {
        Self {
            file_path: None,
            hunks: Vec::new(),
            config: DiffConfig::new(),
            scroll_offset: 0,
            file_diffs: HashMap::new(),
        }
    }

    pub fn with_file_path(mut self, path: &str) -> Self {
        self.file_path = Some(path.to_string());
        self
    }

    pub fn with_config(mut self, config: DiffConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        self.hunks.push(hunk);
    }

    pub fn hunks(&self) -> &[DiffHunk] {
        &self.hunks
    }

    pub fn hunks_mut(&mut self) -> &mut [DiffHunk] {
        &mut self.hunks
    }

    pub fn from_unified_diff(diff: &str) -> Self {
        let mut result = Self::new();
        result.parse_unified_diff(diff);
        result
    }

    fn parse_unified_diff(&mut self, diff: &str) {
        let mut current_hunk: Option<DiffHunk> = None;

        for line in diff.lines() {
            if line.starts_with("@@") {
                if let Some(hunk) = current_hunk {
                    self.hunks.push(hunk);
                }
                current_hunk = Some(DiffHunk::from_header(line));
            } else if let Some(ref mut hunk) = current_hunk {
                if let Some(parsed_line) = DiffLine::from_diff_line(line) {
                    hunk.add_line(parsed_line);
                }
            }
        }

        if let Some(hunk) = current_hunk {
            self.hunks.push(hunk);
        }
    }
}

impl Widget for CodeDiff {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let content = if let Some(path) = &self.file_path {
            format!("Diff: {}", path)
        } else {
            "Diff: (no file)".to_string()
        };

        let widget = ratatui::widgets::Paragraph::new(content);
        widget.render(area, buf);
    }
}
