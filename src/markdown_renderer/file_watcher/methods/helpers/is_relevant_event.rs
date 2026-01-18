//! Check if an event is relevant for triggering a reload.

use notify::{event::ModifyKind, Event, EventKind};

/// Check if an event is relevant for triggering a reload.
///
/// Filters for data modification events (content changes) and ignores
/// metadata-only changes.
pub fn is_relevant_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Modify(ModifyKind::Data(_))
            | EventKind::Modify(ModifyKind::Any)
            | EventKind::Create(_)
            | EventKind::Remove(_)
    )
}
