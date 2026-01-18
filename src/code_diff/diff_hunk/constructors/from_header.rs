use crate::code_diff::diff_hunk::DiffHunk;

impl DiffHunk {
    /// Creates a new diff hunk by parsing a unified diff header line.
    ///
    /// Parses headers in the format: `@@ -old_start,old_count +new_start,new_count @@ context`
    ///
    /// # Arguments
    ///
    /// * `header` - The hunk header line to parse
    ///
    /// # Returns
    ///
    /// `Some(DiffHunk)` if parsing succeeds, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::DiffHunk;
    ///
    /// let hunk = DiffHunk::from_header("@@ -1,4 +1,5 @@ fn main()").unwrap();
    /// assert_eq!(hunk.old_start, 1);
    /// assert_eq!(hunk.old_count, 4);
    /// assert_eq!(hunk.new_start, 1);
    /// assert_eq!(hunk.new_count, 5);
    /// assert_eq!(hunk.context.as_deref(), Some("fn main()"));
    /// ```
    pub fn from_header(header: &str) -> Option<Self> {
        parse_hunk_header(header)
    }
}

/// Parses a unified diff hunk header line.
///
/// # Arguments
///
/// * `header` - The header line to parse
///
/// # Returns
///
/// `Some(DiffHunk)` if parsing succeeds, `None` otherwise
fn parse_hunk_header(header: &str) -> Option<DiffHunk> {
    let header = header.trim();

    // Must start with @@
    if !header.starts_with("@@") {
        return None;
    }

    // Find the closing @@
    let rest = header.strip_prefix("@@")?.trim_start();
    let end_marker = rest.find("@@")?;

    let range_part = &rest[..end_marker].trim();
    let context_part = rest[end_marker + 2..].trim();

    // Parse -old_start,old_count +new_start,new_count
    let parts: Vec<&str> = range_part.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let (old_start, old_count) = parse_range(parts[0].strip_prefix('-')?)?;
    let (new_start, new_count) = parse_range(parts[1].strip_prefix('+')?)?;

    let context = if context_part.is_empty() {
        None
    } else {
        Some(context_part.to_string())
    };

    Some(DiffHunk {
        old_start,
        old_count,
        new_start,
        new_count,
        context,
        lines: Vec::new(),
    })
}

/// Parses a range specification like "1,4" or "1" into (start, count).
///
/// # Arguments
///
/// * `range` - The range string to parse
///
/// # Returns
///
/// `Some((start, count))` if parsing succeeds, `None` otherwise
fn parse_range(range: &str) -> Option<(usize, usize)> {
    if let Some((start, count)) = range.split_once(',') {
        Some((start.parse().ok()?, count.parse().ok()?))
    } else {
        // Single line change: "1" means start=1, count=1
        let start: usize = range.parse().ok()?;
        Some((start, 1))
    }
}
