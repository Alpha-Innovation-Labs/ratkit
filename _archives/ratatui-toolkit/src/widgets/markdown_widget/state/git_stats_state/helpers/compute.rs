//! Compute git diff stats for a file.

use std::path::Path;
use std::process::Command;

/// Compute git diff stats (additions, modified_files, deletions) for a specific file.
///
/// # Arguments
///
/// * `file_path` - Optional path to the file. If None, gets stats for entire repo.
///
/// # Returns
///
/// A tuple of (additions, modified_file_count, deletions).
pub fn compute_git_stats(file_path: Option<&Path>) -> (usize, usize, usize) {
    let args = match file_path {
        Some(path) => vec![
            "diff",
            "--numstat",
            "HEAD",
            "--",
            path.to_str().unwrap_or(""),
        ],
        None => vec!["diff", "--numstat", "HEAD"],
    };

    let output = Command::new("git").args(&args).output().ok();

    if let Some(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let (mut adds, mut dels, mut modified) = (0usize, 0usize, 0usize);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let line_adds = parts[0].parse::<usize>().unwrap_or(0);
                    let line_dels = parts[1].parse::<usize>().unwrap_or(0);
                    adds += line_adds;
                    dels += line_dels;
                    // Count as modified if file has changes
                    if line_adds > 0 || line_dels > 0 {
                        modified += 1;
                    }
                }
            }
            return (adds, modified, dels);
        }
    }
    (0, 0, 0)
}
