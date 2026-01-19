//! Render the theme picker modal dialog.
//!
//! Displays a centered modal with a search input and scrollable list of themes.
//! The currently hovered theme is applied live for preview effect.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::helpers::{all_app_themes, get_app_theme_display_name};

/// Maximum visible themes in the picker (determines popup height).
const MAX_VISIBLE_THEMES: usize = 20;

/// Render the theme picker modal dialog.
///
/// Shows a centered popup with:
/// - A search input at the top that filters themes as you type
/// - A scrollable list of theme names
/// - The currently selected theme is highlighted with a selection indicator
/// - The saved/active theme shows a checkmark
///
/// # Arguments
///
/// * `frame` - The ratatui frame to render into
/// * `app` - Application state containing theme picker state
///
/// # Layout
///
/// The modal is centered on screen with:
/// - A title bar showing "Select Theme"
/// - A search input field
/// - A scrollable list of theme names
/// - Footer hints for navigation keys
pub fn render_theme_picker(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let themes = all_app_themes();

    // Filter themes based on search input
    let filtered_themes: Vec<(usize, &str)> = themes
        .iter()
        .enumerate()
        .filter(|(_, name)| {
            if app.theme_filter.is_empty() {
                true
            } else {
                let filter_lower = app.theme_filter.to_lowercase();
                name.to_lowercase().contains(&filter_lower)
                    || get_app_theme_display_name(name)
                        .to_lowercase()
                        .contains(&filter_lower)
            }
        })
        .map(|(i, name)| (i, *name))
        .collect();

    // Calculate popup dimensions
    let popup_width = 44u16;
    let visible_count = filtered_themes.len().min(MAX_VISIBLE_THEMES);
    let popup_height = (visible_count + 7) as u16; // +7 for borders, search, separator, and footer

    // Center the popup
    let popup_area = Rect {
        x: area.width.saturating_sub(popup_width) / 2,
        y: area.height.saturating_sub(popup_height) / 2,
        width: popup_width.min(area.width),
        height: popup_height.min(area.height),
    };

    // Clear the popup area
    frame.render_widget(Clear, popup_area);

    // Calculate scroll offset for viewport
    let scroll_offset =
        calculate_scroll_offset(app.theme_picker_index, visible_count, filtered_themes.len());

    // Build content lines
    let mut items: Vec<Line> = Vec::new();

    // Search input line
    let search_style = Style::default().fg(app.current_theme.text);
    let cursor = if app.theme_filter.is_empty() { "_" } else { "" };
    items.push(Line::from(vec![
        Span::styled(" / ", Style::default().fg(app.current_theme.text_muted)),
        Span::styled(
            format!("{}{}", app.theme_filter, cursor),
            search_style.add_modifier(Modifier::BOLD),
        ),
    ]));

    // Separator line
    let separator = "─".repeat(popup_width.saturating_sub(2) as usize);
    items.push(Line::from(Span::styled(
        separator,
        Style::default().fg(app.current_theme.border),
    )));

    // Theme list
    if filtered_themes.is_empty() {
        items.push(Line::from(Span::styled(
            "   No matching themes",
            Style::default().fg(app.current_theme.text_muted),
        )));
    } else {
        for (filtered_idx, (original_idx, theme_name)) in filtered_themes
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_count)
        {
            let display_name = get_app_theme_display_name(theme_name);
            let is_selected = filtered_idx == app.theme_picker_index;
            let is_saved = *original_idx == app.saved_theme_index;

            // Build prefix with selection and saved indicators
            let prefix = if is_selected { " > " } else { "   " };
            let suffix = if is_saved { " *" } else { "" };

            // Use theme's own colors for styling
            let style = if is_selected {
                Style::default()
                    .fg(app.current_theme.primary)
                    .bg(app.current_theme.background_panel)
                    .add_modifier(Modifier::BOLD)
            } else if is_saved {
                Style::default()
                    .fg(app.current_theme.success)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.current_theme.text)
            };

            items.push(Line::from(Span::styled(
                format!("{}{}{}", prefix, display_name, suffix),
                style,
            )));
        }
    }

    // Add footer line with navigation hints
    items.push(Line::from(""));
    items.push(Line::from(vec![
        Span::styled(" [", Style::default().fg(app.current_theme.text_muted)),
        Span::styled("j/k", Style::default().fg(app.current_theme.accent)),
        Span::styled(
            "] scroll  [",
            Style::default().fg(app.current_theme.text_muted),
        ),
        Span::styled("Enter", Style::default().fg(app.current_theme.success)),
        Span::styled(
            "] select  [",
            Style::default().fg(app.current_theme.text_muted),
        ),
        Span::styled("Esc", Style::default().fg(app.current_theme.error)),
        Span::styled(
            "] cancel",
            Style::default().fg(app.current_theme.text_muted),
        ),
    ]));

    // Build the title with count indicator
    let title = if !app.theme_filter.is_empty() {
        format!(
            " Select Theme ({}/{}) ",
            filtered_themes.len(),
            themes.len()
        )
    } else if filtered_themes.len() > visible_count {
        format!(
            " Select Theme ({}/{}) ",
            app.theme_picker_index + 1,
            filtered_themes.len()
        )
    } else {
        " Select Theme ".to_string()
    };

    let popup = Paragraph::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.current_theme.border_active))
            .style(Style::default().bg(app.current_theme.background_menu))
            .title(Span::styled(
                title,
                Style::default()
                    .fg(app.current_theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
    );

    frame.render_widget(popup, popup_area);
}

/// Calculate the scroll offset to keep the selected item visible.
///
/// # Arguments
///
/// * `selected_index` - Currently selected theme index
/// * `visible_count` - Number of visible items in viewport
/// * `total_count` - Total number of themes
///
/// # Returns
///
/// The scroll offset to apply to the list.
fn calculate_scroll_offset(
    selected_index: usize,
    visible_count: usize,
    total_count: usize,
) -> usize {
    if total_count <= visible_count {
        return 0;
    }

    // Keep selected item roughly centered
    let half_visible = visible_count / 2;

    if selected_index <= half_visible {
        0
    } else if selected_index >= total_count - half_visible {
        total_count - visible_count
    } else {
        selected_index - half_visible
    }
}

/// Get the filtered themes list based on the current filter.
///
/// Returns a vector of (original_index, theme_name) tuples.
pub fn get_filtered_themes(filter: &str) -> Vec<(usize, &'static str)> {
    let themes = all_app_themes();
    themes
        .iter()
        .enumerate()
        .filter(|(_, name)| {
            if filter.is_empty() {
                true
            } else {
                let filter_lower = filter.to_lowercase();
                name.to_lowercase().contains(&filter_lower)
                    || get_app_theme_display_name(name)
                        .to_lowercase()
                        .contains(&filter_lower)
            }
        })
        .map(|(i, name)| (i, *name))
        .collect()
}
