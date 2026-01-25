// Linux-specific errors
use crate::input::ErrorType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LinuxError {
    #[error("permission denied")]
    PermissionDenied,

    #[error("device not found")]
    DeviceNotFound,

    #[error("invalid device")]
    InvalidDevice,
}

/// Convert Linux-specific errors to generic ErrorType
pub(super) fn classify_error(err: &anyhow::Error) -> ErrorType {
    match err.downcast_ref::<LinuxError>() {
        Some(LinuxError::PermissionDenied) => ErrorType::Permission,
        Some(LinuxError::DeviceNotFound) => ErrorType::NotFound,
        Some(LinuxError::InvalidDevice) => ErrorType::InvalidDevice,
        None => ErrorType::Unknown,
    }
}
