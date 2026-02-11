//! Core types for the layout manager.

use ratatui::layout::Rect;
use std::fmt;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Unique identifier for a UI element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(Uuid);

impl ElementId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Layout region within the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    /// Top region (e.g., menu bar, header)
    Top,
    /// Center region (e.g., main content, panes)
    Center,
    /// Bottom region (e.g., status bar, footer)
    Bottom,
}

/// Visibility state of an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// Element is visible and interactive.
    Visible,
    /// Element is hidden but retains state.
    Hidden,
}

/// Metadata about a registered element.
#[derive(Debug, Clone)]
pub struct ElementMetadata {
    /// Unique identifier for this element.
    pub id: ElementId,
    /// Layout region this element belongs to.
    pub region: Region,
    /// Current visibility state.
    pub visibility: Visibility,
    /// Z-order within the region (higher = rendered on top).
    pub z_order: u32,
    /// Whether this element can receive focus.
    pub focusable: bool,
    /// Calculated rectangle for this element (updated by layout manager).
    pub rect: Rect,
    /// Optional height for top/bottom regions (0 = auto/default).
    pub fixed_height: Option<u16>,
    /// Optional capture state for mouse events.
    pub mouse_capture: Option<ElementId>,
}

impl ElementMetadata {
    pub fn new(id: ElementId, region: Region) -> Self {
        Self {
            id,
            region,
            visibility: Visibility::Visible,
            z_order: 0,
            focusable: false,
            rect: Rect::default(),
            fixed_height: None,
            mouse_capture: None,
        }
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn with_z_order(mut self, z_order: u32) -> Self {
        self.z_order = z_order;
        self
    }

    pub fn with_focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    pub fn with_fixed_height(mut self, height: u16) -> Self {
        self.fixed_height = Some(height);
        self
    }

    pub fn is_visible(&self) -> bool {
        self.visibility == Visibility::Visible
    }

    pub fn can_receive_focus(&self) -> bool {
        self.is_visible() && self.focusable
    }
}

/// Current layout state computed by the layout manager.
#[derive(Debug, Clone)]
pub struct LayoutState {
    /// Available terminal area.
    pub terminal_area: Rect,
    /// Calculated area for top region.
    pub top_area: Rect,
    /// Calculated area for center region.
    pub center_area: Rect,
    /// Calculated area for bottom region.
    pub bottom_area: Rect,
    /// Total height allocated to top region.
    pub top_height: u16,
    /// Total height allocated to bottom region.
    pub bottom_height: u16,
}

impl LayoutState {
    pub fn new(terminal_area: Rect) -> Self {
        Self {
            terminal_area,
            top_area: Rect::default(),
            center_area: Rect::default(),
            bottom_area: Rect::default(),
            top_height: 0,
            bottom_height: 0,
        }
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self::new(Rect::default())
    }
}

/// Dirty flag state for layout and rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyFlags {
    /// Layout needs recomputation (resize, add/remove, visibility change).
    pub layout_dirty: bool,
    /// Elements need redraw (internal state change).
    pub elements_dirty: bool,
}

impl DirtyFlags {
    pub fn clean() -> Self {
        Self {
            layout_dirty: false,
            elements_dirty: false,
        }
    }

    pub fn all_dirty() -> Self {
        Self {
            layout_dirty: true,
            elements_dirty: true,
        }
    }

    pub fn set_layout_dirty(&mut self) {
        self.layout_dirty = true;
    }

    pub fn set_elements_dirty(&mut self) {
        self.elements_dirty = true;
    }

    pub fn needs_redraw(&self) -> bool {
        self.layout_dirty || self.elements_dirty
    }

    pub fn clear(&mut self) {
        self.layout_dirty = false;
        self.elements_dirty = false;
    }
}

impl Default for DirtyFlags {
    fn default() -> Self {
        Self::clean()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseCaptureState {
    None,
    Captured {
        element_id: ElementId,
        captured_at: Instant,
        timeout: Option<Duration>,
    },
}

impl MouseCaptureState {
    pub fn is_captured(&self) -> bool {
        matches!(self, MouseCaptureState::Captured { .. })
    }

    pub fn element_id(&self) -> Option<ElementId> {
        match self {
            MouseCaptureState::Captured { element_id, .. } => Some(*element_id),
            MouseCaptureState::None => None,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self {
            MouseCaptureState::Captured {
                captured_at,
                timeout: Some(duration),
                ..
            } => captured_at.elapsed() > *duration,
            MouseCaptureState::Captured { timeout: None, .. } => false,
            MouseCaptureState::None => false,
        }
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        match self {
            MouseCaptureState::Captured {
                captured_at,
                timeout: Some(duration),
                ..
            } => {
                let elapsed = captured_at.elapsed();
                if elapsed >= *duration {
                    Some(Duration::ZERO)
                } else {
                    Some(*duration - elapsed)
                }
            }
            MouseCaptureState::Captured { timeout: None, .. } => None,
            MouseCaptureState::None => None,
        }
    }
}

impl Default for MouseCaptureState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub struct MouseSnapshot {
    pub captured_element: Option<ElementId>,
    pub z_order_hits: Vec<(ElementId, Rect)>,
    pub captured_at: Instant,
}

impl MouseSnapshot {
    pub fn new(captured_element: Option<ElementId>, z_order_hits: Vec<(ElementId, Rect)>) -> Self {
        Self {
            captured_element,
            z_order_hits,
            captured_at: Instant::now(),
        }
    }

    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.captured_at.elapsed() > max_age
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticInfo {
    pub total_elements: usize,
    pub visible_elements: usize,
    pub focusable_elements: usize,
    pub focused_element: Option<ElementId>,
    pub captured_element: Option<ElementId>,
    pub terminal_size: (u16, u16),
    pub region_areas: Vec<(Region, Rect)>,
    pub z_order_top: Vec<(ElementId, Region, u32)>,
    pub dirty_flags: DirtyFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDebounceState {
    Idle,
    Pending {
        pending_width: u16,
        pending_height: u16,
        scheduled_at: Instant,
    },
}
