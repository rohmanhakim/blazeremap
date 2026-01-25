/*
 Event type definitions for controller input processing.

  This module defines the core event types used throughout BlazeRemap for
  representing controller inputs and their mapped outputs.

  # Event Flow
  Physical Controller → InputEvent → Mapping Engine → OutputEvent → Virtual Device

  # Types
  - [`InputEvent`]: Represents a raw input from a physical controller (button press,
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
  a common domain model that works across platforms and controller types.
*/
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Mode,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad,
    Unknown,
}

impl fmt::Display for ButtonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::DPadUp => write!(f, "DPad Up"),
            Self::DPadDown => write!(f, "DPad Down"),
            Self::DPadLeft => write!(f, "DPad Left"),
            Self::DPadRight => write!(f, "DPad Right"),
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

impl fmt::Display for AxisCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        assert_eq!(ButtonCode::DPadUp.to_string(), "DPad Up");
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
}
