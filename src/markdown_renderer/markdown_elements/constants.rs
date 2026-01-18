//! Constants for markdown rendering styling.
//!
//! Contains icons, markers, and color constants used in markdown rendering.

use ratatui::style::Color;

/// Code block color theme
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeBlockTheme {
    /// Ayu Dark theme (default) - warm orange/amber accents
    #[default]
    AyuDark,
    /// GitHub Dark theme
    GitHubDark,
    /// Dracula theme - purple/pink tones
    Dracula,
    /// Nord theme - cool blue tones
    Nord,
    /// Monokai theme - warm vibrant colors
    Monokai,
    /// One Dark (Atom) theme
    OneDark,
    /// Gruvbox Dark theme
    Gruvbox,
    /// Tokyo Night theme
    TokyoNight,
    /// Catppuccin Mocha theme
    Catppuccin,
}

/// Colors for a code block theme
#[derive(Debug, Clone, Copy)]
pub struct CodeBlockColors {
    /// Border color
    pub border: Color,
    /// Background color
    pub background: Color,
    /// Header background
    pub header_bg: Color,
    /// Header text color
    pub header_text: Color,
    /// Icon/language color
    pub icon: Color,
    /// Line number color
    pub line_number: Color,
    /// Line number separator color
    pub line_separator: Color,
}

impl CodeBlockTheme {
    /// Get the colors for this theme
    pub fn colors(&self) -> CodeBlockColors {
        match self {
            CodeBlockTheme::AyuDark => CodeBlockColors {
                border: Color::Rgb(57, 63, 84),
                background: Color::Rgb(10, 14, 20),
                header_bg: Color::Rgb(15, 20, 28),
                header_text: Color::Rgb(179, 186, 197),
                icon: Color::Rgb(255, 180, 84),      // Ayu orange
                line_number: Color::Rgb(70, 80, 100),
                line_separator: Color::Rgb(45, 52, 70),
            },
            CodeBlockTheme::GitHubDark => CodeBlockColors {
                border: Color::Rgb(70, 70, 70),
                background: Color::Rgb(30, 30, 30),
                header_bg: Color::Rgb(40, 40, 40),
                header_text: Color::Rgb(180, 180, 180),
                icon: Color::Rgb(255, 200, 100),
                line_number: Color::Rgb(90, 90, 90),
                line_separator: Color::Rgb(60, 60, 60),
            },
            CodeBlockTheme::Dracula => CodeBlockColors {
                border: Color::Rgb(98, 114, 164),
                background: Color::Rgb(40, 42, 54),
                header_bg: Color::Rgb(68, 71, 90),
                header_text: Color::Rgb(248, 248, 242),
                icon: Color::Rgb(255, 121, 198), // Pink
                line_number: Color::Rgb(98, 114, 164),
                line_separator: Color::Rgb(68, 71, 90),
            },
            CodeBlockTheme::Nord => CodeBlockColors {
                border: Color::Rgb(76, 86, 106),
                background: Color::Rgb(46, 52, 64),
                header_bg: Color::Rgb(59, 66, 82),
                header_text: Color::Rgb(216, 222, 233),
                icon: Color::Rgb(136, 192, 208), // Frost blue
                line_number: Color::Rgb(76, 86, 106),
                line_separator: Color::Rgb(59, 66, 82),
            },
            CodeBlockTheme::Monokai => CodeBlockColors {
                border: Color::Rgb(117, 113, 94),
                background: Color::Rgb(39, 40, 34),
                header_bg: Color::Rgb(54, 55, 49),
                header_text: Color::Rgb(248, 248, 242),
                icon: Color::Rgb(230, 219, 116), // Yellow
                line_number: Color::Rgb(117, 113, 94),
                line_separator: Color::Rgb(73, 72, 62),
            },
            CodeBlockTheme::OneDark => CodeBlockColors {
                border: Color::Rgb(76, 82, 99),
                background: Color::Rgb(40, 44, 52),
                header_bg: Color::Rgb(50, 56, 66),
                header_text: Color::Rgb(171, 178, 191),
                icon: Color::Rgb(229, 192, 123), // Gold
                line_number: Color::Rgb(76, 82, 99),
                line_separator: Color::Rgb(58, 64, 76),
            },
            CodeBlockTheme::Gruvbox => CodeBlockColors {
                border: Color::Rgb(146, 131, 116),
                background: Color::Rgb(40, 40, 40),
                header_bg: Color::Rgb(60, 56, 54),
                header_text: Color::Rgb(235, 219, 178),
                icon: Color::Rgb(250, 189, 47), // Yellow
                line_number: Color::Rgb(146, 131, 116),
                line_separator: Color::Rgb(80, 73, 69),
            },
            CodeBlockTheme::TokyoNight => CodeBlockColors {
                border: Color::Rgb(61, 89, 161),
                background: Color::Rgb(26, 27, 38),
                header_bg: Color::Rgb(36, 40, 59),
                header_text: Color::Rgb(169, 177, 214),
                icon: Color::Rgb(125, 207, 255), // Cyan
                line_number: Color::Rgb(61, 89, 161),
                line_separator: Color::Rgb(41, 46, 66),
            },
            CodeBlockTheme::Catppuccin => CodeBlockColors {
                border: Color::Rgb(127, 132, 156),
                background: Color::Rgb(30, 30, 46),
                header_bg: Color::Rgb(49, 50, 68),
                header_text: Color::Rgb(205, 214, 244),
                icon: Color::Rgb(249, 226, 175), // Yellow
                line_number: Color::Rgb(127, 132, 156),
                line_separator: Color::Rgb(69, 71, 90),
            },
        }
    }

