use std::thread;
use std::time::Duration;
use ratatui_toolkit::toast::{Toast, ToastLevel, ToastManager};

/// Test Toast creation with different ToastLevel variants
#[test]
fn test_toast_creation_with_different_levels() {
    let info_toast = Toast::new("Info message", ToastLevel::Info);
    assert_eq!(info_toast.level, ToastLevel::Info);
    assert_eq!(info_toast.message, "Info message");
    assert_eq!(info_toast.duration, Duration::from_secs(3));

    let warning_toast = Toast::new("Warning message", ToastLevel::Warning);
    assert_eq!(warning_toast.level, ToastLevel::Warning);
    assert_eq!(warning_toast.message, "Warning message");

    let error_toast = Toast::new("Error message", ToastLevel::Error);
    assert_eq!(error_toast.level, ToastLevel::Error);
    assert_eq!(error_toast.message, "Error message");

    let success_toast = Toast::new("Success message", ToastLevel::Success);
    assert_eq!(success_toast.level, ToastLevel::Success);
    assert_eq!(success_toast.message, "Success message");
}

/// Test Toast creation with custom duration
#[test]
fn test_toast_with_custom_duration() {
    let custom_duration = Duration::from_millis(500);
    let toast = Toast::with_duration("Custom duration", ToastLevel::Info, custom_duration);

    assert_eq!(toast.message, "Custom duration");
    assert_eq!(toast.level, ToastLevel::Info);
    assert_eq!(toast.duration, custom_duration);
}

/// Test toast with zero duration
#[test]
fn test_toast_with_zero_duration() {
    let zero_duration = Duration::from_secs(0);
    let toast = Toast::with_duration("Instant toast", ToastLevel::Info, zero_duration);

    assert_eq!(toast.duration, zero_duration);
    // With zero duration, it should be expired immediately or very quickly
    // We need to give it a tiny bit of time for the instant comparison
    thread::sleep(Duration::from_millis(1));
    assert!(toast.is_expired());
}

/// Test toast with very short duration
#[test]
fn test_toast_with_short_duration() {
    let short_duration = Duration::from_millis(50);
    let toast = Toast::with_duration("Short toast", ToastLevel::Info, short_duration);

    // Should not be expired immediately
    assert!(!toast.is_expired());

    // Wait for expiration
    thread::sleep(Duration::from_millis(100));
    assert!(toast.is_expired());
}

/// Test toast expiration logic
#[test]
fn test_toast_expiration() {
    let toast = Toast::with_duration("Short lived", ToastLevel::Info, Duration::from_millis(100));

    // Should not be expired immediately after creation
    assert!(!toast.is_expired());

    // Wait for it to expire
    thread::sleep(Duration::from_millis(150));
    assert!(toast.is_expired());
}

/// Test toast lifetime_percent calculation
#[test]
fn test_toast_lifetime_percent() {
    let toast = Toast::with_duration("Test", ToastLevel::Info, Duration::from_millis(200));

    // Initially, lifetime should be close to 1.0 (100%)
    let initial_percent = toast.lifetime_percent();
    assert!(initial_percent > 0.9 && initial_percent <= 1.0);

    // Wait for half the duration
    thread::sleep(Duration::from_millis(100));
    let mid_percent = toast.lifetime_percent();
    assert!(
        mid_percent > 0.3 && mid_percent < 0.7,
        "Expected mid_percent to be around 0.5, got {}",
        mid_percent
    );

    // Wait until expired
    thread::sleep(Duration::from_millis(150));
    let expired_percent = toast.lifetime_percent();
    assert!(
        expired_percent <= 0.0,
        "Expected expired_percent to be <= 0, got {}",
        expired_percent
    );
}

/// Test ToastLevel color mapping
#[test]
fn test_toast_level_colors() {
    use ratatui::style::Color;

    assert_eq!(ToastLevel::Success.color(), Color::Green);
    assert_eq!(ToastLevel::Error.color(), Color::Red);
    assert_eq!(ToastLevel::Info.color(), Color::Cyan);
    assert_eq!(ToastLevel::Warning.color(), Color::Yellow);
}

/// Test ToastLevel icon mapping
#[test]
fn test_toast_level_icons() {
    assert_eq!(ToastLevel::Success.icon(), "✓");
    assert_eq!(ToastLevel::Error.icon(), "✗");
    assert_eq!(ToastLevel::Info.icon(), "ℹ");
    assert_eq!(ToastLevel::Warning.icon(), "⚠");
}

