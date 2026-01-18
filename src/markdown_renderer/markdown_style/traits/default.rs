//! Default trait implementation for MarkdownStyle.

use ratatui::style::Color;

use crate::markdown_renderer::markdown_style::MarkdownStyle;

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
            // H1 = Deep blue
            h1_fg: Color::Rgb(255, 255, 255),
            h1_bg: Color::Rgb(30, 58, 138),
            // H2 = Cyan/teal
            h2_fg: Color::Rgb(255, 255, 255),
            h2_bg: Color::Rgb(8, 145, 178),
            // H3 = Purple/magenta
            h3_fg: Color::Rgb(255, 255, 255),
            h3_bg: Color::Rgb(168, 85, 247),
            // H4 = Orange/amber
            h4_fg: Color::Rgb(255, 255, 255),
            h4_bg: Color::Rgb(217, 119, 6),
            // H5 = Gray
            h5_fg: Color::Rgb(255, 255, 255),
            h5_bg: Color::Rgb(107, 114, 128),
            // H6 = Dark gray
            h6_fg: Color::Rgb(255, 255, 255),
            h6_bg: Color::Rgb(75, 85, 99),

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

            // Text colors
            text_fg: Color::Rgb(220, 220, 220),
            text_bg: Color::Reset,
            link_fg: Color::Rgb(97, 175, 239),
            emph_fg: Color::Rgb(198, 120, 221),
            strong_fg: Color::Rgb(191, 97, 106),
            hr_fg: Color::Rgb(144, 145, 156),
            table_border_fg: Color::Rgb(96, 98, 109),
        }
    }
}
