use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;
use std::time::{Duration, Instant};

/// Toast notification level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Success,
    Error,
    Info,
    Warning,
}

impl ToastLevel {
    /// Get the color for this toast level
    pub fn color(&self) -> Color {
        match self {
            ToastLevel::Success => Color::Green,
            ToastLevel::Error => Color::Red,
            ToastLevel::Info => Color::Cyan,
            ToastLevel::Warning => Color::Yellow,
        }
    }

    /// Get the icon for this toast level
    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Success => "✓",
            ToastLevel::Error => "✗",
            ToastLevel::Info => "ℹ",
            ToastLevel::Warning => "⚠",
        }
    }
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub created_at: Instant,
    pub duration: Duration,
}

impl Toast {
    /// Create a new toast with default 3 second duration
    pub fn new(message: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
        }
    }

    /// Create a toast with custom duration
    pub fn with_duration(
        message: impl Into<String>,
        level: ToastLevel,
        duration: Duration,
    ) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: Instant::now(),
            duration,
        }
    }

    /// Check if this toast has expired
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    /// Get the remaining lifetime as a percentage (0.0 to 1.0)
    pub fn lifetime_percent(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (total - elapsed) / total
    }
}

/// Manages multiple toast notifications
#[derive(Debug, Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
    max_toasts: usize,
}

impl ToastManager {
    /// Create a new toast manager
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max_toasts: 5, // Maximum number of toasts to show at once
        }
    }

    /// Add a new toast
    pub fn add(&mut self, toast: Toast) {
        // Remove expired toasts before adding new one
        self.remove_expired();

        // Add the new toast
        self.toasts.push(toast);

        // Keep only the most recent toasts if we exceed max
        if self.toasts.len() > self.max_toasts {
            self.toasts.drain(0..self.toasts.len() - self.max_toasts);
        }

        tracing::debug!("Toast added, total toasts: {}", self.toasts.len());
    }

    /// Add a success toast
    pub fn success(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Success));
    }

    /// Add an error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Error));
    }

    /// Add an info toast
    pub fn info(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Info));
    }

    /// Add a warning toast
    pub fn warning(&mut self, message: impl Into<String>) {
        self.add(Toast::new(message, ToastLevel::Warning));
    }

    /// Remove expired toasts
    pub fn remove_expired(&mut self) {
        let before = self.toasts.len();
        self.toasts.retain(|toast| !toast.is_expired());
        let removed = before - self.toasts.len();
        if removed > 0 {
            tracing::debug!("Removed {} expired toasts", removed);
        }
    }

    /// Get all active toasts
    pub fn get_active(&self) -> &[Toast] {
        &self.toasts
    }

    /// Check if there are any active toasts
    pub fn has_toasts(&self) -> bool {
        !self.toasts.is_empty()
    }

    /// Clear all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

/// Render toasts in the bottom-right corner of the screen
pub fn render_toasts(frame: &mut Frame, toasts: &ToastManager) {
    let active_toasts = toasts.get_active();
    if active_toasts.is_empty() {
        return;
    }

    let area = frame.area();

    // Constants for toast sizing
    const TOAST_WIDTH: u16 = 40;
    const TOAST_HEIGHT: u16 = 3; // 1 line of text + borders
    const TOAST_MARGIN: u16 = 2;
    const TOAST_SPACING: u16 = 1;

    // Calculate position for toasts (bottom-right corner)
    let mut y_offset = area.height.saturating_sub(TOAST_MARGIN);

    // Render toasts from bottom to top (newest at bottom)
    for toast in active_toasts.iter().rev() {
        // Calculate toast area
        let toast_y = y_offset.saturating_sub(TOAST_HEIGHT);
        let toast_x = area.width.saturating_sub(TOAST_WIDTH + TOAST_MARGIN);

        let toast_area = Rect {
            x: toast_x,
            y: toast_y,
            width: TOAST_WIDTH,
            height: TOAST_HEIGHT,
        };

        // Don't render if toast would be off-screen
        if toast_y == 0 || toast_x == 0 {
            break;
        }

        // Clear the area behind the toast
        frame.render_widget(Clear, toast_area);

        // Create the toast widget
        let color = toast.level.color();
        let icon = toast.level.icon();

        let text = Line::from(vec![
            Span::raw("  "), // Extra left margin
            Span::styled(
                icon,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "), // Extra space after icon
            Span::raw(&toast.message),
            Span::raw(" "), // Extra right margin
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, toast_area);

        // Move up for the next toast
        y_offset = toast_y.saturating_sub(TOAST_SPACING);
    }
}
