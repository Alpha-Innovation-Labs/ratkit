use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::{Color, Modifier, Style};
use std::fs;
use std::path::Path;

/// State for text input with multi-line support and special prefix parsing.
#[derive(Debug, Clone)]
pub struct InputState {
    /// Current input text
    text: String,
    /// Cursor position in text
    cursor: usize,
    /// Lines of text (split by newlines)
    lines: Vec<String>,
    /// Current line being edited
    current_line: usize,
    /// Whether @ prefix is active (file attachment mode)
    is_file_mode: bool,
    /// File search query
    file_query: String,
    /// Available files for fuzzy search
    available_files: Vec<String>,
    /// Selected file index in search results
    selected_file_index: usize,
    /// Whether / prefix is active (command mode)
    is_command_mode: bool,
    /// Command being entered
    command: String,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            lines: vec![String::new()],
            current_line: 0,
            is_file_mode: false,
            file_query: String::new(),
            available_files: Vec::new(),
            selected_file_index: 0,
            is_command_mode: false,
            command: String::new(),
        }
    }
}

impl InputState {
    /// Create a new input state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get current input text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Check if in file attachment mode.
    pub fn is_file_mode(&self) -> bool {
        self.is_file_mode
    }

    /// Check if in command mode.
    pub fn is_command_mode(&self) -> bool {
        self.is_command_mode
    }

    /// Get current file search query.
    pub fn file_query(&self) -> &str {
        &self.file_query
    }

    /// Get filtered files matching query.
    pub fn filtered_files(&self) -> Vec<String> {
        let query_lower = self.file_query.to_lowercase();
        self.available_files
            .iter()
            .filter(|f| f.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    /// Get selected file index.
    pub fn selected_file_index(&self) -> usize {
        self.selected_file_index
    }

    /// Get current command.
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Set available files for fuzzy search.
    pub fn set_available_files(&mut self, files: Vec<String>) {
        self.available_files = files;
    }

    /// Load files from current working directory.
    ///
    /// Filters out common ignore patterns:
    /// - .git
    /// - node_modules
    /// - target
    /// - __pycache__
    /// - .venv
    /// - venv
    pub fn load_files_from_cwd(&mut self) {
        let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        let ignore_patterns = [
            ".git",
            "node_modules",
            "target",
            "__pycache__",
            ".venv",
            "venv",
            "dist",
            "build",
            ".DS_Store",
        ];

        self.available_files = fs::read_dir(&cwd)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| {
                !ignore_patterns
                    .iter()
                    .any(|pattern| name.eq_ignore_ascii_case(pattern) || name.starts_with(pattern))
            })
            .collect();
    }

    /// Handle a key event.
    ///
    /// Returns:
    /// - `Some(text)` if Enter was pressed (submit message or command)
    /// - `Some(file)` if a file was selected
    /// - `None` otherwise
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        match key.code {
            KeyCode::Char('@') => {
                if !self.is_file_mode && !self.is_command_mode {
                    self.is_file_mode = true;
                }
                None
            }
            KeyCode::Char('/') => {
                if !self.is_file_mode && !self.is_command_mode {
                    self.is_command_mode = true;
                }
                None
            }
            KeyCode::Char(c) => {
                let is_ctrl_j = key.modifiers.contains(KeyModifiers::CONTROL) && c == 'j';

                if is_ctrl_j {
                    self.insert_newline();
                } else if self.is_file_mode {
                    self.file_query.push(c);
                    self.selected_file_index = 0;
                } else if self.is_command_mode {
                    self.command.push(c);
                } else {
                    self.insert_char(c);
                }
                None
            }
            KeyCode::Backspace => {
                if self.is_file_mode {
                    if !self.file_query.is_empty() {
                        self.file_query.pop();
                        if self.file_query.is_empty() {
                            self.is_file_mode = false;
                        }
                    }
                } else if self.is_command_mode {
                    self.command.pop();
                    if self.command.is_empty() {
                        self.is_command_mode = false;
                    }
                } else {
                    self.backspace();
                }
                None
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if !self.is_file_mode && !self.is_command_mode && self.cursor > 0 {
                    self.cursor -= 1;
                }
                None
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.is_file_mode && !self.is_command_mode && self.cursor < self.text.len() {
                    self.cursor += 1;
                }
                None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.is_file_mode {
                    let filtered = self.filtered_files();
                    if !filtered.is_empty() {
                        self.selected_file_index = if self.selected_file_index == 0 {
                            filtered.len() - 1
                        } else {
                            self.selected_file_index - 1
                        };
                    }
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.is_file_mode {
                    let filtered = self.filtered_files();
                    if !filtered.is_empty() {
                        self.selected_file_index = (self.selected_file_index + 1) % filtered.len();
                    }
                }
                None
            }
            KeyCode::Enter => {
                if self.is_file_mode {
                    let filtered = self.filtered_files();
                    if let Some(file) = filtered.get(self.selected_file_index) {
                        let file = file.clone();
                        self.is_file_mode = false;
                        self.file_query.clear();
                        self.selected_file_index = 0;
                        Some(format!("@{}", file))
                    } else {
                        None
                    }
                } else if self.is_command_mode {
                    let command = self.command.clone();
                    self.is_command_mode = false;
                    self.command.clear();
                    Some(format!("/{}", command))
                } else {
                    let text = self.text.clone();
                    self.clear();
                    Some(text)
                }
            }
            KeyCode::Esc => {
                if self.is_file_mode {
                    self.is_file_mode = false;
                    self.file_query.clear();
                    self.selected_file_index = 0;
                }
                if self.is_command_mode {
                    self.is_command_mode = false;
                    self.command.clear();
                }
                None
            }
            _ => None,
        }
    }

    /// Insert a character at cursor position.
    fn insert_char(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor += 1;
        self.update_lines();
    }

    /// Insert a newline.
    fn insert_newline(&mut self) {
        self.text.insert(self.cursor, '\n');
        self.cursor += 1;
        self.update_lines();
    }

    /// Delete character before cursor.
    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
            self.update_lines();
        }
    }

    /// Clear input.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.lines = vec![String::new()];
        self.current_line = 0;
    }

    /// Update lines based on text.
    fn update_lines(&mut self) {
        self.lines = self.text.split('\n').map(|s| s.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
    }

    /// Update cursor position from current line.
    fn update_cursor_from_lines(&mut self) {
        let mut pos = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if i == self.current_line {
                self.cursor = pos + line.len();
                return;
            }
            pos += line.len() + 1;
        }
    }
}
