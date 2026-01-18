//! Events emitted by the markdown widget.

/// Events that can be emitted by the markdown widget.
///
/// The widget handles all internal state management and returns these events
/// so the parent application can react appropriately (e.g., show toast messages).
#[derive(Debug, Clone)]
pub enum MarkdownEvent {
    /// No event occurred.
    None,

    /// The focused line changed (via click or keyboard navigation).
    FocusedLine {
        /// The new focused line number (1-indexed).
        line: usize,
    },

    /// A heading was toggled (collapsed/expanded).
    HeadingToggled {
        /// The heading level (1-6).
        level: u8,
        /// The heading text.
        text: String,
        /// Whether the heading is now collapsed.
        collapsed: bool,
    },

    /// A double-click occurred on a line.
    DoubleClick {
        /// Source line number (1-indexed).
        line_number: usize,
        /// Type of line clicked (e.g., "Heading", "Paragraph").
        line_kind: String,
        /// Text content of the line.
        content: String,
    },

    /// Text was copied to clipboard.
    Copied {
        /// The text that was copied.
        text: String,
    },

    /// Selection mode was entered (drag started).
    SelectionStarted,

    /// Selection mode was exited.
    SelectionEnded,

    /// Content was scrolled.
    Scrolled {
        /// The new scroll offset.
        offset: usize,
        /// Direction of scroll (positive = down, negative = up).
        direction: i32,
    },
}
