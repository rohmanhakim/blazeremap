// Gamepad type definitions

use std::fmt;

/// Represents different gamepad types we can detect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadType {
    Unknown,
    XboxOne,
    XboxSeries,
    XboxElite,
    DualShock4,
    DualSense,
    Generic,
}

impl fmt::Display for GamepadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XboxOne => write!(f, "Xbox One"),
            Self::XboxSeries => write!(f, "Xbox Series X/S"),
            Self::XboxElite => write!(f, "Xbox Elite"),
            Self::DualShock4 => write!(f, "DualShock 4"),
            Self::DualSense => write!(f, "DualSense"),
            Self::Generic => write!(f, "Generic"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Gamepad capabilities that can be detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadCapability {
    ForceFeedback,
    ElitePaddles,
}

impl fmt::Display for GamepadCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForceFeedback => write!(f, "Force Feedback"),
            Self::ElitePaddles => write!(f, "Elite Paddles"),
        }
    }
}

/// Helper function to convert capabilities to strings
pub fn capabilities_to_strings(caps: &[GamepadCapability]) -> Vec<String> {
    caps.iter().map(|cap| cap.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_type_display() {
        assert_eq!(GamepadType::XboxOne.to_string(), "Xbox One");
        assert_eq!(GamepadType::DualShock4.to_string(), "DualShock 4");
    }

    #[test]
    fn test_capability_display() {
        assert_eq!(GamepadCapability::ForceFeedback.to_string(), "Force Feedback");
    }

    #[test]
    fn test_capabilities_to_strings() {
        let caps = vec![GamepadCapability::ForceFeedback, GamepadCapability::ElitePaddles];
        let strings = capabilities_to_strings(&caps);
        assert_eq!(strings, vec!["Force Feedback", "Elite Paddles"]);
    }
}
