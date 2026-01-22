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

use blazeremap::device::ControllerType;
use blazeremap::platform;

/// Test that we can detect at least one controller
///
/// Prerequisites:
/// - At least one controller must be connected
/// - User must have permissions to read /dev/input
#[test]
#[ignore] // Only run when explicitly requested
fn test_detect_real_controller() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    // Should find at least one controller
    assert!(
        !result.controller_info.is_empty(),
        "No controllers detected! Make sure a controller is connected."
    );

    // Print what we found
    println!("\nDetected controllers:");
    for (i, info) in result.controller_info.iter().enumerate() {
        println!("  [{}] {} - {}", i, info.name, info.controller_type);
    }
}

/// Test that detected controllers have valid data
#[test]
#[ignore]
fn test_controller_info_validity() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    assert!(!result.controller_info.is_empty(), "No controllers detected for validation test");

    for info in &result.controller_info {
        // Name should not be empty
        assert!(!info.name.is_empty(), "Controller name is empty");

        // Path should be a valid /dev/input path
        assert!(info.path.starts_with("/dev/input/"), "Invalid device path: {}", info.path);

        // Vendor ID should not be zero (unlikely for real hardware)
        assert_ne!(info.vendor_id, 0, "Vendor ID is zero");

        // Controller type should not be Unknown
        assert_ne!(
            info.controller_type,
            ControllerType::Unknown,
            "Controller type is Unknown for {}",
            info.name
        );

        println!("✓ Valid: {} ({:04X}:{:04X})", info.name, info.vendor_id, info.product_id);
    }
}

/// Test that we don't detect keyboards or mice as controllers
#[test]
#[ignore]
fn test_no_false_positives() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    // Check that no detected device has keyboard/mouse-like names
    for info in &result.controller_info {
        let name_lower = info.name.to_lowercase();

        // These should never appear
        assert!(
            !name_lower.contains("keyboard"),
            "False positive: keyboard detected as controller: {}",
            info.name
        );
        assert!(
            !name_lower.contains("mouse")
                || name_lower.contains("controller")
                || name_lower.contains("gamepad"),
            "False positive: mouse detected as controller: {}",
            info.name
        );
        assert!(
            !name_lower.contains("power button"),
            "False positive: power button detected as controller: {}",
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
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    // Try to find a DualShock 4
    let ds4 = result
        .controller_info
        .iter()
        .find(|info| info.controller_type == ControllerType::DualShock4);

    if let Some(controller) = ds4 {
        println!("Found DualShock 4:");
        println!("  Name: {}", controller.name);
        println!("  Vendor: {} ({:04X})", controller.vendor_name, controller.vendor_id);
        println!("  Capabilities: {:?}", controller.capabilities);

        // DualShock 4 should have Sony vendor ID
        assert_eq!(controller.vendor_id, 0x054C, "DualShock 4 should have Sony vendor ID");

        // Should detect force feedback capability
        assert!(
            controller
                .capabilities
                .contains(&blazeremap::device::ControllerCapability::ForceFeedback),
            "DualShock 4 should have force feedback capability"
        );
    } else {
        println!("⚠ No DualShock 4 detected (test skipped)");
    }
}

/// Test Xbox controller detection
#[test]
#[ignore]
fn test_xbox_detection() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    // Try to find any Xbox controller
    let xbox = result.controller_info.iter().find(|info| {
        matches!(
            info.controller_type,
            ControllerType::XboxOne | ControllerType::XboxSeries | ControllerType::XboxElite
        )
    });

    if let Some(controller) = xbox {
        println!("Found Xbox controller:");
        println!("  Name: {}", controller.name);
        println!("  Type: {}", controller.controller_type);
        println!("  Capabilities: {:?}", controller.capabilities);

        // Xbox controllers should have Microsoft vendor ID
        assert_eq!(controller.vendor_id, 0x045E, "Xbox controller should have Microsoft vendor ID");
    } else {
        println!("⚠ No Xbox controller detected (test skipped)");
    }
}

/// Test Elite controller paddle detection
#[test]
#[ignore]
fn test_elite_paddle_detection() {
    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers().expect("Failed to list controllers");

    // Look for Elite controller
    let elite = result
        .controller_info
        .iter()
        .find(|info| info.controller_type == ControllerType::XboxElite);

    if let Some(controller) = elite {
        println!("Found Xbox Elite controller:");
        println!("  Capabilities: {:?}", controller.capabilities);

        // Elite should have paddle capability
        assert!(
            controller
                .capabilities
                .contains(&blazeremap::device::ControllerCapability::ElitePaddles),
            "Elite controller should have paddle capability"
        );
    } else {
        println!("⚠ No Xbox Elite controller detected (test skipped)");
    }
}

/// Test that controller detection is fast (< 1 second)
#[test]
#[ignore]
fn test_detection_performance() {
    use std::time::Instant;

    let device_manager = platform::new_device_manager();

    let start = Instant::now();
    let result = device_manager.list_controllers().expect("Failed to list controllers");
    let duration = start.elapsed();

    println!("Detection took: {:?}", duration);
    println!("Found {} controllers", result.controller_info.len());

    // Should complete in under 1 second
    assert!(duration.as_secs() < 1, "Detection took too long: {:?}", duration);
}

/// Test that we can detect controllers multiple times reliably
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
        let result = device_manager.list_controllers().expect("Failed to list controllers");
        let duration = start.elapsed();

        durations.push(duration);

        assert!(!result.controller_info.is_empty(), "No controllers on iteration {}", i);

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
