use ratatui::style::Color;

pub(super) fn get_custom_icon(filename: &str) -> Option<(char, Color)> {
    let lower = filename.to_lowercase();

    if lower.ends_with(".just") || lower == "justfile" || lower == ".justfile" {
        return Some(('\u{e779}', Color::Rgb(194, 160, 92)));
    }

    if lower == "makefile" || lower.starts_with("makefile.") || lower == "gnumakefile" {
        return Some(('\u{e779}', Color::Rgb(109, 128, 134)));
    }

    if lower == "gemfile" || lower == "gemfile.lock" {
        return Some(('\u{e21e}', Color::Rgb(112, 21, 22)));
    }

    if lower == ".env" || lower.starts_with(".env.") {
        return Some(('\u{f462}', Color::Rgb(251, 192, 45)));
    }

    if lower == "license"
        || lower == "license.txt"
        || lower == "license.md"
        || lower == "licence"
        || lower == "licence.txt"
        || lower == "copying"
    {
        return Some(('\u{f48a}', Color::Rgb(216, 187, 98)));
    }

    if lower == "jenkinsfile" || lower.starts_with("jenkinsfile.") {
        return Some(('\u{e767}', Color::Rgb(217, 69, 57)));
    }

    if lower == ".ds_store" {
        return Some(('\u{f179}', Color::Rgb(126, 142, 168)));
    }

    None
}
