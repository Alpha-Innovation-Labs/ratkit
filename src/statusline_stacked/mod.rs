//! A status-line widget that can stack up indicators
//! on the left and right end.
//!
//! If you use the constants SLANT_TL_BR and SLANT_BL_TR as
//! separator you can do neo-vim/neovim style statusline.
//!
//! This is adapted from rat-widget's StatusLineStacked.
//!
//! # Example
//!
//! ```rust,no_run
//! use ratatui::style::{Color, Style};
//! use ratatui::text::Span;
//! use ratatui_toolkit::statusline_stacked::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};
//!
//! StatusLineStacked::new()
//!     .start(
//!         Span::from(" STATUS ").style(Style::new().fg(Color::Black).bg(Color::DarkGray)),
//!         Span::from(SLANT_TL_BR).style(Style::new().fg(Color::DarkGray).bg(Color::Green)),
//!     )
//!     .start(
//!         Span::from(" OPERATIONAL ").style(Style::new().fg(Color::Black).bg(Color::Green)),
//!         Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Green)),
//!     )
//!     .center("Some status message...")
//!     .end(
//!         Span::from(" INFO ").style(Style::new().fg(Color::Black).bg(Color::Cyan)),
//!         Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Cyan)),
//!     );
//! ```

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use std::marker::PhantomData;

/// PowerLine block cut at the diagonal (top-left to bottom-right).
/// Requires a Nerd Font or PowerLine font.
pub const SLANT_TL_BR: &str = "\u{e0b8}";

/// PowerLine block cut at the diagonal (bottom-left to top-right).
/// Requires a Nerd Font or PowerLine font.
pub const SLANT_BL_TR: &str = "\u{e0ba}";

/// Statusline with stacked indicators on the left and right side.
///
/// This widget creates a statusline with a "stacked" appearance using
/// PowerLine-style diagonal separators. It has three sections:
/// - Left: Stack indicators from left to right
/// - Center: Centered status message
/// - Right: Stack indicators from right to left
#[derive(Debug, Default, Clone)]
pub struct StatusLineStacked<'a> {
    style: Style,
    left: Vec<(Line<'a>, Line<'a>)>,
    center_margin: u16,
    center: Line<'a>,
    right: Vec<(Line<'a>, Line<'a>)>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> StatusLineStacked<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Baseline style for the center area.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Add to the start group of status flags.
    /// These stack from left to right.
    ///
    /// # Arguments
    /// * `text` - The text/span to display
    /// * `gap` - The separator (usually SLANT_TL_BR or SLANT_BL_TR)
    pub fn start(mut self, text: impl Into<Line<'a>>, gap: impl Into<Line<'a>>) -> Self {
        self.left.push((text.into(), gap.into()));
        self
    }

    /// Add to the start group without a separator.
    /// Useful for the last item in the start group.
    pub fn start_bare(mut self, text: impl Into<Line<'a>>) -> Self {
        self.left.push((text.into(), "".into()));
        self
    }

    /// Margin around centered text.
    pub fn center_margin(mut self, margin: u16) -> Self {
        self.center_margin = margin;
        self
    }

    /// Centered text.
    pub fn center(mut self, text: impl Into<Line<'a>>) -> Self {
        self.center = text.into();
        self
    }

    /// Add to the end group of status flags.
    /// These stack from right to left.
    ///
    /// # Arguments
    /// * `text` - The text/span to display
    /// * `gap` - The separator (usually SLANT_BL_TR)
    pub fn end(mut self, text: impl Into<Line<'a>>, gap: impl Into<Line<'a>>) -> Self {
        self.right.push((text.into(), gap.into()));
        self
    }

    /// Add to the end group without a separator.
    /// Useful for the last item in the end group.
    pub fn end_bare(mut self, text: impl Into<Line<'a>>) -> Self {
        self.right.push((text.into(), "".into()));
        self
    }
}

impl<'a> Widget for StatusLineStacked<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render right side (from right to left)
        let mut x_end = area.right();
        for (status, gap) in self.right.iter() {
            let width = status.width() as u16;
            status.render(
                Rect::new(x_end.saturating_sub(width), area.y, width, 1),
                buf,
            );
            x_end = x_end.saturating_sub(width);

            let width = gap.width() as u16;
            gap.render(
                Rect::new(x_end.saturating_sub(width), area.y, width, 1),
                buf,
            );
            x_end = x_end.saturating_sub(width);
        }

        // Render left side (from left to right)
        let mut x_start = area.x;
        for (status, gap) in self.left.iter() {
            let width = status.width() as u16;
            status.render(Rect::new(x_start, area.y, width, 1), buf);
            x_start += width;

            let width = gap.width() as u16;
            gap.render(Rect::new(x_start, area.y, width, 1), buf);
            x_start += width;
        }

        // Fill center with base style
        buf.set_style(
            Rect::new(x_start, area.y, x_end.saturating_sub(x_start), 1),
            self.style,
        );

        // Render centered text
        let center_width = x_end
            .saturating_sub(x_start)
            .saturating_sub(self.center_margin * 2);

        self.center.render(
            Rect::new(x_start + self.center_margin, area.y, center_width, 1),
            buf,
        );
    }
}

/// Operational mode for styled statusline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationalMode {
    /// Normal operation (green)
    #[default]
    Operational,
    /// Warning state (yellow)
    Dire,
    /// Critical state (red)
    Evacuate,
}

