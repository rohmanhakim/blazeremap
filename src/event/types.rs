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

   # Timestamps
   All [`InputEvent`]s use [`Instant`] timestamps captured when the event is created
   in userspace (not the kernel timestamp). This provides:
   - Monotonic timing (immune to system clock adjustments)
   - High precision for sub-millisecond latency measurement
   - Simple API for calculating elapsed time
*/
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Button,
    Axis,
    DPad,
    Sync,
}

#[derive(Debug, Copy, Clone)] // Changed: Copy instead of just Clone
pub struct InputEvent {
    pub event_type: EventType,
    pub code: u16,
    pub value: i32,
    pub timestamp: Instant,
}

impl InputEvent {
    // For production code - captures current time
    pub fn button_press(code: u16) -> Self {
        Self { event_type: EventType::Button, code, value: 1, timestamp: Instant::now() }
    }

    pub fn button_release(code: u16) -> Self {
        Self { event_type: EventType::Button, code, value: 0, timestamp: Instant::now() }
    }

    pub fn axis_move(code: u16, value: i32) -> Self {
        Self { event_type: EventType::Axis, code, value, timestamp: Instant::now() }
    }

    // For testing - allows providing a specific timestamp
    #[cfg(test)]
    pub fn button_press_at(code: u16, timestamp: Instant) -> Self {
        Self { event_type: EventType::Button, code, value: 1, timestamp }
    }

    #[cfg(test)]
    pub fn button_release_at(code: u16, timestamp: Instant) -> Self {
        Self { event_type: EventType::Button, code, value: 0, timestamp }
    }

    #[cfg(test)]
    pub fn axis_move_at(code: u16, value: i32, timestamp: Instant) -> Self {
        Self { event_type: EventType::Axis, code, value, timestamp }
    }

    pub fn is_button_pressed(&self) -> bool {
        self.event_type == EventType::Button && self.value > 0
    }

    pub fn is_button_released(&self) -> bool {
        self.event_type == EventType::Button && self.value == 0
    }

    pub fn is_axis_moved(&self) -> bool {
        self.event_type == EventType::Axis
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputEvent {
    pub output_type: OutputType,
    pub code: u16,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputType {
    Keyboard,
    Mouse,
    Gamepad,
}

impl OutputEvent {
    pub fn keyboard(code: u16, value: i32) -> Self {
        Self { output_type: OutputType::Keyboard, code, value }
    }

    pub fn mouse_button(code: u16, value: i32) -> Self {
        Self { output_type: OutputType::Mouse, code, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_button_press_event() {
        let event = InputEvent::button_press(304);
        assert_eq!(event.event_type, EventType::Button);
        assert_eq!(event.code, 304);
        assert_eq!(event.value, 1);
        assert!(event.is_button_pressed());
        assert!(!event.is_button_released());
    }

    #[test]
    fn test_button_release_event() {
        let event = InputEvent::button_release(304);
        assert_eq!(event.value, 0);
        assert!(!event.is_button_pressed());
        assert!(event.is_button_released());
    }

    #[test]
    fn test_axis_event() {
        let event = InputEvent::axis_move(0, 15234);
        assert_eq!(event.event_type, EventType::Axis);
        assert_eq!(event.value, 15234);
        assert!(event.is_axis_moved());
    }

    #[test]
    fn test_latency_calculation() {
        // Use actual timing with Instant
        let press = InputEvent::button_press(304);

        // Simulate small delay
        thread::sleep(Duration::from_millis(5));

        let release = InputEvent::button_release(304);

        // Calculate actual elapsed time
        let latency = release.timestamp.duration_since(press.timestamp);

        // Should be at least 5ms (might be slightly more due to scheduler)
        assert!(latency >= Duration::from_millis(5));
        assert!(latency < Duration::from_millis(100)); // Sanity check
    }

    #[test]
    fn test_timestamp_ordering() {
        // Create events in sequence
        let event1 = InputEvent::button_press(304);
        let event2 = InputEvent::button_press(305);

        // Second event should have later or equal timestamp
        assert!(event2.timestamp >= event1.timestamp);
    }

    #[test]
    fn test_timestamp_with_test_helper() {
        // For tests that need specific timestamps
        let base = Instant::now();
        let ts1 = base;
        let ts2 = base + Duration::from_millis(10);

        let press = InputEvent::button_press_at(304, ts1);
        let release = InputEvent::button_release_at(304, ts2);

        let latency = release.timestamp.duration_since(press.timestamp);
        assert_eq!(latency, Duration::from_millis(10));
    }

    #[test]
    fn test_elapsed_since_event() {
        let event = InputEvent::button_press(304);

        // Wait a bit
        thread::sleep(Duration::from_millis(10));

        // Check how much time has elapsed since event
        let elapsed = event.timestamp.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_copy_trait() {
        let event1 = InputEvent::button_press(304);
        let event2 = event1; // Copy (not move)

        // Both should be usable
        assert_eq!(event1.code, 304);
        assert_eq!(event2.code, 304);
    }
}
