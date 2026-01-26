/*
 *  Event type definitions for gamepad input processing.

    This module defines the core event types used throughout BlazeRemap for
    representing gamepad inputs and their mapped outputs.

  # Event Flow
    Physical Gamepad → InputEvent → Mapping Engine → OutputEvent → Virtual Device
  # Types
    - [`InputEvent`]: Represents a raw input from a physical gamepad (button press,
        axis movement, etc.). Timestamps use [`Instant`] for monotonic, high-precision
        latency measurement.
    - [`OutputEvent`]: Represents a mapped output event (keyboard key, mouse button,
        etc.) that will be emitted to a virtual device.
    - [`EventType`]: Categorizes input events (Button, Axis, DPad, Sync).
    - [`OutputType`]: Categorizes output events (Keyboard, Mouse, Gamepad).
    - [`ButtonCode`]: Platform-agnostic gamepad button codes.
    - [`AxisCode`]: Platform-agnostic gamepad axis codes.

    # Timestamps
    All [`InputEvent`]s use [`Instant`] timestamps. For raw physical events, these are
    converted from kernel timestamps using a global anchor to preserve relative timing.
    This provides:
    - Monotonic timing (immune to system clock adjustments)

    - High precision for sub-millisecond latency measurement
    - Simple API for calculating elapsed time

    # Platform Abstraction
    The [`ButtonCode`] and [`AxisCode`] enums provide a platform-agnostic representation
    of gamepad inputs. They map platform-specific codes (like Linux evdev codes) to
    a common domain model that works across platforms and gamepad types.
*/

use std::{
    fmt::{Display, Formatter, Result},
    time::Instant,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)] // Copy for performance in event loops
pub enum InputEvent {
    Button {
        code: ButtonCode,
        pressed: bool, // true = press, false = release
        timestamp: Instant,
    },
    Axis {
        code: AxisCode,
        value: i32,
        timestamp: Instant,
    },
    Sync {
        timestamp: Instant,
    },
}

impl InputEvent {
    // For production code - captures current time
    pub fn button_press(button_code: ButtonCode) -> Self {
        Self::Button { code: button_code, pressed: true, timestamp: Instant::now() }
    }

    pub fn button_release(button_code: ButtonCode) -> Self {
        Self::Button { code: button_code, pressed: false, timestamp: Instant::now() }
    }

    pub fn axis_move(axis_code: AxisCode, value: i32) -> Self {
        Self::Axis { code: axis_code, value, timestamp: Instant::now() }
    }

    pub fn sync() -> Self {
        Self::Sync { timestamp: Instant::now() }
    }

    // Method to ignore for Sony DualShock4 analog sticks
    // implement a dead zone to ignore small movements near center
    pub fn is_in_deadzone(&self) -> bool {
        const ANALOG_CENTER: i32 = 128; // For 0-255 range
        const DEAD_ZONE: i32 = 10; // ±10 from center = ignore

        match self {
            Self::Axis { code, value, .. } => {
                // Don't apply deadzone to triggers (they have different ranges)
                if matches!(code, AxisCode::LeftTrigger | AxisCode::RightTrigger) {
                    return false;
                }

                let distance_from_center = (value - ANALOG_CENTER).abs();
                distance_from_center <= DEAD_ZONE
            }
            _ => false, // Only axis events can be in deadzone
        }
    }

    // For testing - allows providing a specific timestamp
    #[cfg(test)]
    pub fn button_press_at(button_code: ButtonCode, timestamp: Instant) -> Self {
        Self::Button { code: button_code, pressed: true, timestamp }
    }

    #[cfg(test)]
    pub fn button_release_at(button_code: ButtonCode, timestamp: Instant) -> Self {
        Self::Button { code: button_code, pressed: false, timestamp }
    }

    #[cfg(test)]
    pub fn axis_move_at(axis_code: AxisCode, value: i32, timestamp: Instant) -> Self {
        Self::Axis { code: axis_code, value, timestamp }
    }

    #[cfg(test)]
    pub fn sync_at(timestamp: Instant) -> Self {
        Self::Sync { timestamp }
    }

    pub fn is_button_pressed(&self) -> bool {
        matches!(self, Self::Button { pressed: true, .. })
    }

    pub fn is_button_released(&self) -> bool {
        matches!(self, Self::Button { pressed: false, .. })
    }

    pub fn is_axis_moved(&self) -> bool {
        matches!(self, Self::Axis { .. })
    }

