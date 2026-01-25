// Input module
pub mod gamepad;
pub mod manager;

// Re-export main types
pub use gamepad::{Gamepad, GamepadCapability, GamepadInfo, GamepadType};
pub use manager::{ErrorType, InputDetectionResult, InputDeviceError, InputManager};