    /// Get all available themes
    pub fn all() -> &'static [CodeBlockTheme] {
        &[
            CodeBlockTheme::AyuDark,
            CodeBlockTheme::GitHubDark,
            CodeBlockTheme::Dracula,
            CodeBlockTheme::Nord,
            CodeBlockTheme::Monokai,
            CodeBlockTheme::OneDark,
            CodeBlockTheme::Gruvbox,
            CodeBlockTheme::TokyoNight,
            CodeBlockTheme::Catppuccin,
        ]
    }

    /// Get the theme name
    pub fn name(&self) -> &'static str {
        match self {
            CodeBlockTheme::AyuDark => "Ayu Dark",
            CodeBlockTheme::GitHubDark => "GitHub Dark",
            CodeBlockTheme::Dracula => "Dracula",
            CodeBlockTheme::Nord => "Nord",
            CodeBlockTheme::Monokai => "Monokai",
            CodeBlockTheme::OneDark => "One Dark",
            CodeBlockTheme::Gruvbox => "Gruvbox",
            CodeBlockTheme::TokyoNight => "Tokyo Night",
            CodeBlockTheme::Catppuccin => "Catppuccin",
        }
    }
}

/// Heading icons by level (matching render-markdown.nvim).
pub const HEADING_ICONS: [&str; 6] = [
    "󰲡 ", // H1
    "󰲣 ", // H2
    "󰲥 ", // H3
    "󰲧 ", // H4
    "󰲩 ", // H5
    "󰲫 ", // H6
];

/// Bullet markers that cycle by nesting level (matching render-markdown.nvim).
pub const BULLET_MARKERS: [&str; 4] = ["● ", "○ ", "◆ ", "◇ "];

/// Checkbox icons (matching render-markdown.nvim).
pub const CHECKBOX_UNCHECKED: &str = "󰄱 "; // [ ]
pub const CHECKBOX_CHECKED: &str = "󰱒 ";   // [x]
pub const CHECKBOX_TODO: &str = "󰥔 ";      // [-]

/// Blockquote marker (matching render-markdown.nvim).
pub const BLOCKQUOTE_MARKER: &str = "▋";

/// Horizontal rule character.
pub const HORIZONTAL_RULE_CHAR: char = '─';

/// Link icons (matching render-markdown.nvim).
pub const LINK_ICON: &str = "󰌹 ";       // Default hyperlink
pub const IMAGE_ICON: &str = "󰥶 ";      // Image
pub const EMAIL_ICON: &str = "󰀓 ";      // Email

/// Domain-specific link icons.
pub fn get_link_icon(url: &str) -> &'static str {
    let url_lower = url.to_lowercase();
    if url_lower.contains("github.com") {
        "󰊤 "
    } else if url_lower.contains("gitlab.com") {
        "󰮠 "
    } else if url_lower.contains("discord.com") || url_lower.contains("discord.gg") {
        "󰙯 "
    } else if url_lower.contains("linkedin.com") {
        "󰌻 "
    } else if url_lower.contains("reddit.com") {
        "󰑍 "
    } else if url_lower.contains("slack.com") {
        "󰒱 "
    } else if url_lower.contains("stackoverflow.com") {
        "󰓌 "
    } else if url_lower.contains("twitter.com") || url_lower.contains("x.com") {
        " "
    } else if url_lower.contains("wikipedia.org") {
        "󰖬 "
    } else if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") {
        "󰗃 "
    } else if url_lower.starts_with("mailto:") {
        EMAIL_ICON
    } else if url_lower.ends_with(".png")
        || url_lower.ends_with(".jpg")
        || url_lower.ends_with(".jpeg")
        || url_lower.ends_with(".gif")
        || url_lower.ends_with(".svg")
        || url_lower.ends_with(".webp")
    {
        IMAGE_ICON
    } else {
        LINK_ICON
    }
}