/// Test ToastManager initialization
#[test]
fn test_toast_manager_initialization() {
    let manager = ToastManager::new();
    assert_eq!(manager.get_active().len(), 0);
    assert!(!manager.has_toasts());
}

/// Test ToastManager default initialization
#[test]
fn test_toast_manager_default() {
    let manager = ToastManager::default();
    assert_eq!(manager.get_active().len(), 0);
    assert!(!manager.has_toasts());
}

/// Test adding toasts to manager
#[test]
fn test_add_toast_to_manager() {
    let mut manager = ToastManager::new();

    let toast = Toast::new("Test toast", ToastLevel::Info);
    manager.add(toast);

    assert_eq!(manager.get_active().len(), 1);
    assert!(manager.has_toasts());
    assert_eq!(manager.get_active()[0].message, "Test toast");
}

/// Test adding multiple toasts
#[test]
fn test_add_multiple_toasts() {
    let mut manager = ToastManager::new();

    manager.add(Toast::new("First", ToastLevel::Info));
    manager.add(Toast::new("Second", ToastLevel::Warning));
    manager.add(Toast::new("Third", ToastLevel::Error));

    assert_eq!(manager.get_active().len(), 3);
    assert_eq!(manager.get_active()[0].message, "First");
    assert_eq!(manager.get_active()[1].message, "Second");
    assert_eq!(manager.get_active()[2].message, "Third");
}

/// Test toast ordering (FIFO - First In First Out)
#[test]
fn test_toast_ordering() {
    let mut manager = ToastManager::new();

    manager.add(Toast::new("Toast 1", ToastLevel::Info));
    manager.add(Toast::new("Toast 2", ToastLevel::Info));
    manager.add(Toast::new("Toast 3", ToastLevel::Info));

    let toasts = manager.get_active();
    assert_eq!(toasts.len(), 3);
    assert_eq!(toasts[0].message, "Toast 1");
    assert_eq!(toasts[1].message, "Toast 2");
    assert_eq!(toasts[2].message, "Toast 3");
}

/// Test maximum toast count limits
#[test]
fn test_max_toast_limit() {
    let mut manager = ToastManager::new();

    // Add more than the maximum (5) toasts
    for i in 1..=10 {
        manager.add(Toast::new(format!("Toast {}", i), ToastLevel::Info));
    }

    // Should only have the most recent 5 toasts
    let toasts = manager.get_active();
    assert_eq!(toasts.len(), 5);

    // Verify these are the most recent ones (6-10)
    assert_eq!(toasts[0].message, "Toast 6");
    assert_eq!(toasts[1].message, "Toast 7");
    assert_eq!(toasts[2].message, "Toast 8");
    assert_eq!(toasts[3].message, "Toast 9");
    assert_eq!(toasts[4].message, "Toast 10");
}

/// Test removing expired toasts
#[test]
fn test_remove_expired_toasts() {
    let mut manager = ToastManager::new();

    // Add a toast with short duration
    manager.add(Toast::with_duration(
        "Short",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));

    // Add a toast with longer duration
    manager.add(Toast::with_duration(
        "Long",
        ToastLevel::Info,
        Duration::from_secs(10),
    ));

    assert_eq!(manager.get_active().len(), 2);

    // Wait for the short toast to expire
    thread::sleep(Duration::from_millis(100));

    // Remove expired toasts
    manager.remove_expired();

    // Should only have the long-lived toast
    assert_eq!(manager.get_active().len(), 1);
    assert_eq!(manager.get_active()[0].message, "Long");
}

/// Test automatic removal of expired toasts when adding new ones
#[test]
fn test_auto_remove_expired_on_add() {
    let mut manager = ToastManager::new();

    // Add expired toasts
    manager.add(Toast::with_duration(
        "Expired 1",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));
    manager.add(Toast::with_duration(
        "Expired 2",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));

    // Wait for expiration
    thread::sleep(Duration::from_millis(100));

    // Add a new toast - this should trigger cleanup
    manager.add(Toast::new("New toast", ToastLevel::Success));

    // Should only have the new toast
    assert_eq!(manager.get_active().len(), 1);
    assert_eq!(manager.get_active()[0].message, "New toast");
}

