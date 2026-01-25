//! Hardware integration tests
//!
//! These tests require actual hardware connected and may need root permissions.
//!
//! Run with: cargo test --test hardware_test -- --nocapture --ignored
//!
//! The `--ignored` flag is used because these tests:
//! - Require physical hardware
//! - Need root/input group permissions
//! - Can't run in CI/CD environments

use blazeremap::input::GamepadType;
use blazeremap::platform;

/// Test that we can detect at least one gamepad
///
/// Prerequisites:
/// - At least one gamepad must be connected
/// - User must have permissions to read /dev/input
#[test]
#[ignore] // Only run when explicitly requested
fn test_detect_real_gamepad() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    // Should find at least one gamepad
    assert!(
        !result.gamepad_info.is_empty(),
        "No gamepads detected! Make sure a gamepad is connected."
    );

    // Print what we found
    println!("\nDetected gamepads:");
    for (i, info) in result.gamepad_info.iter().enumerate() {
        println!("  [{}] {} - {}", i, info.name, info.gamepad_type);
    }
}

/// Test that detected gamepads have valid data
#[test]
#[ignore]
fn test_gamepad_info_validity() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    assert!(!result.gamepad_info.is_empty(), "No gamepads detected for validation test");

    for info in &result.gamepad_info {
        // Name should not be empty
        assert!(!info.name.is_empty(), "Gamepad name is empty");

        // Path should be a valid /dev/input path
        assert!(info.path.starts_with("/dev/input/"), "Invalid device path: {}", info.path);

        // Vendor ID should not be zero (unlikely for real hardware)
        assert_ne!(info.vendor_id, 0, "Vendor ID is zero");

        // Gamepad type should not be Unknown
        assert_ne!(
            info.gamepad_type,
            GamepadType::Unknown,
            "Gamepad type is Unknown for {}",
            info.name
        );

        println!("✓ Valid: {} ({:04X}:{:04X})", info.name, info.vendor_id, info.product_id);
    }
}

/// Test that we don't detect keyboards or mice as gamepads
#[test]
#[ignore]
fn test_no_false_positives() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    // Check that no detected device has keyboard/mouse-like names
    for info in &result.gamepad_info {
        let name_lower = info.name.to_lowercase();

        // These should never appear
        assert!(
            !name_lower.contains("keyboard"),
            "False positive: keyboard detected as gamepad: {}",
            info.name
        );
        assert!(
            !name_lower.contains("mouse")
                || name_lower.contains("gamepad")
                || name_lower.contains("gamepad"),
            "False positive: mouse detected as gamepad: {}",
            info.name
        );
        assert!(
            !name_lower.contains("power button"),
            "False positive: power button detected as gamepad: {}",
            info.name
        );
    }

    println!("✓ No false positives detected");
}

/// Test DualShock 4 detection
#[test]
#[ignore]
fn test_dualshock4_detection() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    // Try to find a DualShock 4
    let ds4 = result.gamepad_info.iter().find(|info| info.gamepad_type == GamepadType::DualShock4);

    if let Some(gamepad) = ds4 {
        println!("Found DualShock 4:");
        println!("  Name: {}", gamepad.name);
        println!("  Vendor: {} ({:04X})", gamepad.vendor_name, gamepad.vendor_id);
        println!("  Capabilities: {:?}", gamepad.capabilities);

        // DualShock 4 should have Sony vendor ID
        assert_eq!(gamepad.vendor_id, 0x054C, "DualShock 4 should have Sony vendor ID");

        // Should detect force feedback capability
        assert!(
            gamepad.capabilities.contains(&blazeremap::input::GamepadCapability::ForceFeedback),
            "DualShock 4 should have force feedback capability"
        );
    } else {
        println!("⚠ No DualShock 4 detected (test skipped)");
    }
}

/// Test Xbox gamepad detection
#[test]
#[ignore]
fn test_xbox_detection() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    // Try to find any Xbox gamepad
    let xbox = result.gamepad_info.iter().find(|info| {
        matches!(
            info.gamepad_type,
            GamepadType::XboxOne | GamepadType::XboxSeries | GamepadType::XboxElite
        )
    });

    if let Some(gamepad) = xbox {
        println!("Found Xbox gamepad:");
        println!("  Name: {}", gamepad.name);
        println!("  Type: {}", gamepad.gamepad_type);
        println!("  Capabilities: {:?}", gamepad.capabilities);

        // Xbox gamepads should have Microsoft vendor ID
        assert_eq!(gamepad.vendor_id, 0x045E, "Xbox gamepad should have Microsoft vendor ID");
    } else {
        println!("⚠ No Xbox gamepad detected (test skipped)");
    }
}

/// Test Elite gamepad paddle detection
#[test]
#[ignore]
fn test_elite_paddle_detection() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");

    // Look for Elite gamepad
    let elite = result.gamepad_info.iter().find(|info| info.gamepad_type == GamepadType::XboxElite);

    if let Some(gamepad) = elite {
        println!("Found Xbox Elite gamepad:");
        println!("  Capabilities: {:?}", gamepad.capabilities);

        // Elite should have paddle capability
        assert!(
            gamepad.capabilities.contains(&blazeremap::input::GamepadCapability::ElitePaddles),
            "Elite gamepad should have paddle capability"
        );
    } else {
        println!("⚠ No Xbox Elite gamepad detected (test skipped)");
    }
}

/// Test that gamepad detection is fast (< 1 second)
#[test]
#[ignore]
fn test_detection_performance() {
    use std::time::Instant;

    let device_manager = platform::new_device_manager();

    let start = Instant::now();
    let result = device_manager.list_gamepads().expect("Failed to list gamepads");
    let duration = start.elapsed();

    println!("Detection took: {:?}", duration);
    println!("Found {} gamepads", result.gamepad_info.len());

    // Should complete in under 1 second
    assert!(duration.as_secs() < 1, "Detection took too long: {:?}", duration);
}

/// Test that we can detect gamepads multiple times reliably
/// and measure performance characteristics
#[test]
#[ignore]
fn test_repeated_detection() {
    use std::time::Instant;

    let device_manager = platform::new_device_manager();

    let iterations = 10;
    let mut durations = Vec::new();

    println!("Running {} detection iterations...\n", iterations);

    for i in 0..iterations {
        let start = Instant::now();
        let result = device_manager.list_gamepads().expect("Failed to list gamepads");
        let duration = start.elapsed();

        durations.push(duration);

        assert!(!result.gamepad_info.is_empty(), "No gamepads on iteration {}", i);

        println!("  Iteration {}: {:?}", i + 1, duration);
    }

    // Calculate statistics
    let total: std::time::Duration = durations.iter().sum();
    let avg = total / iterations as u32;
    let min = durations.iter().min().unwrap();
    let max = durations.iter().max().unwrap();

    println!("\nPerformance Statistics:");
    println!("  Total:   {:?}", total);
    println!("  Average: {:?}", avg);
    println!("  Min:     {:?}", min);
    println!("  Max:     {:?}", max);

    // Sanity checks (not too strict)
    assert!(avg.as_secs() < 1, "Average detection unreasonably slow: {:?}", avg);
    assert!(max.as_secs() < 2, "Slowest detection unreasonably slow: {:?}", max);

    // Check consistency - max shouldn't be 10x slower than min
    let max_millis = max.as_millis();
    let min_millis = min.as_millis();
    if min_millis > 0 {
        let variance = max_millis / min_millis;
        println!("  Variance: {}x", variance);

        assert!(
            variance < 10,
            "Too much variance in detection times: min={:?}, max={:?}",
            min,
            max
        );
    }
}
