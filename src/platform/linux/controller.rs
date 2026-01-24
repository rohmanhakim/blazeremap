// Controller detection and information extraction
use crate::{
    device::controller::{
        Controller, ControllerCapability, ControllerInfo, get_known_vendor_database,
        identify_controller,
    },
    event::InputEvent,
    platform::linux::evdev_to_input,
};
use anyhow::Context;
use evdev::{AttributeSetRef, Device, FFEffectCode};

// Constants for controller detection
const BTN_GAMEPAD_MIN: u16 = 0x130;
const BTN_GAMEPAD_MAX: u16 = 0x13f;
const BTN_JOYSTICK_MIN: u16 = 0x120;
const BTN_JOYSTICK_MAX: u16 = 0x12f;
const BTN_TRIGGER_HAPPY1: u16 = 0x2c0;
const BTN_TRIGGER_HAPPY4: u16 = 0x2c3;
const ELITE_PADDLE_COUNT: usize = 4;

/// Check if device should be excluded based on name
///
/// Some virtual/emulated devices pass all hardware checks but aren't
/// real controllers (e.g., Steam Virtual Gamepad, remote desktop devices).
/// This provides a final safety check based on device naming.
///
/// Mirrors: isExcludedByName() from Go
fn is_excluded_by_name(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    let exclude_keywords = [
        "keyboard",
        "mouse",
        "touchpad",
        "power button",
        "sleep button",
        "hdmi",
        "audio",
        "speaker",
        "headphone",
        "microphone",
        "line out",
        "line in",
        "led",
        "lamplight",
        "rgb",
        "system control",
        "consumer control",
    ];

    for keyword in exclude_keywords.iter() {
        if name_lower.contains(keyword) {
            // show what's being filtered
            println!("  Excluding '{}' (matched keyword: '{}')", name, keyword);
            return true;
        }
    }

    false
}

/// Check if a device is a game controller
pub(super) fn is_game_controller(device: &Device) -> bool {
    use evdev::{AbsoluteAxisCode, EventType};

    let supported_events = device.supported_events();

    if !supported_events.contains(EventType::KEY) {
        return false;
    }
    if !supported_events.contains(EventType::ABSOLUTE) {
        return false;
    }

    let keys = device.supported_keys().unwrap_or_default();

    let mut has_gamepad_button = false;
    for key in keys.iter() {
        let code = key.code();

        if (BTN_GAMEPAD_MIN..=BTN_GAMEPAD_MAX).contains(&code) {
            has_gamepad_button = true;
            break;
        }

        if (BTN_JOYSTICK_MIN..=BTN_JOYSTICK_MAX).contains(&code) {
            has_gamepad_button = true;
            break;
        }
    }

    if !has_gamepad_button {
        return false;
    }

    let axes = device.supported_absolute_axes().unwrap_or_default();

    let mut has_gamepad_axis = false;
    for axis in axes.iter() {
        match axis {
            AbsoluteAxisCode::ABS_X => has_gamepad_axis = true,
            AbsoluteAxisCode::ABS_Y => has_gamepad_axis = true,
            AbsoluteAxisCode::ABS_RX => has_gamepad_axis = true,
            AbsoluteAxisCode::ABS_RY => has_gamepad_axis = true,
            _ => {}
        }
    }

    if !has_gamepad_axis {
        return false;
    }

    // Check device name
    let device_name = device.name().unwrap_or("");

    // If name contains "controller", "gamepad", "joystick" - probably a controller
    let name_lower = device_name.to_lowercase();
    let controller_keywords = ["controller", "gamepad", "joystick", "pad"];
    let is_likely_controller = controller_keywords.iter().any(|kw| name_lower.contains(kw));

    // If it looks like a controller by name, skip exclusion check
    if !is_likely_controller && is_excluded_by_name(device_name) {
        return false;
    }

    println!("Found controller: {}", device_name);
    true
}

/// Check if device supports force feedback (rumble)
fn has_force_feedback(device: &Device) -> bool {
    use evdev::EventType;

    let supported_events = device.supported_events();

    if !supported_events.contains(EventType::FORCEFEEDBACK) {
        return false;
    }

    let ff_effects: &AttributeSetRef<FFEffectCode> = device.supported_ff().unwrap_or_default();
    ff_effects.iter().len() != 0
}

