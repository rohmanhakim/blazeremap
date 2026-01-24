// Controller module

// pub mod controller;
pub mod database;
pub mod info;
pub mod types;

// Re-export commonly used types
// pub use controller::Controller;
pub use database::{get_known_vendor_database, identify_controller};
pub use info::ControllerInfo;
pub use types::{ControllerCapability, ControllerType, capabilities_to_strings};

pub trait Controller {
    /// Get detailed info about the controller
    fn get_info(&self) -> &ControllerInfo;

    /// Read the next input event (BLOCKING)
    /// Returns None when device is disconnected
    fn read_event(&mut self) -> anyhow::Result<Option<crate::event::InputEvent>>;

    /// Close releases the device
    fn close(self) -> anyhow::Result<()>;
}
