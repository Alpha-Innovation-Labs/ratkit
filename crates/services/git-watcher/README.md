# ratkit-git-watcher

Git watching service for detecting git repository changes in ratkit.

## Features

- Monitors `.git` directory for changes
- Configurable debounce intervals
- Non-blocking interface suitable for TUI event loops
- Useful for caching git statistics and only recomputing when state changes

## Usage

```rust
use ratkit_git_watcher::GitWatcher;
use std::path::Path;

let mut watcher = GitWatcher::new().unwrap();
watcher.watch(Path::new("/path/to/repo")).unwrap();

// In your event loop:
if watcher.check_for_changes() {
    println!("Git state changed, recompute stats!");
}
```

## Dependencies

- `notify` v6 - File system watching
