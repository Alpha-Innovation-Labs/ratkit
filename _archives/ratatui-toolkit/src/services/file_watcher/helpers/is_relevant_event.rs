//! Filter for relevant file system events.

use notify::{event::ModifyKind, Event, EventKind};

/// Check if an event is relevant (a file content change).
///
/// Filters out metadata-only changes and other non-content events.
/// Returns `true` for:
/// - File creates
/// - File deletes
/// - File renames
/// - File content modifications (data changes)
///
/// Returns `false` for:
/// - Metadata changes (permissions, timestamps)
/// - Access events
/// - Other non-content events
pub fn is_relevant_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_)
            | EventKind::Remove(_)
            | EventKind::Modify(ModifyKind::Data(_))
            | EventKind::Modify(ModifyKind::Name(_))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind, RenameMode};

    #[test]
    fn test_create_is_relevant() {
        let event = Event::new(EventKind::Create(CreateKind::File));
        assert!(is_relevant_event(&event));
    }

    #[test]
    fn test_remove_is_relevant() {
        let event = Event::new(EventKind::Remove(RemoveKind::File));
        assert!(is_relevant_event(&event));
    }

    #[test]
    fn test_data_modify_is_relevant() {
        let event = Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content)));
        assert!(is_relevant_event(&event));
    }

    #[test]
    fn test_rename_is_relevant() {
        let event = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)));
        assert!(is_relevant_event(&event));
    }

    #[test]
    fn test_metadata_is_not_relevant() {
        let event = Event::new(EventKind::Modify(ModifyKind::Metadata(
            notify::event::MetadataKind::Permissions,
        )));
        assert!(!is_relevant_event(&event));
    }

    #[test]
    fn test_access_is_not_relevant() {
        let event = Event::new(EventKind::Access(notify::event::AccessKind::Read));
        assert!(!is_relevant_event(&event));
    }
}
