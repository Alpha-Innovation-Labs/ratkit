# ratkit-repo-watcher

Repository watcher for detecting git and working tree changes in ratkit.

## Features

- Combines git watcher (`.git` state) with file watcher (working tree edits)
- Provides cached list of modified files via `git status --porcelain`
- Configurable inclusion of untracked files
- Non-blocking interface suitable for TUI event loops

## Usage

```rust
use ratkit_repo_watcher::RepoWatcher;
use std::path::Path;

let mut watcher = RepoWatcher::new().unwrap();
watcher.watch(Path::new("/path/to/repo")).unwrap();

// In your event loop:
if watcher.check_for_changes() {
    let changes = watcher.get_change_set();
    println!("Modified files: {}", changes.modified.len());
    for path in changes.all_paths() {
        println!("Changed: {}", path.display());
    }
}
```

## Dependencies

- `notify` v6 - File system watching
- `ratkit-file-watcher` - File watching service
- `ratkit-git-watcher` - Git watching service
