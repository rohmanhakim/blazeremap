// Controller detection and information extraction
use crate::device::controller::{
    ControllerCapability, ControllerInfo, get_known_vendor_database, identify_controller,
};
use evdev::{AttributeSetRef, Device, FFEffectType};

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
    use evdev::{AbsoluteAxisType, EventType};

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
            AbsoluteAxisType::ABS_X => has_gamepad_axis = true,
            AbsoluteAxisType::ABS_Y => has_gamepad_axis = true,
            AbsoluteAxisType::ABS_RX => has_gamepad_axis = true,
            AbsoluteAxisType::ABS_RY => has_gamepad_axis = true,
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

    let ff_effects: &AttributeSetRef<FFEffectType> = device.supported_ff().unwrap_or_default();
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

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[test]
    fn test_compare_with_and_without_name_filter() {
        use evdev::{AbsoluteAxisType, EventType, enumerate};

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
                    AbsoluteAxisType::ABS_X
                    | AbsoluteAxisType::ABS_Y
                    | AbsoluteAxisType::ABS_RX
                    | AbsoluteAxisType::ABS_RY => {
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