/// Language icons for code blocks (Nerd Font icons).
pub fn get_language_icon(lang: &str) -> &'static str {
    match lang.to_lowercase().as_str() {
        // Systems Programming
        "rust" | "rs" => "\u{e7a8} ",           //
        "c" => "\u{e61e} ",                     //
        "cpp" | "c++" | "cxx" | "cc" => "\u{e61d} ", //
        "zig" => "\u{e6a9} ",                   //
        "nim" => "\u{e677} ",                   //
        "d" => "\u{e7af} ",                     //
        "ada" => "󱗆 ",
        "fortran" | "f90" | "f95" => "󱈚 ",
        "assembly" | "asm" | "nasm" => "\u{e6ab} ", //

        // Web Frontend
        "javascript" | "js" | "mjs" | "cjs" => "\u{e74e} ", //
        "typescript" | "ts" | "mts" | "cts" => "\u{e628} ", //
        "jsx" | "tsx" | "react" => "\u{e7ba} ", //
        "html" | "htm" => "\u{e736} ",          //
        "css" => "\u{e749} ",                   //
        "scss" | "sass" => "\u{e74b} ",         //
        "less" => "\u{e758} ",                  //
        "svelte" => "\u{e697} ",                //
        "vue" => "\u{e6a0} ",                   //
        "astro" => "\u{e6b6} ",                 //

        // Web Backend / Scripting
        "python" | "py" | "pyw" | "pyi" => "󰌠 ",
        "ruby" | "rb" | "erb" => "\u{e739} ",   //
        "php" => "\u{e73d} ",                   //
        "perl" | "pl" | "pm" => "\u{e769} ",    //
        "lua" => "\u{e620} ",                   //
        "r" | "rmd" => "\u{e68a} ",             //
        "julia" => "\u{e624} ",                 //

        // JVM Languages
        "java" => "\u{e738} ",                  //
        "kotlin" | "kt" | "kts" => "\u{e634} ", //
        "scala" => "\u{e737} ",                 //
        "groovy" => "\u{e775} ",                //
        "clojure" | "clj" | "cljs" | "cljc" => "\u{e76a} ", //

        // .NET Languages
        "csharp" | "cs" | "c#" => "󰌛 ",
        "fsharp" | "fs" | "f#" => "\u{e7a7} ",  //
        "vb" | "vbnet" | "visualbasic" => "󰛤 ",

        // Functional Languages
        "haskell" | "hs" => "\u{e777} ",        //
        "elixir" | "ex" | "exs" => "\u{e62d} ", //
        "erlang" | "erl" => "\u{e7b1} ",        //
        "ocaml" | "ml" => "\u{e67a} ",          //
        "elm" => "\u{e62c} ",                   //
        "purescript" | "purs" => "\u{e630} ",   //
        "racket" | "rkt" => "󰘧 ",
        "scheme" | "scm" => "󰘧 ",
        "lisp" | "cl" | "el" => "󰅲 ",

        // Mobile
        "swift" => "\u{e755} ",                 //
        "objectivec" | "objc" | "m" => "\u{e61e} ", //
        "dart" => "\u{e798} ",                  //

        // Go & Modern
        "go" | "golang" => "\u{e626} ",         //
        "v" | "vlang" => "\u{e6ac} ",           //
        "crystal" => "\u{e7a3} ",               //
        "odin" => "󰮔 ",

        // Shell & Scripting
        "bash" | "sh" | "shell" | "zsh" | "fish" | "ksh" | "csh" => "\u{e795} ", //
        "powershell" | "ps1" | "psm1" => "󰨊 ",
        "batch" | "bat" | "cmd" => "\u{e629} ", //
        "nushell" | "nu" => "\u{e795} ",        //

        // Data & Config
        "json" | "jsonc" | "json5" => "\u{e60b} ", //
        "yaml" | "yml" => "\u{e6a8} ",          //
        "toml" => "\u{e6b2} ",                  //
        "xml" | "xsl" | "xslt" => "󰗀 ",
        "ini" | "conf" | "cfg" => "\u{e615} ",  //
        "env" | "dotenv" => "\u{e615} ",        //
        "csv" => "\u{e64a} ",                   //

        // Markup & Docs
        "markdown" | "md" | "mdx" => "\u{e73e} ", //
        "latex" | "tex" => "\u{e69b} ",         //
        "rst" | "restructuredtext" => "\u{e6a5} ", //
        "asciidoc" | "adoc" => "\u{e6a5} ",     //
        "org" => "\u{e633} ",                   //

        // Database
        "sql" | "mysql" | "postgresql" | "postgres" | "sqlite" => "\u{e706} ", //
        "plsql" | "plpgsql" => "\u{e706} ",     //
        "mongodb" | "mongo" => "\u{e7a4} ",     //
        "redis" => "\u{e76d} ",                 //
        "graphql" | "gql" => "\u{e662} ",       //
        "prisma" => "\u{e684} ",                //

        // DevOps & Infrastructure
        "docker" | "dockerfile" | "containerfile" => "\u{e7b0} ", //
        "kubernetes" | "k8s" => "󱃾 ",
        "terraform" | "tf" | "hcl" => "󱁢 ",
        "ansible" => "\u{e7b0} ",               //
        "vagrant" => "\u{e7b0} ",               //
        "nix" => "\u{e779} ",                   //
        "nginx" => "\u{e776} ",                 //
        "apache" => "\u{e769} ",                //

        // Build & Package
        "makefile" | "make" | "mk" => "\u{e779} ", //
        "cmake" => "\u{e615} ",                 //
        "gradle" => "\u{e660} ",                //
        "maven" | "pom" => "\u{e674} ",         //
        "cargo" => "\u{e7a8} ",                 //  (Rust)
        "npm" | "package.json" => "\u{e71e} ",  //
        "yarn" => "\u{e6a7} ",                  //
        "pnpm" => "\u{e71e} ",                  //
        "pip" | "requirements" => "󰌠 ",

        // Version Control
        "git" | "gitignore" | "gitconfig" | "gitattributes" => "\u{e702} ", //
        "diff" | "patch" => "\u{e728} ",        //

        // Editor & IDE
        "vim" | "vimrc" | "neovim" | "nvim" => "\u{e62b} ", //
        "emacs" | "elisp" => "\u{e632} ",       //
        "vscode" | "code" => "󰨞 ",

        // Other Languages
        "solidity" | "sol" => "\u{e6ac} ",      //
        "move" => "󰆧 ",
        "cairo" => "󰜥 ",
        "wasm" | "wat" | "webassembly" => "\u{e6a1} ", //
        "llvm" | "ir" => "\u{e61e} ",           //
        "cuda" | "cu" => "\u{e61e} ",           //
        "opencl" => "\u{e61e} ",                //
        "glsl" | "hlsl" | "shader" => "\u{e6ad} ", //
        "proto" | "protobuf" => "\u{e6ae} ",    //
        "thrift" => "\u{e6ae} ",                //
        "avro" => "\u{e6ae} ",                  //
        "capnp" => "\u{e6ae} ",                 //
        "flatbuffers" | "fbs" => "\u{e6ae} ",   //

        // Misc
        "regex" | "regexp" => "\u{e656} ",      //
        "http" | "rest" => "\u{e60c} ",         //
        "binary" | "hex" => "\u{e7a3} ",        //
        "log" | "logs" => "\u{e714} ",          //
        "text" | "txt" | "plain" => "\u{e612} ", //

        // Default
        _ => "\u{e612} ",                       //
    }
}

/// Background colors for heading levels.
pub fn heading_bg_color(level: u8) -> Color {
    match level {
        1 => Color::Rgb(80, 40, 80), // Purple-ish
        2 => Color::Rgb(40, 60, 80), // Blue-ish
        3 => Color::Rgb(40, 80, 60), // Green-ish
        4 => Color::Rgb(80, 60, 40), // Orange-ish
        5 => Color::Rgb(60, 60, 60), // Gray
        6 => Color::Rgb(50, 50, 50), // Darker gray
        _ => Color::Rgb(50, 50, 50),
    }
}

/// Foreground colors for heading levels.
pub fn heading_fg_color(level: u8) -> Color {
    match level {
        1 => Color::Rgb(255, 180, 255), // Bright magenta
        2 => Color::Rgb(130, 180, 255), // Bright blue
        3 => Color::Rgb(130, 255, 180), // Bright cyan
        4 => Color::Rgb(255, 200, 130), // Bright orange
        5 => Color::Rgb(200, 200, 200), // Light gray
        6 => Color::Rgb(170, 170, 170), // Gray
        _ => Color::White,
    }
}
