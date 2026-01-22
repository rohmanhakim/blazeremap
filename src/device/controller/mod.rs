// Controller module

pub mod database;
pub mod info;
pub mod types;

// Re-export commonly used types
pub use database::{get_known_vendor_database, identify_controller};
pub use info::{Controller, ControllerInfo};
pub use types::{ControllerCapability, ControllerType, capabilities_to_strings};
