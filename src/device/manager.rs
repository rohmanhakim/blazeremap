// Device management types and traits

use super::controller::ControllerInfo;
use thiserror::Error;

/// DeviceManager trait - handles device discovery and creation
pub trait DeviceManager {
    /// List all connected controllers
    fn list_controllers(&self) -> anyhow::Result<DetectionResult>;
}

/// Results of controller detection
#[derive(Debug, Default)]
pub struct DetectionResult {
    pub controller_info: Vec<ControllerInfo>,
    pub errors: Vec<DeviceError>,
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
pub struct DeviceError {
    pub path: String,
    pub error_type: ErrorType,
    #[source]
    pub source: anyhow::Error,
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} error at {}: {}", self.error_type, self.path, self.source)
    }
}

impl DeviceError {
    pub fn new(path: String, error_type: ErrorType, source: anyhow::Error) -> Self {
        Self { path, error_type, source }
    }
}
