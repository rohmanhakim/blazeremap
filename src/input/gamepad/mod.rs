// Gamepad module

pub mod database;
pub mod info;
pub mod types;

// Re-export commonly used types
pub use database::{get_known_vendor_database, identify_gamepad};
pub use info::GamepadInfo;
pub use types::{GamepadCapability, GamepadType, capabilities_to_strings};

#[cfg_attr(test, mockall::automock)]
pub trait Gamepad {
    /// Get detailed info about the gamepad
    fn get_info(&self) -> &GamepadInfo;

    /// Read the next input event (BLOCKING)
    /// Returns None when device is disconnected
    fn read_event(&mut self) -> anyhow::Result<Option<crate::event::InputEvent>>;

    /// Close releases the device
    fn close(self) -> anyhow::Result<()>;
}
