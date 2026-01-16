use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::StatefulWidget,
};

use crate::file_system_tree::FileSystemTree;
use crate::tree_view::TreeViewState;

use crate::file_system_tree::traits::get_ayu_dark_color::get_ayu_dark_color;
use crate::file_system_tree::traits::get_custom_icon::get_custom_icon;
use devicons::{icon_for_file, Theme as DevIconTheme};

impl<'a> StatefulWidget for FileSystemTree<'a> {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let config = self.config;
        let block = self.block;

        let tree_view = crate::tree_view::TreeView::new(self.nodes)
            .icons("", "")
            .render_fn(move |entry, node_state| {
                let (icon_glyph, icon_color) = if entry.is_dir {
                    if node_state.is_expanded {
                        ('\u{f07c}', Color::Rgb(31, 111, 136))
                    } else {
                        ('\u{f07b}', Color::Rgb(31, 111, 136))
                    }
                } else {
                    let theme = if config.use_dark_theme {
                        DevIconTheme::Dark
                    } else {
                        DevIconTheme::Light
                    };
                    let icon_char = if let Some((custom_icon, _)) = get_custom_icon(&entry.name) {
                        custom_icon
                    } else {
                        let file_icon = icon_for_file(&entry.name, &Some(theme));
                        file_icon.icon
                    };
                    let color = get_ayu_dark_color(&entry.name);
                    (icon_char, color)
                };

                let style = ratatui::style::Style::default().fg(icon_color);

                Line::from(vec![
                    Span::styled(
                        format!("{} ", icon_glyph),
                        ratatui::style::Style::default().fg(icon_color),
                    ),
                    Span::styled(entry.name.clone(), style),
                ])
            })
            .highlight_style(Style::default().bg(Color::Rgb(15, 25, 40)));

        let tree_view = if let Some(block) = block {
            tree_view.block(block)
        } else {
            tree_view
        };

        tree_view.render(area, buf, state);
    }
}