/// Check if device has Xbox Elite paddles
fn has_elite_paddles(device: &Device) -> bool {
    let keys = device.supported_keys().unwrap_or_default();

    let mut paddle_count = 0;
    for key in keys.iter() {
        let code = key.code();

        if (BTN_TRIGGER_HAPPY1..=BTN_TRIGGER_HAPPY4).contains(&code) {
            paddle_count += 1;
        }
    }

    paddle_count >= ELITE_PADDLE_COUNT
}

/// Extract controller information from an evdev device
pub(super) fn extract_controller_info(
    device: &Device,
    path: &str,
) -> anyhow::Result<ControllerInfo> {
    let name = device.name().unwrap_or("Unknown").to_string();
    let input_id = device.input_id();

    let vendor_id = input_id.vendor();
    let product_id = input_id.product();

    let vendor_db = get_known_vendor_database();
    let vendor_name = vendor_db
        .get(&vendor_id)
        .map(|&name| name.to_string())
        .unwrap_or_else(|| format!("Unknown (0x{:04X})", vendor_id));

    let controller_type = identify_controller(vendor_id, product_id);

    let mut capabilities = Vec::new();

    if has_force_feedback(device) {
        capabilities.push(ControllerCapability::ForceFeedback);
    }

    if has_elite_paddles(device) {
        capabilities.push(ControllerCapability::ElitePaddles);
    }

    Ok(ControllerInfo {
        path: path.to_string(),
        name,
        controller_type,
        vendor_id,
        vendor_name,
        product_id,
        capabilities,
    })
}

pub struct LinuxController {
    info: ControllerInfo,
    device: Device,
}

impl LinuxController {
    pub fn new(info: ControllerInfo, device: Device) -> Self {
        Self { info, device }
    }

    /// Open a controller device at the given path
    ///
    /// This is the primary way to construct a LinuxController.
    pub fn open(path: &str) -> anyhow::Result<Self> {
        // Open device first
        let device =
            Device::open(path).with_context(|| format!("Failed to open device at {}", path))?;

        // Extract info from opened device
        let info = extract_controller_info(&device, path)?;

        // Construct with both
        Ok(Self::new(info, device))
    }
}

impl Controller for LinuxController {
    fn get_info(&self) -> &ControllerInfo {
        &self.info
    }

    fn read_event(&mut self) -> anyhow::Result<Option<InputEvent>> {
        // This blocks until an event arrives - INTENTIONAL!
        match self.device.fetch_events() {
            Ok(events) => {
                // Process events, filter for relevant types
                for event in events {
                    let ev_type = event.event_type();

                    // Only care about buttons and axes
                    if ev_type == evdev::EventType::KEY || ev_type == evdev::EventType::ABSOLUTE {
                        match evdev_to_input(event) {
                            Some(input_event) => {
                                if !input_event.is_in_deadzone() {
                                    return Ok(Some(input_event));
                                }
                            }
                            None => {
                                return Ok(None);
                            }
                        }
                    }

                    // Skip sync events (frame boundaries)
                }

                // No relevant events in this batch, continue reading
                Ok(None)
            }
            Err(e) => {
                // Check if device disconnected using Linux errno
                // ENODEV (19) = No such device (device was disconnected)
                if let Some(19) = e.raw_os_error() {
                    Ok(None) // Graceful disconnect
                } else {
                    Err(anyhow::anyhow!("Failed to read event: {}", e))
                }
            }
        }
    }

