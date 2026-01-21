//! Get icon for file based on extension.

/// Returns a nerd font icon for the given filename based on extension.
pub fn file_icon(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("");

    match ext.to_lowercase().as_str() {
        // Rust
        "rs" => "\u{e7a8}", //  (rust icon)

        // Web
        "js" => "\u{e74e}",            //  (javascript)
        "ts" => "\u{e628}",            //  (typescript)
        "jsx" | "tsx" => "\u{e7ba}",   //  (react)
        "html" => "\u{e736}",          //  (html)
        "css" => "\u{e749}",           //  (css)
        "scss" | "sass" => "\u{e74b}", //  (sass)
        "vue" => "\u{e6a0}",           //  (vue)

        // Config
        "toml" => "\u{e6b2}",         //  (config)
        "yaml" | "yml" => "\u{e6a8}", //  (yaml)
        "json" => "\u{e60b}",         //  (json)
        "xml" => "\u{e619}",          //  (xml)

        // Docs
        "md" | "markdown" => "\u{e73e}", //  (markdown)
        "txt" => "\u{f15c}",             //  (text file)
        "pdf" => "\u{f1c1}",             //  (pdf)

        // Shell
        "sh" | "bash" | "zsh" => "\u{e795}", //  (terminal)
        "fish" => "\u{f489}",                //  (fish)

        // Python
        "py" => "\u{e73c}", //  (python)

        // Go
        "go" => "\u{e627}", //  (go)

        // C/C++
        "c" => "\u{e61e}",                  //  (c)
        "cpp" | "cc" | "cxx" => "\u{e61d}", //  (c++)
        "h" | "hpp" => "\u{e61f}",          //  (header)

        // Java/Kotlin
        "java" => "\u{e738}",       //  (java)
        "kt" | "kts" => "\u{e634}", //  (kotlin)

        // Ruby
        "rb" => "\u{e739}", //  (ruby)

        // PHP
        "php" => "\u{e73d}", //  (php)

        // Lua
        "lua" => "\u{e620}", //  (lua)

        // Docker
        "dockerfile" => "\u{f308}", //  (docker)

        // Git
        "gitignore" | "gitattributes" => "\u{f1d3}", //  (git)

        // Lock files
        "lock" => "\u{f023}", //  (lock)

        // Images
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => "\u{f1c5}", //  (image)

        // Default
        _ => "\u{f15b}", //  (generic file)
    }
}
