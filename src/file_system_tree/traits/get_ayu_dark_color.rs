use ratatui::style::Color;

pub(super) fn get_ayu_dark_color(filename: &str) -> Color {
    let lower = filename.to_lowercase();

    if lower.ends_with(".sh")
        || lower.ends_with(".bash")
        || lower.ends_with(".zsh")
        || lower.ends_with(".fish")
        || lower.ends_with(".py")
        || lower.ends_with(".rb")
    {
        return Color::Rgb(126, 147, 80);
    }

    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".svg")
        || lower.ends_with(".ico")
        || lower.ends_with(".webp")
        || lower.ends_with(".bmp")
    {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".mp3")
        || lower.ends_with(".mp4")
        || lower.ends_with(".wav")
        || lower.ends_with(".avi")
        || lower.ends_with(".mkv")
        || lower.ends_with(".flac")
        || lower.ends_with(".ogg")
        || lower.ends_with(".webm")
    {
        return Color::Rgb(126, 147, 80);
    }

    if lower.ends_with(".zip")
        || lower.ends_with(".tar")
        || lower.ends_with(".gz")
        || lower.ends_with(".bz2")
        || lower.ends_with(".xz")
        || lower.ends_with(".7z")
        || lower.ends_with(".rar")
    {
        return Color::Rgb(168, 83, 97);
    }

    if lower.ends_with(".pdf")
        || lower.ends_with(".doc")
        || lower.ends_with(".docx")
        || lower.ends_with(".rtf")
        || lower.ends_with(".odt")
    {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".json")
        || lower.ends_with(".js")
        || lower.ends_with(".ts")
        || lower.ends_with(".jsx")
        || lower.ends_with(".tsx")
    {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".yml") || lower.ends_with(".yaml") {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".toml") {
        return Color::Rgb(148, 100, 182);
    }

    if lower.ends_with(".rs") {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".c")
        || lower.ends_with(".cpp")
        || lower.ends_with(".h")
        || lower.ends_with(".hpp")
    {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".go") {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".md") || lower.ends_with(".txt") || lower.ends_with(".log") {
        return Color::Rgb(230, 225, 207);
    }

    Color::Rgb(230, 225, 207)
}
