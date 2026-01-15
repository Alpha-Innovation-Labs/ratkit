use ratatui::style::Color;

/// Configuration for markdown rendering styles
#[derive(Clone)]
pub struct MarkdownStyle {
    pub h1_icon: &'static str,
    pub h2_icon: &'static str,
    pub h3_icon: &'static str,
    pub h4_icon: &'static str,
    pub h5_icon: &'static str,
    pub h6_icon: &'static str,

    pub h1_fg: Color,
    pub h1_bg: Color,
    pub h2_fg: Color,
    pub h2_bg: Color,
    pub h3_fg: Color,
    pub h3_bg: Color,

    pub bullet_l1: &'static str,
    pub bullet_l2: &'static str,
    pub bullet_l3: &'static str,

    pub code_block_border: bool,
    pub code_block_bg: Color,
    pub inline_code_bg: Color,
    pub inline_code_fg: Color,

    pub quote_icon: &'static str,
    pub quote_fg: Color,
    pub quote_bg: Color,

    pub callout_note_icon: &'static str,
    pub callout_tip_icon: &'static str,
    pub callout_warning_icon: &'static str,
    pub callout_caution_icon: &'static str,
}

impl Default for MarkdownStyle {
    fn default() -> Self {
        Self {
            // Heading icons (matching render-markdown.nvim defaults)
            h1_icon: "󰲡 ",
            h2_icon: "󰲣 ",
            h3_icon: "󰲥 ",
            h4_icon: "󰲧 ",
            h5_icon: "󰲩 ",
            h6_icon: "󰲫 ",

            // Heading colors matching render-markdown.nvim with typical colorscheme
            // H1 = DiffAdd (blue in your screenshot)
            h1_fg: Color::Rgb(255, 255, 255),
            h1_bg: Color::Rgb(30, 58, 138), // Deep blue
            // H2 = DiffChange (cyan/teal in your screenshot)
            h2_fg: Color::Rgb(255, 255, 255),
            h2_bg: Color::Rgb(8, 145, 178), // Cyan/teal
            // H3 = DiffDelete (purple/magenta in your screenshot)
            h3_fg: Color::Rgb(255, 255, 255),
            h3_bg: Color::Rgb(168, 85, 247), // Purple/magenta

            // Bullet points
            bullet_l1: "● ",
            bullet_l2: "○ ",
            bullet_l3: "◆ ",

            // Code blocks
            code_block_border: true,
            code_block_bg: Color::Rgb(40, 42, 54),
            inline_code_bg: Color::Rgb(68, 71, 90),
            inline_code_fg: Color::Rgb(139, 233, 253),

            // Block quotes
            quote_icon: "▐ ",
            quote_fg: Color::Rgb(139, 233, 253),
            quote_bg: Color::Rgb(40, 42, 54),

            // Callouts
            callout_note_icon: "󰋽 ",
            callout_tip_icon: "󰌶 ",
            callout_warning_icon: "󰀪 ",
            callout_caution_icon: "󰳦 ",
        }
    }
}
