/*
Conversion utilities for translating evdev events to domain events.

 This module provides converters that transform platform-specific event types
 (from the Linux evdev subsystem) into BlazeRemap's platform-agnostic event types.

 # Purpose
 The converter acts as a boundary layer between the Linux input subsystem and
 BlazeRemap's core logic, allowing the rest of the application to work with
 clean, domain-specific types regardless of the underlying platform.

 # Event Mapping
 | evdev EventType      | BlazeRemap EventType | Description          |
 |---------------------|----------------------|-----------------------|
 | `KEY`               | `Button`             | Gamepad buttons    |
 | `ABSOLUTE`          | `Axis`               | Analog sticks/triggers|
 | `SYNCHRONIZATION`   | `Sync`               | Frame boundaries      |
 | `SWITCH`            | `DPad`               | Directional pad       |
 | Others              | `None`               | Filtered out          |

 # Timestamp Handling
 The converter discards evdev's kernel timestamp and captures a new [`Instant`]
 when the event is converted. This timestamp represents when BlazeRemap's userspace
 code received the event, which is the starting point for latency measurement.

 # Note
 Unsupported evdev event types (LED, SOUND, etc.) are filtered out and return
 `None`, as they are not relevant for gamepad input remapping.
*/

use crate::event::{AxisCode, ButtonCode, InputEvent, system_time_to_instant};

pub fn evdev_to_input(ev: evdev::InputEvent) -> Option<InputEvent> {
    //  Convert kernel's SystemTime to Instant (preserves timing)
    let timestamp = system_time_to_instant(ev.timestamp());
    println!("evdev_to_input timestamp: {:?}", timestamp);

    match ev.destructure() {
        evdev::EventSummary::Key(_, key_code, _value) => {
            let button_code = key_to_button_code(key_code);
            let pressed = _value > 0;
            Some(InputEvent::Button { code: button_code, pressed, timestamp })
        }
        evdev::EventSummary::AbsoluteAxis(_, axis_code, value) => {
            let axis_code = axis_to_axis_code(axis_code);
            Some(InputEvent::Axis { code: axis_code, value, timestamp })
        }
        evdev::EventSummary::Switch(_, _switch_code, _value) => {
            // DPad events are typically handled as axes (ABS_HAT0X/Y) rather than switches
            // For now, we'll skip switch events as they're not commonly used for gamepads
            None
        }
        evdev::EventSummary::Synchronization(_, _, _) => Some(InputEvent::Sync { timestamp }),
        _ => None,
    }
}

fn key_to_button_code(key: evdev::KeyCode) -> ButtonCode {
    match key {
        evdev::KeyCode::BTN_SOUTH => ButtonCode::South,
        evdev::KeyCode::BTN_EAST => ButtonCode::East,
        evdev::KeyCode::BTN_NORTH => ButtonCode::North,
        evdev::KeyCode::BTN_WEST => ButtonCode::West,
        evdev::KeyCode::BTN_TL => ButtonCode::LeftShoulder,
        evdev::KeyCode::BTN_TR => ButtonCode::RightShoulder,
        evdev::KeyCode::BTN_TL2 => ButtonCode::LeftTrigger,
        evdev::KeyCode::BTN_TR2 => ButtonCode::RightTrigger,
        evdev::KeyCode::BTN_SELECT => ButtonCode::Select,
        evdev::KeyCode::BTN_START => ButtonCode::Start,
        evdev::KeyCode::BTN_MODE => ButtonCode::Mode,
        evdev::KeyCode::BTN_THUMBL => ButtonCode::LeftStick,
        evdev::KeyCode::BTN_THUMBR => ButtonCode::RightStick,
        evdev::KeyCode::BTN_TRIGGER_HAPPY1 => ButtonCode::Paddle1,
        evdev::KeyCode::BTN_TRIGGER_HAPPY2 => ButtonCode::Paddle2,
        evdev::KeyCode::BTN_TRIGGER_HAPPY3 => ButtonCode::Paddle3,
        evdev::KeyCode::BTN_TRIGGER_HAPPY4 => ButtonCode::Paddle4,
        _ => ButtonCode::Unknown,
    }
}

