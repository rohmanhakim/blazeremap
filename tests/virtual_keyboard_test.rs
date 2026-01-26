use blazeremap::event::KeyboardCode;
use blazeremap::output::keyboard::VirtualKeyboard;
use blazeremap::platform::linux::LinuxVirtualKeyboard;
use evdev::Device;
use std::thread;
use std::time::Duration;

/// Helper function to find a device by name in /dev/input
fn find_device_by_name(name: &str) -> Option<String> {
    for entry in std::fs::read_dir("/dev/input").ok()? {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.to_str()?.contains("event") {
            if let Ok(device) = Device::open(&path) {
                if let Some(device_name) = device.name() {
                    if device_name.contains(name) {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
}

#[test]
#[ignore] // Only run with: cargo test -- --ignored --include-ignored
fn test_virtual_keyboard_creation() {
    // This test requires root/uinput permissions
    let result = LinuxVirtualKeyboard::new("BlazeRemap Integration Test");

    assert!(result.is_ok(), "Failed to create virtual keyboard: {:?}", result.err());

    // Give udev time to register the device
    thread::sleep(Duration::from_millis(100));

    // Verify device appears in /dev/input
    let device_path = find_device_by_name("BlazeRemap Integration Test");
    assert!(device_path.is_some(), "Virtual keyboard not found in /dev/input");

    println!("✓ Virtual keyboard created at: {:?}", device_path);
}

#[test]
#[ignore]
fn test_virtual_keyboard_key_press_release() {
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap Key Test")
        .expect("Failed to create virtual keyboard");

    // Test press
    let result = keyboard.press_key(blazeremap::event::KeyboardCode::A);
    assert!(result.is_ok(), "Failed to press key: {:?}", result.err());

    thread::sleep(Duration::from_millis(50));

    // Test release
    let result = keyboard.release_key(blazeremap::event::KeyboardCode::A);
    assert!(result.is_ok(), "Failed to release key: {:?}", result.err());

    println!("✓ Key press/release successful");
}

#[test]
#[ignore]
fn test_virtual_keyboard_tap() {
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap Tap Test")
        .expect("Failed to create virtual keyboard");

    // Test tap (press + release)
    let result = keyboard.tap_key(blazeremap::event::KeyboardCode::Space);
    assert!(result.is_ok(), "Failed to tap key: {:?}", result.err());

    println!("✓ Key tap successful");
}

#[test]
#[ignore]
fn test_virtual_keyboard_multiple_keys() {
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap Multi Key Test")
        .expect("Failed to create virtual keyboard");

    let keys = [KeyboardCode::A, KeyboardCode::B, KeyboardCode::C, KeyboardCode::Space];

    for key in &keys {
        let result = keyboard.tap_key(key.clone());
        assert!(result.is_ok(), "Failed to tap {:?}: {:?}", key, result.err());
        thread::sleep(Duration::from_millis(10));
    }

    println!("✓ Multiple key taps successful");
}

#[test]
#[ignore]
fn test_virtual_keyboard_cleanup() {
    let device_name = "BlazeRemap Cleanup Test";

    // Create keyboard
    let keyboard =
        LinuxVirtualKeyboard::new(device_name).expect("Failed to create virtual keyboard");

    thread::sleep(Duration::from_millis(100));

    // Verify it exists
    let device_path = find_device_by_name(device_name);
    assert!(device_path.is_some(), "Device not found after creation");

    // Drop the keyboard (cleanup)
    drop(keyboard);

    // Give kernel time to remove device
    thread::sleep(Duration::from_millis(200));

    // Verify it's gone
    let device_path_after = find_device_by_name(device_name);
    assert!(device_path_after.is_none(), "Device still exists after cleanup");

    println!("✓ Virtual keyboard cleanup successful");
}

#[test]
#[ignore]
fn test_virtual_keyboard_sys_path() {
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap SysPath Test")
        .expect("Failed to create virtual keyboard");

    let sys_path = keyboard.sys_path();
    assert!(sys_path.is_ok(), "Failed to get sys_path: {:?}", sys_path.err());

    let path = sys_path.unwrap();
    assert!(path.starts_with("/sys/devices/virtual/input"));

    println!("✓ Sysfs path: {:?}", path);
}

#[test]
#[ignore]
fn test_virtual_keyboard_rapid_events() {
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap Rapid Test")
        .expect("Failed to create virtual keyboard");

    // Simulate rapid button mashing (100 taps)
    for _ in 0..100 {
        keyboard.tap_key(KeyboardCode::Space).expect("Failed during rapid tap test");
    }

    println!("✓ Rapid event test successful (100 taps)");
}
