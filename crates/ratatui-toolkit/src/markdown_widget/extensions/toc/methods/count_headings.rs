//! Static method to count headings in markdown content.

use super::super::Toc;

impl<'a> Toc<'a> {
    /// Count the number of headings in markdown content.
    ///
    /// This is a static method useful for calculating dynamic TOC height
    /// without constructing a full Toc widget.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    ///
    /// # Returns
    ///
    /// The number of headings found.
    pub fn count_headings(content: &str) -> usize {
        let mut count = 0;
        let mut in_code_block = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Track code blocks
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            if in_code_block {
                continue;
            }

            // Check for headings
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count();
                if level <= 6 {
                    let text = trimmed[level..].trim();
                    if !text.is_empty() {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