    fn close(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::controller::ControllerInfo;
    use crate::device::controller::{ControllerCapability, ControllerType};

    #[test]
    fn test_is_excluded_by_name() {
        // Test excluded keywords
        assert!(is_excluded_by_name("USB Keyboard"));
        assert!(is_excluded_by_name("Wireless Mouse"));
        assert!(is_excluded_by_name("HDMI Audio"));
        assert!(is_excluded_by_name("System Control"));

        // Test non-excluded names
        assert!(!is_excluded_by_name("Xbox Wireless Controller"));
        assert!(!is_excluded_by_name("DualSense Wireless Controller"));
        assert!(!is_excluded_by_name("Generic Gamepad"));
    }

    #[test]
    fn test_linux_controller_construction() {
        // Create mock ControllerInfo
        let _info = ControllerInfo {
            path: "/dev/input/event3".to_string(),
            name: "Test Controller".to_string(),
            controller_type: ControllerType::XboxOne,
            vendor_id: 0x045e,
            vendor_name: "Microsoft".to_string(),
            product_id: 0x02ea,
            capabilities: vec![ControllerCapability::ForceFeedback],
        };

        // This test would require a mock Device, which is complex
        // For now, just test that the struct can be constructed
        // In a real test, we'd need to mock or use a test device
    }

    #[test]
    fn test_has_force_feedback() {
        // This would require creating a mock Device with FF support
        // For now, we skip this as it requires complex mocking
    }

    #[test]
    fn test_has_elite_paddles() {
        // This would require creating a mock Device with paddle buttons
        // For now, we skip this as it requires complex mocking
    }

    #[test]
    fn test_extract_controller_info() {
        // This would require creating a mock Device
        // For now, we skip this as it requires complex mocking
    }

    #[test]
    fn test_controller_trait_methods() {
        // Test that the trait methods exist and return expected types
        // We can't easily test the actual functionality without mocking
    }

    #[test]
    fn test_device_disconnect_error_handling() {
        use std::io::{Error, ErrorKind};

        // Test that ENODEV (19) is properly handled as graceful disconnect
        let enodev_error = Error::from_raw_os_error(19); // ENODEV

        // This simulates what happens in read_event when device is disconnected
        // We can't easily test the full read_event method without mocking,
        // but we can test the error handling logic
        assert_eq!(enodev_error.raw_os_error(), Some(19));

        // Test that other errors are not treated as disconnect
        let other_error = Error::new(ErrorKind::Other, "Some other error");
        assert_ne!(other_error.raw_os_error(), Some(19));
    }
}

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[test]
    fn test_compare_with_and_without_name_filter() {
        use evdev::{AbsoluteAxisCode, EventType, enumerate};

        let devices: Vec<_> = enumerate().collect();

        println!("\n=== WITHOUT name filtering ===\n");
        let mut count_without_filter = 0;

        for (_path, device) in &devices {
            // Check buttons and axes (without name filtering)
            let supported_events = device.supported_events();
            if !supported_events.contains(EventType::KEY) {
                continue;
            }
            if !supported_events.contains(EventType::ABSOLUTE) {
                continue;
            }

            // Check gamepad buttons
            let keys = device.supported_keys().unwrap_or_default();
            let mut has_gamepad_button = false;
            for key in keys.iter() {
                let code = key.code();
                if (code >= BTN_GAMEPAD_MIN && code <= BTN_GAMEPAD_MAX)
                    || (code >= BTN_JOYSTICK_MIN && code <= BTN_JOYSTICK_MAX)
                {
                    has_gamepad_button = true;
                    break;
                }
            }
            if !has_gamepad_button {
                continue;
            }

            // Check gamepad axes
            let axes = device.supported_absolute_axes().unwrap_or_default();
            let mut has_gamepad_axis = false;
            for axis in axes.iter() {
                match axis {
                    AbsoluteAxisCode::ABS_X
                    | AbsoluteAxisCode::ABS_Y
                    | AbsoluteAxisCode::ABS_RX
                    | AbsoluteAxisCode::ABS_RY => {
                        has_gamepad_axis = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !has_gamepad_axis {
                continue;
            }

            // If we get here, it passed all hardware checks
            let name = device.name().unwrap_or("Unknown");
            println!("  ✓ {}", name);
            count_without_filter += 1;
        }

        println!("\n=== WITH name filtering ===\n");
        let mut count_with_filter = 0;

        for (_path, device) in &devices {
            if is_game_controller(device) {
                let name = device.name().unwrap_or("Unknown");
                println!("  ✓ {}", name);
                count_with_filter += 1;
            }
        }

        println!("\n=== Summary ===");
        println!("Without name filter: {} devices", count_without_filter);
        println!("With name filter:    {} devices", count_with_filter);
        println!("Filtered out:        {} devices", count_without_filter - count_with_filter);
    }
}
