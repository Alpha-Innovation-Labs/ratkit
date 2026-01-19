mod with_theme;

use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::statusbar::{StatusBar, StatusItem};

impl<'a> StatusBar<'a> {
    pub(super) fn build_spans(&self, items: &[StatusItem<'a>]) -> Vec<Span<'_>> {
        let mut spans = Vec::new();
        for (i, item) in items.iter().enumerate() {
            spans.push(Span::styled(item.text.clone(), item.style));
            if i < items.len() - 1 {
                if let Some(sep) = item.separator {
                    spans.push(Span::styled(sep, Style::default().fg(Color::DarkGray)));
                }
            }
        }
        spans
    }
}
