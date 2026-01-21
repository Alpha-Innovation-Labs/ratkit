//! Collect git status changes for a repository.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::services::repo_watcher::{GitChangeSet, GitFileStatus};

/// Collect git changes using `git status --porcelain`.
pub fn collect_git_changes(repo_path: &Path, include_untracked: bool) -> GitChangeSet {
    let repo_str = repo_path.to_str().unwrap_or(".");
    let mut command = Command::new("git");
    command.args(["-C", repo_str, "status", "--porcelain=v1", "-z"]);
    if !include_untracked {
        command.arg("-uno");
    }

    let output = command.output().ok();
    if let Some(output) = output {
        if output.status.success() {
            return parse_porcelain_status(&output.stdout, include_untracked);
        }
    }

    GitChangeSet::default()
}

fn parse_porcelain_status(output: &[u8], include_untracked: bool) -> GitChangeSet {
    let mut changes = GitChangeSet::default();
    let mut entries = output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty());

    while let Some(entry) = entries.next() {
        if entry.len() < 3 {
            continue;
        }

        let status_x = entry[0];
        let status_y = entry[1];

        let status = match_status(status_x, status_y);
        let Some(status) = status else {
            continue;
        };

        if status == GitFileStatus::Untracked && !include_untracked {
            continue;
        }

        let path_bytes = if is_rename_like(status_x, status_y) {
            entries.next().unwrap_or(&entry[3..])
        } else {
            &entry[3..]
        };

        if path_bytes.is_empty() {
            continue;
        }

        let path = PathBuf::from(String::from_utf8_lossy(path_bytes).to_string());
        push_path(&mut changes, status, path);
    }

    changes
}

fn match_status(status_x: u8, status_y: u8) -> Option<GitFileStatus> {
    if status_x == b'!' || status_y == b'!' {
        return None;
    }

    if status_x == b'?' && status_y == b'?' {
        return Some(GitFileStatus::Untracked);
    }

    if status_x == b'R' || status_y == b'R' {
        return Some(GitFileStatus::Renamed);
    }

    if status_x == b'A' || status_y == b'A' || status_x == b'C' || status_y == b'C' {
        return Some(GitFileStatus::Added);
    }

    if status_x == b'D' || status_y == b'D' {
        return Some(GitFileStatus::Deleted);
    }

    if status_x == b'M'
        || status_y == b'M'
        || status_x == b'U'
        || status_y == b'U'
        || status_x == b'T'
        || status_y == b'T'
    {
        return Some(GitFileStatus::Modified);
    }

    None
}

fn is_rename_like(status_x: u8, status_y: u8) -> bool {
    status_x == b'R' || status_y == b'R' || status_x == b'C' || status_y == b'C'
}

fn push_path(changes: &mut GitChangeSet, status: GitFileStatus, path: PathBuf) {
    match status {
        GitFileStatus::Added => changes.added.push(path),
        GitFileStatus::Modified => changes.modified.push(path),
        GitFileStatus::Deleted => changes.deleted.push(path),
        GitFileStatus::Renamed => changes.renamed.push(path),
        GitFileStatus::Untracked => changes.untracked.push(path),
    }
}
