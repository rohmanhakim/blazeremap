// BlazeRemap - Linux controller remapping library
//! This is the main library crate for BlazeRemap.
//!
//! # Architecture
//!
//! BlazeRemap follows a hexagonal/ports-and-adapters architecture:
//!
//! - `device`: Core domain logic (controllers, capabilities, traits)
//! - `platform`: Platform-specific implementations (Linux evdev)
//! - `cli`: User interface layer (CLI commands)
//! - `app`: Application composition and wiring

// Public modules
pub mod app;
pub mod cli;
pub mod device;
pub mod platform;

// Re-export commonly used types
pub use device::controller::{Controller, ControllerInfo, ControllerType};
pub use device::{DetectionResult, DeviceManager};
