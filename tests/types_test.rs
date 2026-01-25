// Integration test to verify type definitions work correctly
// This file goes in tests/types_test.rs

use blazeremap::input::gamepad::{GamepadCapability, GamepadType, identify_gamepad};

#[test]
fn test_gamepad_type_display() {
    assert_eq!(GamepadType::XboxOne.to_string(), "Xbox One");
    assert_eq!(GamepadType::XboxSeries.to_string(), "Xbox Series X/S");
    assert_eq!(GamepadType::DualShock4.to_string(), "DualShock 4");
}

#[test]
fn test_capability_display() {
    assert_eq!(GamepadCapability::ForceFeedback.to_string(), "Force Feedback");
    assert_eq!(GamepadCapability::ElitePaddles.to_string(), "Elite Paddles");
}

#[test]
fn test_identify_xbox_one_controller() {
    // Your Xbox One S Controller (Bluetooth) - vendor 045e, product 02fd
    let gamepad_type = identify_gamepad(0x045e, 0x02fd);
    assert_eq!(gamepad_type, GamepadType::XboxOne);
}

#[test]
fn test_identify_dualshock4() {
    // DualShock 4 Gen 2
    let gamepad_type = identify_gamepad(0x054c, 0x09cc);
    assert_eq!(gamepad_type, GamepadType::DualShock4);
}

#[test]
fn test_identify_unknown_gamepad() {
    let gamepad_type = identify_gamepad(0xFFFF, 0xFFFF);
    assert_eq!(gamepad_type, GamepadType::Generic);
}

#[test]
fn test_gamepad_type_equality() {
    assert_eq!(GamepadType::XboxOne, GamepadType::XboxOne);
    assert_ne!(GamepadType::XboxOne, GamepadType::DualShock4);
}