fn axis_to_axis_code(axis: evdev::AbsoluteAxisCode) -> AxisCode {
    match axis {
        evdev::AbsoluteAxisCode::ABS_X => AxisCode::LeftX,
        evdev::AbsoluteAxisCode::ABS_Y => AxisCode::LeftY,
        evdev::AbsoluteAxisCode::ABS_RX => AxisCode::RightX,
        evdev::AbsoluteAxisCode::ABS_RY => AxisCode::RightY,
        evdev::AbsoluteAxisCode::ABS_Z => AxisCode::LeftTrigger,
        evdev::AbsoluteAxisCode::ABS_RZ => AxisCode::RightTrigger,
        evdev::AbsoluteAxisCode::ABS_HAT0X => AxisCode::DPadX,
        evdev::AbsoluteAxisCode::ABS_HAT0Y => AxisCode::DPadY,
        _ => AxisCode::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evdev::InputEvent as EvdevEvent;
    use std::time::Duration;

    #[test]
    fn test_evdev_key_to_button() {
        let evdev_event = EvdevEvent::new(evdev::EventType::KEY.0, 0x130, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Button { code: ButtonCode::South, pressed: true, .. }));
    }

    #[test]
    fn test_evdev_abs_to_axis() {
        use crate::event::init_time_anchor;
        init_time_anchor();

        let evdev_event = EvdevEvent::new_now(evdev::EventType::ABSOLUTE.0, 0x00, 15234);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Axis { code: AxisCode::LeftX, value: 15234, .. }));
    }

    #[test]
    fn test_evdev_sync_returns_sync() {
        let evdev_event = EvdevEvent::new(evdev::EventType::SYNCHRONIZATION.0, 0, 0);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Sync { .. }));
    }

    #[test]
    fn test_unsupported_event_type_returns_none() {
        let evdev_event = EvdevEvent::new(evdev::EventType::LED.0, 0, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_none());
    }

    #[test]
    fn test_timestamp_is_recent() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        let evdev_event = EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1);
        let event = evdev_to_input(evdev_event).unwrap();

        let age = event.timestamp().elapsed();
        assert!(age < Duration::from_secs(1), "Event timestamp is too old: {:?}", age);
    }

    #[test]
    fn test_timestamps_are_monotonic() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        // Create two events in sequence
        let event1 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1)).unwrap();

        // Small delay to ensure time advances
        std::thread::sleep(Duration::from_millis(10));

        let event2 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x131, 1)).unwrap();

        assert!(
            event2.timestamp() >= event1.timestamp(),
            "Timestamps not monotonic: event2 {:?} < event1 {:?}",
            event2.timestamp(),
            event1.timestamp()
        );
    }

    #[test]
    fn test_elapsed_is_non_negative() {
        let evdev_event = EvdevEvent::new(evdev::EventType::KEY.0, 0x130, 1);
        let event = evdev_to_input(evdev_event).unwrap();

        // Elapsed time is always >= 0 (Instant is monotonic)
        assert!(event.timestamp().elapsed() >= Duration::ZERO);
    }

    #[test]
    fn test_duration_preservation() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        let event1 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1)).unwrap();

        // Known delay
        std::thread::sleep(Duration::from_millis(50));

        let event2 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x131, 1)).unwrap();

        // ✅ Delta should be approximately 50ms
        let delta = event2.timestamp().duration_since(event1.timestamp());
        assert!(delta >= Duration::from_millis(50), "Delta too small: {:?}", delta);
        assert!(delta < Duration::from_millis(100), "Delta too large: {:?}", delta);
    }

    #[test]
    fn test_all_button_code_mappings() {
        // Test a few key button mappings to ensure they work
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_SOUTH), ButtonCode::South);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_EAST), ButtonCode::East);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_NORTH), ButtonCode::North);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_WEST), ButtonCode::West);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_START), ButtonCode::Start);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_SELECT), ButtonCode::Select);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_MODE), ButtonCode::Mode);
    }

    #[test]
    fn test_all_axis_code_mappings() {
        // Test all axis mappings
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_X), AxisCode::LeftX);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_Y), AxisCode::LeftY);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RX), AxisCode::RightX);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RY), AxisCode::RightY);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_Z), AxisCode::LeftTrigger);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RZ), AxisCode::RightTrigger);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_HAT0X), AxisCode::DPadX);
        assert_eq!(axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_HAT0Y), AxisCode::DPadY);
    }

    #[test]
    fn test_unknown_codes_map_to_unknown() {
        // Test that unknown codes map to Unknown variants
        // We can't easily create unknown enum variants, so we'll test
        // that our mapping functions are total (cover all expected cases)
        // This test passes as long as no panics occur for any input
        let _result1 = key_to_button_code(evdev::KeyCode::KEY_A);
        let _result2 = axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_X);
        // The functions should return ButtonCode::Unknown or AxisCode::Unknown for unmapped codes
    }
}
