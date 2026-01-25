use std::{
    fmt::{Display, Formatter, Result},
    time::Instant,
};

use crate::event::{AxisCode, ButtonCode};

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
