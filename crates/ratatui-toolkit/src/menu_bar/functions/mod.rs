pub fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let code = c as u32;
            if (0xF000..=0xF8FF).contains(&code) {
                2
            } else {
                unicode_width::UnicodeWidthChar::width(c).unwrap_or(1)
            }
        })
        .sum()
}