    pub fn timestamp(&self) -> Instant {
        match self {
            Self::Button { timestamp, .. } => *timestamp,
            Self::Axis { timestamp, .. } => *timestamp,
            Self::Sync { timestamp } => *timestamp,
        }
    }
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Button { code, pressed, .. } => {
                write!(f, "{} ({})", code, if *pressed { "pressed" } else { "released" })
            }
            Self::Axis { code, value, .. } => {
                write!(f, "{}: {}", code, value)
            }
            Self::Sync { .. } => {
                write!(f, "Sync")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ButtonCode {
    South,
    East,
    North,
    West,
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,
    Select,
    Start,
    LeftStick,
    RightStick,
    Mode,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad,
    Unknown,
}

impl Display for ButtonCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::South => write!(f, "South"),
            Self::East => write!(f, "East"),
            Self::North => write!(f, "North"),
            Self::West => write!(f, "West"),
            Self::LeftShoulder => write!(f, "Left Shoulder"),
            Self::RightShoulder => write!(f, "Right Shoulder"),
            Self::LeftTrigger => write!(f, "Left Trigger"),
            Self::RightTrigger => write!(f, "Right Trigger"),
            Self::Select => write!(f, "Select"),
            Self::Start => write!(f, "Start"),
            Self::LeftStick => write!(f, "Left Stick"),
            Self::RightStick => write!(f, "Right Stick"),
            Self::Mode => write!(f, "Mode"),
            Self::Misc1 => write!(f, "Misc"),
            Self::Paddle1 => write!(f, "Paddle 1"),
            Self::Paddle2 => write!(f, "Paddle 2"),
            Self::Paddle3 => write!(f, "Paddle 3"),
            Self::Paddle4 => write!(f, "Paddle 4"),
            Self::Touchpad => write!(f, "Touchpad"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<&str> for ButtonCode {
    fn from(s: &str) -> Self {
        match s {
            "South" => ButtonCode::South,
            "East" => ButtonCode::East,
            "North" => ButtonCode::North,
            "West" => ButtonCode::West,
            "Left Shoulder" | "LeftShoulder" => ButtonCode::LeftShoulder,
            "Right Shoulder" | "RightShoulder" => ButtonCode::RightShoulder,
            "Left Trigger" | "LeftTrigger" => ButtonCode::LeftTrigger,
            "Right Trigger" | "RightTrigger" => ButtonCode::RightTrigger,
            "Select" => ButtonCode::Select,
            "Start" => ButtonCode::Start,
            "Left Stick" | "LeftStick" => ButtonCode::LeftStick,
            "Right Stick" | "RightStick" => ButtonCode::RightStick,
            "Mode" => ButtonCode::Mode,
            "Misc" | "Misc1" => ButtonCode::Misc1,
            "Paddle 1" | "Paddle1" => ButtonCode::Paddle1,
            "Paddle 2" | "Paddle2" => ButtonCode::Paddle2,
            "Paddle 3" | "Paddle3" => ButtonCode::Paddle3,
            "Paddle 4" | "Paddle4" => ButtonCode::Paddle4,
            "Touchpad" => ButtonCode::Touchpad,
            _ => ButtonCode::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisCode {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
    DPadX,
    DPadY,
    Unknown,
}

impl Display for AxisCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::LeftX => write!(f, "Left X"),
            Self::LeftY => write!(f, "Left Y"),
            Self::RightX => write!(f, "Right X"),
            Self::RightY => write!(f, "Right Y"),
            Self::LeftTrigger => write!(f, "Left Trigger"),
            Self::RightTrigger => write!(f, "Right Trigger"),
            Self::DPadX => write!(f, "DPad X"),
            Self::DPadY => write!(f, "DPad Y"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<&str> for AxisCode {
    fn from(s: &str) -> Self {
        match s {
            "LeftX" | "Left X" => AxisCode::LeftX,
            "LeftY" | "Left Y" => AxisCode::LeftY,
            "RightX" | "Right X" => AxisCode::RightX,
            "RightY" | "Right Y" => AxisCode::RightY,
            "LeftTrigger" | "Left Trigger" => AxisCode::LeftTrigger,
            "RightTrigger" | "Right Trigger" => AxisCode::RightTrigger,
            "DPadX" | "DPad X" => AxisCode::DPadX,
            "DPadY" | "DPad Y" => AxisCode::DPadY,
            _ => AxisCode::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisDirection {
    Positive, // Value > 0 (Down, Right)
    Negative, // Value < 0 (Up, Left)
}

impl Display for AxisDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Positive => write!(f, "Positive"),
            Self::Negative => write!(f, "Negative"),
        }
    }
}

pub fn axis_and_direction_to_string(axis_code: AxisCode, direction: AxisDirection) -> String {
    match axis_code {
        AxisCode::DPadX => match direction {
            AxisDirection::Negative => "DPad Left".to_string(),
            AxisDirection::Positive => "DPad Right".to_string(),
        },
        AxisCode::DPadY => match direction {
            AxisDirection::Negative => "DPad Up".to_string(),
            AxisDirection::Positive => "DPad Down".to_string(),
        },
        AxisCode::LeftX | AxisCode::RightX => match direction {
            AxisDirection::Negative => axis_code.to_string() + " Left",
            AxisDirection::Positive => axis_code.to_string() + " Right",
        },
        AxisCode::LeftY | AxisCode::RightY => match direction {
            AxisDirection::Negative => axis_code.to_string() + " Up",
            AxisDirection::Positive => axis_code.to_string() + " Down",
        },
        // For other AxisCode values like triggers or unknown, use their Display implementation
        _ => axis_code.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::event::InputEvent;

    use super::*;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_button_press_event() {
        let event = InputEvent::button_press(ButtonCode::South);
        assert!(event.is_button_pressed());
        assert!(!event.is_button_released());
        assert!(!event.is_axis_moved());
        assert!(!event.is_in_deadzone());
    }

    #[test]
    fn test_button_release_event() {
        let event = InputEvent::button_release(ButtonCode::South);
        assert!(!event.is_button_pressed());
        assert!(event.is_button_released());
        assert!(!event.is_axis_moved());
    }

    #[test]
    fn test_axis_event() {
        let event = InputEvent::axis_move(AxisCode::LeftX, 15234);
        assert!(!event.is_button_pressed());
        assert!(!event.is_button_released());
        assert!(event.is_axis_moved());
    }

    #[test]
    fn test_latency_calculation() {
        // Use actual timing with Instant
        let press = InputEvent::button_press(ButtonCode::South);

        // Simulate small delay
        thread::sleep(Duration::from_millis(5));

        let release = InputEvent::button_release(ButtonCode::South);

        // Calculate actual elapsed time
        let latency = release.timestamp().duration_since(press.timestamp());

        // Should be at least 5ms (might be slightly more due to scheduler)
        assert!(latency >= Duration::from_millis(5));
        assert!(latency < Duration::from_millis(100)); // Sanity check
    }

    #[test]
    fn test_timestamp_ordering() {
        // Create events in sequence
        let event1 = InputEvent::button_press(ButtonCode::South);
        let event2 = InputEvent::button_press(ButtonCode::East);

        // Second event should have later or equal timestamp
        assert!(event2.timestamp() >= event1.timestamp());
    }

    #[test]
    fn test_timestamp_with_test_helper() {
        // For tests that need specific timestamps
        let base = Instant::now();
        let ts1 = base;
        let ts2 = base + Duration::from_millis(10);

        let press = InputEvent::button_press_at(ButtonCode::South, ts1);
        let release = InputEvent::button_release_at(ButtonCode::South, ts2);

        let latency = release.timestamp().duration_since(press.timestamp());
        assert_eq!(latency, Duration::from_millis(10));
    }

    #[test]
    fn test_elapsed_since_event() {
        let event = InputEvent::button_press(ButtonCode::South);

        // Wait a bit
        thread::sleep(Duration::from_millis(10));

        // Check how much time has elapsed since event
        let elapsed = event.timestamp().elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_copy_trait() {
        let event1 = InputEvent::button_press(ButtonCode::South);
        let event2 = event1; // Copy (not move)

        // Both should be usable (Copy trait allows this)
        assert!(event1.is_button_pressed());
        assert!(event2.is_button_pressed());
    }

    #[test]
    fn test_button_code_display() {
        assert_eq!(ButtonCode::South.to_string(), "South");
        assert_eq!(ButtonCode::East.to_string(), "East");
        assert_eq!(ButtonCode::North.to_string(), "North");
        assert_eq!(ButtonCode::West.to_string(), "West");
        assert_eq!(ButtonCode::LeftShoulder.to_string(), "Left Shoulder");
        assert_eq!(ButtonCode::RightShoulder.to_string(), "Right Shoulder");
        assert_eq!(ButtonCode::LeftTrigger.to_string(), "Left Trigger");
        assert_eq!(ButtonCode::RightTrigger.to_string(), "Right Trigger");
        assert_eq!(ButtonCode::LeftStick.to_string(), "Left Stick");
        assert_eq!(ButtonCode::RightStick.to_string(), "Right Stick");
    }

    #[test]
    fn test_axis_code_display() {
        assert_eq!(AxisCode::LeftX.to_string(), "Left X");
        assert_eq!(AxisCode::LeftY.to_string(), "Left Y");
        assert_eq!(AxisCode::RightX.to_string(), "Right X");
        assert_eq!(AxisCode::RightY.to_string(), "Right Y");
        assert_eq!(AxisCode::LeftTrigger.to_string(), "Left Trigger");
        assert_eq!(AxisCode::RightTrigger.to_string(), "Right Trigger");
        assert_eq!(AxisCode::DPadX.to_string(), "DPad X");
        assert_eq!(AxisCode::DPadY.to_string(), "DPad Y");
    }

    #[test]
    fn test_input_event_display() {
        let button_event = InputEvent::button_press(ButtonCode::South);
        assert_eq!(format!("{}", button_event), "South (pressed)");

        let release_event = InputEvent::button_release(ButtonCode::South);
        assert_eq!(format!("{}", release_event), "South (released)");

        let axis_event = InputEvent::axis_move(AxisCode::LeftX, 12345);
        assert_eq!(format!("{}", axis_event), "Left X: 12345");

        let sync_event = InputEvent::sync();
        assert_eq!(format!("{}", sync_event), "Sync");
    }

    #[test]
    fn test_is_in_deadzone() {
        // Test axis events within deadzone
        let center_event = InputEvent::axis_move(AxisCode::LeftX, 128);
        assert!(center_event.is_in_deadzone());

        let near_center_event = InputEvent::axis_move(AxisCode::LeftX, 125);
        assert!(near_center_event.is_in_deadzone());

        let boundary_low = InputEvent::axis_move(AxisCode::LeftX, 118);
        assert!(boundary_low.is_in_deadzone());

        let boundary_high = InputEvent::axis_move(AxisCode::LeftX, 138);
        assert!(boundary_high.is_in_deadzone());

        // Test axis events outside deadzone
        let outside_low = InputEvent::axis_move(AxisCode::LeftX, 110);
        assert!(!outside_low.is_in_deadzone());

        let outside_high = InputEvent::axis_move(AxisCode::LeftX, 150);
        assert!(!outside_high.is_in_deadzone());

        // Test that triggers are never in deadzone (even at center)
        let trigger_center = InputEvent::axis_move(AxisCode::LeftTrigger, 128);
        assert!(!trigger_center.is_in_deadzone());

        // Test that non-axis events are not in deadzone
        let button_event = InputEvent::button_press(ButtonCode::South);
        assert!(!button_event.is_in_deadzone());

        let sync_event = InputEvent::sync();
        assert!(!sync_event.is_in_deadzone());
    }

    #[test]
    fn test_deadzone_boundary_cases() {
        // Test exact deadzone boundaries (±10 from center)
        let deadzone_min = InputEvent::axis_move(AxisCode::LeftX, 128 - 10);
        assert!(deadzone_min.is_in_deadzone());

        let deadzone_max = InputEvent::axis_move(AxisCode::LeftX, 128 + 10);
        assert!(deadzone_max.is_in_deadzone());

        let just_outside_min = InputEvent::axis_move(AxisCode::LeftX, 128 - 11);
        assert!(!just_outside_min.is_in_deadzone());

        let just_outside_max = InputEvent::axis_move(AxisCode::LeftX, 128 + 11);
        assert!(!just_outside_max.is_in_deadzone());
    }

    #[test]
    fn test_axis_and_direction_to_string() {
        // DPadX
        assert_eq!(
            axis_and_direction_to_string(AxisCode::DPadX, AxisDirection::Negative),
            "DPad Left"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::DPadX, AxisDirection::Positive),
            "DPad Right"
        );

        // DPadY
        assert_eq!(
            axis_and_direction_to_string(AxisCode::DPadY, AxisDirection::Negative),
            "DPad Up"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::DPadY, AxisDirection::Positive),
            "DPad Down"
        );

        // LeftX
        assert_eq!(
            axis_and_direction_to_string(AxisCode::LeftX, AxisDirection::Negative),
            "Left X Left"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::LeftX, AxisDirection::Positive),
            "Left X Right"
        );

        // RightX
        assert_eq!(
            axis_and_direction_to_string(AxisCode::RightX, AxisDirection::Negative),
            "Right X Left"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::RightX, AxisDirection::Positive),
            "Right X Right"
        );

        // LeftY
        assert_eq!(
            axis_and_direction_to_string(AxisCode::LeftY, AxisDirection::Negative),
            "Left Y Up"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::LeftY, AxisDirection::Positive),
            "Left Y Down"
        );

        // RightY
        assert_eq!(
            axis_and_direction_to_string(AxisCode::RightY, AxisDirection::Negative),
            "Right Y Up"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::RightY, AxisDirection::Positive),
            "Right Y Down"
        );

        // Other axis codes (should just return their display string)
        assert_eq!(
            axis_and_direction_to_string(AxisCode::LeftTrigger, AxisDirection::Negative),
            "Left Trigger"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::RightTrigger, AxisDirection::Positive),
            "Right Trigger"
        );
        assert_eq!(
            axis_and_direction_to_string(AxisCode::Unknown, AxisDirection::Negative),
            "Unknown"
        );
    }
}
