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
   | `KEY`               | `Button`             | Controller buttons    |
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
   `None`, as they are not relevant for controller input remapping.
*/
use crate::event::{EventType, InputEvent};

pub fn evdev_to_input(ev: evdev::InputEvent) -> Option<InputEvent> {
    let event_type = match ev.event_type() {
        evdev::EventType::KEY => EventType::Button,
        evdev::EventType::ABSOLUTE => EventType::Axis,
        evdev::EventType::SYNCHRONIZATION => EventType::Sync,
        evdev::EventType::SWITCH => EventType::DPad,
        _ => return None,
    };

    let timestamp = std::time::Instant::now();

    Some(InputEvent { event_type, code: ev.code(), value: ev.value(), timestamp })
}

#[cfg(test)]
mod tests {
    use super::*;
    use evdev::{EventType as EvdevEventType, InputEvent as EvdevEvent};
    use std::time::Duration;

    #[test]
    fn test_evdev_key_to_button() {
        let evdev_event = EvdevEvent::new(EvdevEventType::KEY, 304, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.event_type, EventType::Button);
        assert_eq!(event.code, 304);
        assert_eq!(event.value, 1);
    }

    #[test]
    fn test_evdev_abs_to_axis() {
        let evdev_event = EvdevEvent::new(EvdevEventType::ABSOLUTE, 0, 15234);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.event_type, EventType::Axis);
        assert_eq!(event.code, 0);
        assert_eq!(event.value, 15234);
    }

    #[test]
    fn test_evdev_sync_returns_sync() {
        let evdev_event = EvdevEvent::new(EvdevEventType::SYNCHRONIZATION, 0, 0);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.event_type, EventType::Sync);
    }

    #[test]
    fn test_evdev_switch_to_dpad() {
        let evdev_event = EvdevEvent::new(EvdevEventType::SWITCH, 0, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.event_type, EventType::DPad);
    }

    #[test]
    fn test_unsupported_event_type_returns_none() {
        let evdev_event = EvdevEvent::new(EvdevEventType::LED, 0, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_none());
    }

    #[test]
    fn test_timestamp_is_recent() {
        let before = std::time::Instant::now();
        let evdev_event = EvdevEvent::new(EvdevEventType::KEY, 304, 1);
        let event = evdev_to_input(evdev_event).unwrap();
        let after = std::time::Instant::now();

        // Timestamp should be captured during conversion
        assert!(event.timestamp >= before);
        assert!(event.timestamp <= after);
    }

    #[test]
    fn test_timestamps_are_monotonic() {
        // Create two events in sequence
        let event1 = evdev_to_input(EvdevEvent::new(EvdevEventType::KEY, 304, 1)).unwrap();

        // Small delay to ensure time advances
        std::thread::sleep(Duration::from_micros(100));

        let event2 = evdev_to_input(EvdevEvent::new(EvdevEventType::KEY, 305, 1)).unwrap();

        // Second timestamp should be >= first (monotonic property)
        assert!(event2.timestamp >= event1.timestamp);
    }

    #[test]
    fn test_elapsed_is_non_negative() {
        let evdev_event = EvdevEvent::new(EvdevEventType::KEY, 304, 1);
        let event = evdev_to_input(evdev_event).unwrap();

        // Elapsed time is always >= 0 (Instant is monotonic)
        assert!(event.timestamp.elapsed() >= Duration::ZERO);
    }
}