/// Builder for creating a styled statusline that looks like rat-salsa's menu_status2.
///
/// This provides a pre-configured statusline with the "Westinghouse" reactor-control aesthetic.
pub struct StyledStatusLine<'a> {
    mode: OperationalMode,
    title: &'a str,
    center_text: String,
    render_count: usize,
    event_count: usize,
    render_time_us: u64,
    event_time_us: u64,
    message_count: u32,
    use_slants: bool,
}

impl<'a> StyledStatusLine<'a> {
    /// Create a new styled statusline with default "WESTINGHOUSE[STATUS]2" theme.
    pub fn new() -> Self {
        Self {
            mode: OperationalMode::Operational,
            title: " WESTINGHOUSE[STATUS]2 ",
            center_text: String::new(),
            render_count: 0,
            event_count: 0,
            render_time_us: 0,
            event_time_us: 0,
            message_count: 0,
            use_slants: true,
        }
    }

    /// Set the operational mode (changes color scheme).
    pub fn mode(mut self, mode: OperationalMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set custom title (default is "WESTINGHOUSE[STATUS]2").
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Set the centered status message.
    pub fn center_text(mut self, text: impl Into<String>) -> Self {
        self.center_text = text.into();
        self
    }

    /// Set render metrics.
    pub fn render_metrics(mut self, count: usize, time_us: u64) -> Self {
        self.render_count = count;
        self.render_time_us = time_us;
        self
    }

    /// Set event metrics.
    pub fn event_metrics(mut self, count: usize, time_us: u64) -> Self {
        self.event_count = count;
        self.event_time_us = time_us;
        self
    }

    /// Set message count.
    pub fn message_count(mut self, count: u32) -> Self {
        self.message_count = count;
        self
    }

    /// Use slanted separators (default: true). Set to false for a simpler look.
    pub fn use_slants(mut self, use_slants: bool) -> Self {
        self.use_slants = use_slants;
        self
    }

    /// Build the statusline widget.
    pub fn build(self) -> StatusLineStacked<'a> {
        // Color scheme from RADIUM palette (rat-salsa's menu_status2)
        // These match exactly: pal.gray[3], pal.green[3], pal.yellow[3], pal.red[3], pal.cyan[0], pal.cyan[7]
        let color_title = Color::Rgb(70, 73, 77); // gray[3]
        let color_mode = match self.mode {
            OperationalMode::Operational => Color::Rgb(42, 193, 138), // green[3]
            OperationalMode::Dire => Color::Rgb(255, 210, 88),        // yellow[3]
            OperationalMode::Evacuate => Color::Rgb(246, 90, 90),     // red[3]
        };
        let color_info = Color::Rgb(44, 163, 170); // cyan[0]
        let color_dark = Color::Rgb(80, 202, 210); // cyan[7]
        let text_black = Color::Rgb(16, 19, 23); // text_black

        let mode_str = match self.mode {
            OperationalMode::Operational => " OPERATIONAL ",
            OperationalMode::Dire => " DIRE ",
            OperationalMode::Evacuate => " EVACUATE ",
        };

        if self.use_slants {
            // Style 1: With slanted separators (like menu_status2 stacked_1)
            StatusLineStacked::new()
                .style(Style::new().fg(Color::White).bg(color_dark))
                .start(
                    Span::from(self.title).style(Style::new().fg(text_black).bg(color_title)),
                    Span::from(SLANT_TL_BR).style(Style::new().fg(color_title).bg(color_mode)),
                )
                .start(
                    Span::from(mode_str).style(Style::new().fg(text_black).bg(color_mode)),
                    Span::from(SLANT_TL_BR).style(Style::new().fg(color_mode)),
                )
                .center_margin(1)
                .center(self.center_text)
                .end(
                    Span::from(format!(
                        "R[{}][{}µs] ",
                        self.render_count, self.render_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info).bg(color_dark)),
                )
                .end(
                    "",
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_dark).bg(color_info)),
                )
                .end(
                    Span::from(format!(
                        "E[{}][{}µs] ",
                        self.event_count, self.event_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info).bg(color_dark)),
                )
                .end(
                    "",
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_dark).bg(color_info)),
                )
                .end(
                    Span::from(format!("MSG[{}] ", self.message_count))
                        .style(Style::new().fg(text_black).bg(color_info)),
                    Span::from(SLANT_BL_TR).style(Style::new().fg(color_info)),
                )
        } else {
            // Style 2: Simple style without decorative slants (like menu_status2 stacked_2)
            StatusLineStacked::new()
                .style(Style::new().fg(Color::White).bg(color_dark))
                .start_bare(
                    Span::from(self.title).style(Style::new().fg(Color::White).bg(color_title)),
                )
                .start_bare(Span::from(mode_str).style(Style::new().fg(text_black).bg(color_mode)))
                .center_margin(1)
                .center(self.center_text)
                .end_bare(
                    Span::from(format!(
                        "R[{}][{}µs] ",
                        self.render_count, self.render_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                )
                .end_bare(
                    Span::from(format!(
                        "E[{}][{}µs] ",
                        self.event_count, self.event_time_us
                    ))
                    .style(Style::new().fg(text_black).bg(color_info)),
                )
                .end_bare(
                    Span::from(format!(" MSG[{}] ", self.message_count))
                        .style(Style::new().fg(text_black).bg(color_info)),
                )
        }
    }
}

impl<'a> Default for StyledStatusLine<'a> {
    fn default() -> Self {
        Self::new()
    }
}
