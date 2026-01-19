//! Fetch git diff from current working directory.

use std::process::Command;

/// Get git diff from the current working directory.
///
/// Tries in order:
/// 1. Unstaged changes (`git diff`)
/// 2. Staged changes (`git diff --cached`)
/// 3. Last commit diff (`git diff HEAD~1`)
///
/// Returns empty string if not in a git repository or no changes found.
///
/// # Example
///
/// ```rust,no_run
/// use ratatui_toolkit::code_diff::helpers::get_git_diff;
///
/// let diff = get_git_diff();
/// if !diff.is_empty() {
///     println!("Found changes:\n{}", diff);
/// }
/// ```
pub fn get_git_diff() -> String {
    // Try unstaged changes first
    if let Ok(output) = Command::new("git").args(["diff"]).output() {
        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        if !diff.trim().is_empty() {
            return diff;
        }
    }

    // Try staged changes
    if let Ok(output) = Command::new("git").args(["diff", "--cached"]).output() {
        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        if !diff.trim().is_empty() {
            return diff;
        }
    }

    // Fallback to last commit diff
    if let Ok(output) = Command::new("git").args(["diff", "HEAD~1"]).output() {
        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        if !diff.trim().is_empty() {
            return diff;
        }
    }

    // If all else fails, return empty
    String::new()
}
