//! Helper function for rendering a single tree entry line.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::diff_file_tree::helpers::file_icon;
use crate::diff_file_tree::{DiffFileEntry, FileStatus};
use crate::theme::AppTheme;
use crate::tree_view::NodeState;

/// Gets the color for a file status from the theme.
fn status_color(status: FileStatus, theme: &AppTheme) -> Color {
    match status {
        FileStatus::Added => theme.diff.added,
        FileStatus::Modified => theme.warning,
        FileStatus::Deleted => theme.diff.removed,
        FileStatus::Renamed => theme.info,
    }
}

/// Renders a diff file entry as a styled line.
///
/// # Arguments
///
/// * `entry` - The diff file entry to render
/// * `node_state` - The current state of the node (selected, expanded, etc.)
/// * `focused` - Whether the tree widget has focus
/// * `theme` - Application theme for styling
///
/// # Returns
///
/// A styled `Line` for rendering.
pub fn render_entry<'a>(
    entry: &DiffFileEntry,
    node_state: &NodeState,
    focused: bool,
    theme: &AppTheme,
) -> Line<'a> {
    let mut spans = Vec::new();

    // Status marker (only for files, not directories)
    // Directories get their folder icons from tree_view's .icons() method
    if !entry.is_dir {
        let (marker, marker_color) = if let Some(status) = entry.status {
            (status.prefix(), status_color(status, theme))
        } else {
            (" ", theme.text_muted)
        };
        spans.push(Span::styled(
            format!("{} ", marker),
            Style::default().fg(marker_color),
        ));

        // File type icon based on extension
        let icon = file_icon(&entry.name);
        spans.push(Span::styled(
            format!("{} ", icon),
            Style::default().fg(theme.accent),
        ));
    }

    // Name (with directory slash)
    let name = if entry.is_dir {
        format!("{}/", entry.name)
    } else {
        entry.name.clone()
    };

    let name_style = if node_state.is_selected && focused {
        Style::default()
            .fg(theme.selected_text)
            .bg(theme.primary)
            .add_modifier(Modifier::BOLD)
    } else if node_state.is_selected {
        Style::default()
            .fg(theme.text)
            .bg(theme.background_element)
    } else if entry.is_dir {
        // Directories use info color (blue) to distinguish from modified files (yellow)
        Style::default().fg(theme.info)
    } else if let Some(status) = entry.status {
        Style::default().fg(status_color(status, theme))
    } else {
        Style::default().fg(theme.text)
    };

    spans.push(Span::styled(name, name_style));

    Line::from(spans)
}
