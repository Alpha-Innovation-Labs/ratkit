//! Parse YAML frontmatter from markdown content.

/// Parse YAML frontmatter from the beginning of content.
///
/// # Arguments
///
/// * `content` - The markdown content that may contain frontmatter
///
/// # Returns
///
/// A tuple containing:
/// - `Option<Vec<(String, String)>>` - The parsed frontmatter fields as key-value pairs
/// - `&str` - The remaining content after frontmatter
/// - `usize` - The line count of the frontmatter (includes opening and closing `---` lines)
pub fn parse_frontmatter(content: &str) -> (Option<Vec<(String, String)>>, &str, usize) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content, 0);
    }

    // Find the closing ---
    let after_opening = &trimmed[3..];
    if let Some(end_pos) = after_opening.find("\n---") {
        let frontmatter_text = &after_opening[..end_pos];
        let remaining = &after_opening[end_pos + 4..]; // Skip past "\n---"

        // Count lines: 1 for opening ---, lines in frontmatter_text, 1 for closing ---
        let frontmatter_lines = frontmatter_text.lines().count();
        let total_lines = 1 + frontmatter_lines + 1; // opening + content + closing

        // Parse the frontmatter fields
        let mut fields = Vec::new();
        for line in frontmatter_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                // Remove surrounding quotes from value if present
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value[1..value.len() - 1].to_string()
                } else {
                    value
                };
                fields.push((key, value));
            }
        }

        if !fields.is_empty() {
            return (Some(fields), remaining, total_lines);
        }
    }

    (None, content, 0)
}