/// Test concurrent toasts with different durations
#[test]
fn test_concurrent_toasts_different_durations() {
    let mut manager = ToastManager::new();

    // Add toasts with different durations at roughly the same time
    manager.add(Toast::with_duration(
        "50ms",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));
    manager.add(Toast::with_duration(
        "100ms",
        ToastLevel::Warning,
        Duration::from_millis(100),
    ));
    manager.add(Toast::with_duration(
        "150ms",
        ToastLevel::Error,
        Duration::from_millis(150),
    ));

    assert_eq!(manager.get_active().len(), 3);

    // After 75ms, first should be expired
    thread::sleep(Duration::from_millis(75));
    manager.remove_expired();
    assert_eq!(manager.get_active().len(), 2);
    assert_eq!(manager.get_active()[0].message, "100ms");
    assert_eq!(manager.get_active()[1].message, "150ms");

    // After another 50ms (125ms total), second should be expired
    thread::sleep(Duration::from_millis(50));
    manager.remove_expired();
    assert_eq!(manager.get_active().len(), 1);
    assert_eq!(manager.get_active()[0].message, "150ms");

    // After another 50ms (175ms total), all should be expired
    thread::sleep(Duration::from_millis(50));
    manager.remove_expired();
    assert_eq!(manager.get_active().len(), 0);
}

/// Test clearing all toasts
#[test]
fn test_clear_all_toasts() {
    let mut manager = ToastManager::new();

    manager.add(Toast::new("Toast 1", ToastLevel::Info));
    manager.add(Toast::new("Toast 2", ToastLevel::Warning));
    manager.add(Toast::new("Toast 3", ToastLevel::Error));

    assert_eq!(manager.get_active().len(), 3);

    manager.clear();

    assert_eq!(manager.get_active().len(), 0);
    assert!(!manager.has_toasts());
}

/// Test convenience methods for different toast levels
#[test]
fn test_convenience_methods() {
    let mut manager = ToastManager::new();

    manager.success("Success message");
    manager.error("Error message");
    manager.info("Info message");
    manager.warning("Warning message");

    let toasts = manager.get_active();
    assert_eq!(toasts.len(), 4);

    assert_eq!(toasts[0].message, "Success message");
    assert_eq!(toasts[0].level, ToastLevel::Success);

    assert_eq!(toasts[1].message, "Error message");
    assert_eq!(toasts[1].level, ToastLevel::Error);

    assert_eq!(toasts[2].message, "Info message");
    assert_eq!(toasts[2].level, ToastLevel::Info);

    assert_eq!(toasts[3].message, "Warning message");
    assert_eq!(toasts[3].level, ToastLevel::Warning);
}

/// Test complete lifecycle: create → add → expire → remove
#[test]
fn test_complete_lifecycle() {
    let mut manager = ToastManager::new();

    // Create toast
    let toast = Toast::with_duration(
        "Lifecycle test",
        ToastLevel::Info,
        Duration::from_millis(100),
    );

    // Verify initial state
    assert!(!toast.is_expired());

    // Add to manager
    manager.add(toast);
    assert_eq!(manager.get_active().len(), 1);
    assert!(manager.has_toasts());

    // Toast is still active
    assert!(!manager.get_active()[0].is_expired());

    // Wait for expiration
    thread::sleep(Duration::from_millis(150));

    // Toast should be expired but still in manager until cleanup
    assert!(manager.get_active()[0].is_expired());

    // Remove expired
    manager.remove_expired();

    // Verify removed
    assert_eq!(manager.get_active().len(), 0);
    assert!(!manager.has_toasts());
}

/// Test that max limit keeps most recent toasts
#[test]
fn test_max_limit_keeps_most_recent() {
    let mut manager = ToastManager::new();

    // Add exactly max toasts
    for i in 1..=5 {
        manager.add(Toast::new(format!("Toast {}", i), ToastLevel::Info));
    }
    assert_eq!(manager.get_active().len(), 5);

    // Add one more
    manager.add(Toast::new("Toast 6", ToastLevel::Info));

    // Should still be 5, with oldest removed
    assert_eq!(manager.get_active().len(), 5);
    assert_eq!(manager.get_active()[0].message, "Toast 2");
    assert_eq!(manager.get_active()[4].message, "Toast 6");
}

/// Test empty manager operations
#[test]
fn test_empty_manager_operations() {
    let mut manager = ToastManager::new();

    // Operations on empty manager should be safe
    manager.remove_expired();
    assert_eq!(manager.get_active().len(), 0);

    manager.clear();
    assert_eq!(manager.get_active().len(), 0);

    assert!(!manager.has_toasts());
}

