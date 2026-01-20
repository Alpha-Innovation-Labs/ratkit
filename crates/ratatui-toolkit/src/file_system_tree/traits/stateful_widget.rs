use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::StatefulWidget,
};

use crate::file_system_tree::{FileSystemEntry, FileSystemTree};
use crate::primitives::tree_view::{matches_filter, TreeViewRef, TreeViewState};

use crate::file_system_tree::traits::get_ayu_dark_color::get_ayu_dark_color;
use crate::file_system_tree::traits::get_custom_icon::get_custom_icon;
use devicons::{icon_for_file, Theme as DevIconTheme};

/// Filter function for FileSystemEntry nodes.
///
/// Returns true if the entry's name matches the filter (case-insensitive contains).
fn entry_matches_filter(entry: &FileSystemEntry, filter: &Option<String>) -> bool {
    matches_filter(&entry.name, filter)
}

/// Renders the filter input line at the bottom of the tree.
fn render_filter_line(filter_text: Option<&str>, filter_mode: bool, area: Rect, buf: &mut Buffer) {
    if area.height == 0 {
        return;
    }

    let y = area.y + area.height - 1;

    // Build the filter line
    let filter_str = filter_text.unwrap_or("");
    let cursor = if filter_mode { "_" } else { "" };

    let line = Line::from(vec![
        Span::styled(
            "/ ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(filter_str, Style::default().fg(Color::White)),
        Span::styled(
            cursor,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);

    // Fill background for the filter line
    let bg_style = Style::default().bg(Color::Rgb(15, 25, 40));
    for x in area.x..(area.x + area.width) {
        buf[(x, y)].set_style(bg_style);
    }

    buf.set_line(area.x, y, &line, area.width);
}

impl<'a> StatefulWidget for FileSystemTree<'a> {
    type State = TreeViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let config = self.config;
        let block = self.block;
        let filter_mode = state.filter_mode;
        let has_filter = state.filter.as_ref().is_some_and(|f| !f.is_empty());
        let show_filter_line = filter_mode || has_filter;

        // Calculate tree area (leave room for filter line if needed)
        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let tree_view = TreeViewRef::new(&self.nodes)
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
            .filter_fn(entry_matches_filter)
            .highlight_style(Style::default().bg(Color::Rgb(15, 25, 40)));

        let tree_view = if let Some(block) = block {
            tree_view.block(block)
        } else {
            tree_view
        };

        tree_view.render(tree_area, buf, state);

        // Render filter line if needed
        if show_filter_line && area.height > 1 {
            render_filter_line(state.filter.as_deref(), filter_mode, area, buf);
        }
    }
}
