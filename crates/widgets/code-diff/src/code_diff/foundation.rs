//! Foundation types for the code diff widget.

use ratatui::style::Color;

pub mod diff_config {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct DiffConfig {
        pub show_line_numbers: bool,
        pub added_fg: Color,
        pub added_bg: Color,
        pub removed_fg: Color,
        pub removed_bg: Color,
        pub context_fg: Color,
        pub context_bg: Color,
        pub hunk_header_fg: Color,
        pub hunk_header_bg: Color,
        pub line_number_fg: Color,
        pub gutter_width: usize,
        pub context_lines: usize,
    }

    impl DiffConfig {
        pub fn new() -> Self {
            Self {
                show_line_numbers: true,
                added_fg: Color::Black,
                added_bg: Color::LightGreen,
                removed_fg: Color::White,
                removed_bg: Color::LightRed,
                context_fg: Color::Gray,
                context_bg: Color::Reset,
                hunk_header_fg: Color::DarkGray,
                hunk_header_bg: Color::Reset,
                line_number_fg: Color::DarkGray,
                gutter_width: 4,
                context_lines: 3,
            }
        }

        pub fn with_show_line_numbers(mut self, show: bool) -> Self {
            self.show_line_numbers = show;
            self
        }
    }
}

pub mod diff_line {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DiffLineKind {
        Context,
        Added,
        Removed,
        HunkHeader,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DiffLine {
        pub kind: DiffLineKind,
        pub content: String,
        pub old_line_num: Option<usize>,
        pub new_line_num: Option<usize>,
    }

    impl DiffLine {
        pub fn context(content: &str, old_num: usize, new_num: usize) -> Self {
            Self {
                kind: DiffLineKind::Context,
                content: content.to_string(),
                old_line_num: Some(old_num),
                new_line_num: Some(new_num),
            }
        }

        pub fn added(content: &str, new_num: usize) -> Self {
            Self {
                kind: DiffLineKind::Added,
                content: content.to_string(),
                old_line_num: None,
                new_line_num: Some(new_num),
            }
        }

        pub fn removed(content: &str, old_num: usize) -> Self {
            Self {
                kind: DiffLineKind::Removed,
                content: content.to_string(),
                old_line_num: Some(old_num),
                new_line_num: None,
            }
        }

        pub fn hunk_header(content: &str) -> Self {
            Self {
                kind: DiffLineKind::HunkHeader,
                content: content.to_string(),
                old_line_num: None,
                new_line_num: None,
            }
        }

        pub fn from_diff_line(line: &str) -> Option<Self> {
            match line.chars().next() {
                Some('+') if !line.starts_with("+++") => Some(Self::added(line, 0)),
                Some('-') if !line.starts_with("---") => Some(Self::removed(line, 0)),
                Some('@') => Some(Self::hunk_header(line)),
                Some(' ') | Some('\t') | Some('\\') => Some(Self::context(line, 0, 0)),
                _ => None,
            }
        }

        pub fn is_context(&self) -> bool {
            self.kind == DiffLineKind::Context
        }

        pub fn is_added(&self) -> bool {
            self.kind == DiffLineKind::Added
        }

        pub fn is_removed(&self) -> bool {
            self.kind == DiffLineKind::Removed
        }

        pub fn is_hunk_header(&self) -> bool {
            self.kind == DiffLineKind::HunkHeader
        }

        pub fn prefix(&self) -> &'static str {
            match self.kind {
                DiffLineKind::Context => " ",
                DiffLineKind::Added => "+",
                DiffLineKind::Removed => "-",
                DiffLineKind::HunkHeader => "@",
            }
        }
    }
}

pub mod diff_hunk {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct DiffHunk {
        pub old_start: usize,
        pub old_count: usize,
        pub new_start: usize,
        pub new_count: usize,
        pub lines: Vec<diff_line::DiffLine>,
        pub header: String,
    }

    impl DiffHunk {
        pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
            Self {
                old_start,
                old_count,
                new_start,
                new_count,
                lines: Vec::new(),
                header: format!(
                    "@@ -{},{} +{},{} @@",
                    old_start, old_count, new_start, new_count
                ),
            }
        }

        pub fn from_header(header: &str) -> Self {
            let header = header.to_string();

            let parse_numbers = |s: &str| {
                let parts: Vec<&str> = s.split(',').collect();
                (
                    parts.get(0).and_then(|p| p.parse().ok()).unwrap_or(1),
                    parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0),
                )
            };

            let before = header.find("-").map(|i| &header[i + 1..]).unwrap_or("");
            let after = header.find("+").map(|i| &header[i + 1..]).unwrap_or("");

            let (old_start, old_count) = parse_numbers(before);
            let (new_start, new_count) = parse_numbers(after);

            let mut hunk = Self::new(old_start, old_count, new_start, new_count);
            hunk.header = header;
            hunk
        }

        pub fn add_line(&mut self, line: diff_line::DiffLine) {
            self.lines.push(line);
        }

        pub fn lines(&self) -> &[diff_line::DiffLine] {
            &self.lines
        }

        pub fn added_count(&self) -> usize {
            self.lines.iter().filter(|l| l.is_added()).count()
        }

        pub fn removed_count(&self) -> usize {
            self.lines.iter().filter(|l| l.is_removed()).count()
        }

        pub fn total_lines(&self) -> usize {
            self.lines.len()
        }
    }
}

pub mod enums {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum DiffStyle {
        #[default]
        SideBySide,
        Unified,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum DiffLineKind {
        Context,
        Added,
        Removed,
        #[default]
        HunkHeader,
    }
}

pub mod helpers {
    use super::*;

    pub fn get_git_diff(_old: &str, _new: &str) -> String {
        String::new()
    }
}
