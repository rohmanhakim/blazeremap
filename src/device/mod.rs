// Device module
// Mirrors: internal/device/ package

pub mod controller;
pub mod manager;

// Re-export main types
pub use controller::{Controller, ControllerCapability, ControllerInfo, ControllerType};
pub use manager::{DetectionResult, DeviceError, DeviceManager, ErrorType};