/// Test toast message can be String or &str
#[test]
fn test_toast_message_types() {
    let mut manager = ToastManager::new();

    // Test with &str
    manager.add(Toast::new("str message", ToastLevel::Info));

    // Test with String
    let owned = String::from("owned message");
    manager.add(Toast::new(owned, ToastLevel::Info));

    // Test with format!
    manager.add(Toast::new(format!("formatted {}", 123), ToastLevel::Info));

    let toasts = manager.get_active();
    assert_eq!(toasts[0].message, "str message");
    assert_eq!(toasts[1].message, "owned message");
    assert_eq!(toasts[2].message, "formatted 123");
}

/// Test toast with very long duration
#[test]
fn test_toast_with_long_duration() {
    let long_duration = Duration::from_secs(3600); // 1 hour
    let toast = Toast::with_duration("Long lived", ToastLevel::Info, long_duration);

    assert_eq!(toast.duration, long_duration);
    assert!(!toast.is_expired());

    // Lifetime should be very close to 1.0
    let lifetime = toast.lifetime_percent();
    assert!(lifetime > 0.99);
}

/// Test multiple expired toasts removed at once
#[test]
fn test_multiple_expired_removed() {
    let mut manager = ToastManager::new();

    // Add multiple short-lived toasts
    for i in 1..=5 {
        manager.add(Toast::with_duration(
            format!("Toast {}", i),
            ToastLevel::Info,
            Duration::from_millis(50),
        ));
    }

    assert_eq!(manager.get_active().len(), 5);

    // Wait for all to expire
    thread::sleep(Duration::from_millis(100));

    // Remove all expired at once
    manager.remove_expired();

    assert_eq!(manager.get_active().len(), 0);
}

/// Test toast styling based on level
#[test]
fn test_toast_styling_by_level() {
    use ratatui::style::Color;

    let levels = vec![
        (ToastLevel::Info, Color::Cyan, "ℹ"),
        (ToastLevel::Warning, Color::Yellow, "⚠"),
        (ToastLevel::Error, Color::Red, "✗"),
        (ToastLevel::Success, Color::Green, "✓"),
    ];

    for (level, expected_color, expected_icon) in levels {
        assert_eq!(level.color(), expected_color);
        assert_eq!(level.icon(), expected_icon);
    }
}

/// Test has_toasts accuracy
#[test]
fn test_has_toasts_accuracy() {
    let mut manager = ToastManager::new();

    // Empty manager
    assert!(!manager.has_toasts());

    // Add a toast
    manager.add(Toast::new("Test", ToastLevel::Info));
    assert!(manager.has_toasts());

    // Clear
    manager.clear();
    assert!(!manager.has_toasts());

    // Add and let expire
    manager.add(Toast::with_duration(
        "Quick",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));
    assert!(manager.has_toasts());

    thread::sleep(Duration::from_millis(100));
    manager.remove_expired();
    assert!(!manager.has_toasts());
}

/// Test ToastLevel equality
#[test]
fn test_toast_level_equality() {
    assert_eq!(ToastLevel::Info, ToastLevel::Info);
    assert_eq!(ToastLevel::Warning, ToastLevel::Warning);
    assert_eq!(ToastLevel::Error, ToastLevel::Error);
    assert_eq!(ToastLevel::Success, ToastLevel::Success);

    assert_ne!(ToastLevel::Info, ToastLevel::Warning);
    assert_ne!(ToastLevel::Error, ToastLevel::Success);
}

/// Test edge case: adding max+1 toasts with one expired
#[test]
fn test_max_limit_with_expired() {
    let mut manager = ToastManager::new();

    // Add one expired toast
    manager.add(Toast::with_duration(
        "Expired",
        ToastLevel::Info,
        Duration::from_millis(50),
    ));

    // Add max-1 long-lived toasts
    for i in 1..=4 {
        manager.add(Toast::new(format!("Toast {}", i), ToastLevel::Info));
    }

    thread::sleep(Duration::from_millis(100));

    // Add one more - should trigger expired removal
    manager.add(Toast::new("New", ToastLevel::Info));

    // Should have 5 toasts (expired one removed, new one added)
    assert_eq!(manager.get_active().len(), 5);

    // First should be "Toast 1", not "Expired"
    assert_eq!(manager.get_active()[0].message, "Toast 1");
}
