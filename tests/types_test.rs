// Integration test to verify type definitions work correctly
// This file goes in tests/types_test.rs

use blazeremap::device::controller::{ControllerCapability, ControllerType, identify_controller};

#[test]
fn test_controller_type_display() {
    assert_eq!(ControllerType::XboxOne.to_string(), "Xbox One");
    assert_eq!(ControllerType::XboxSeries.to_string(), "Xbox Series X/S");
    assert_eq!(ControllerType::DualShock4.to_string(), "DualShock 4");
}

#[test]
fn test_capability_display() {
    assert_eq!(ControllerCapability::ForceFeedback.to_string(), "Force Feedback");
    assert_eq!(ControllerCapability::ElitePaddles.to_string(), "Elite Paddles");
}

#[test]
fn test_identify_xbox_one_controller() {
    // Your Xbox One S Controller (Bluetooth) - vendor 045e, product 02fd
    let controller_type = identify_controller(0x045e, 0x02fd);
    assert_eq!(controller_type, ControllerType::XboxOne);
}

#[test]
fn test_identify_dualshock4() {
    // DualShock 4 Gen 2
    let controller_type = identify_controller(0x054c, 0x09cc);
    assert_eq!(controller_type, ControllerType::DualShock4);
}

#[test]
fn test_identify_unknown_controller() {
    let controller_type = identify_controller(0xFFFF, 0xFFFF);
    assert_eq!(controller_type, ControllerType::Generic);
}

#[test]
fn test_controller_type_equality() {
    assert_eq!(ControllerType::XboxOne, ControllerType::XboxOne);
    assert_ne!(ControllerType::XboxOne, ControllerType::DualShock4);
}
