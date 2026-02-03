# ratkit-file-watcher

File watching service for detecting file system changes in ratkit.

## Features

- Watch single files or entire directory trees
- Configurable debounce intervals
- Non-blocking interface suitable for TUI event loops
- Support for recursive and non-recursive watching

## Usage

```rust
use ratkit_file_watcher::{FileWatcher, WatchMode};
use std::path::Path;

// Watch a single file
let mut watcher = FileWatcher::for_file().unwrap();
watcher.watch(Path::new("README.md")).unwrap();

// In your event loop:
if watcher.check_for_changes() {
    println!("File changed!");
    let paths = watcher.get_changed_paths();
    for path in paths {
        println!("  - {}", path.display());
    }
}

// Watch a directory recursively
let mut dir_watcher = FileWatcher::for_directory().unwrap();
dir_watcher.watch(Path::new("./src")).unwrap();
```

## Dependencies

- `notify` v6 - File system watching
