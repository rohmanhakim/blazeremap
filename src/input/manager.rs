// Input device management types and traits

use super::gamepad::{Gamepad, GamepadInfo};
use thiserror::Error;

/// InputManager trait - handles input device discovery and creation
#[cfg_attr(test, mockall::automock)]
pub trait InputManager {
    /// List all connected gamepads
    fn list_gamepads(&self) -> anyhow::Result<InputDetectionResult>;

    /// Open a specific gamepad by path
    fn open_gamepad(&self, path: &str) -> anyhow::Result<Box<dyn Gamepad>>;
}

/// Results of gamepad detection
#[derive(Debug, Default)]
pub struct InputDetectionResult {
    pub gamepad_info: Vec<GamepadInfo>,
    pub errors: Vec<InputDeviceError>,
}

/// Error types for device operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Permission,    // Permission denied
    NotFound,      // Device not found
    InvalidDevice, // Invalid or unsupported device
    Unknown,       // Unknown error
}

/// Device-related error
#[derive(Debug, Error)]
pub struct InputDeviceError {
    pub path: String,
    pub error_type: ErrorType,
    #[source]
    pub source: anyhow::Error,
}

impl std::fmt::Display for InputDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} error at {}: {}", self.error_type, self.path, self.source)
    }
}

impl InputDeviceError {
    pub fn new(path: String, error_type: ErrorType, source: anyhow::Error) -> Self {
        Self { path, error_type, source }
    }
}
