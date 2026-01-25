// Platform abstraction module

pub mod linux;

use crate::input::InputManager;

/// Create a device manager for the current platform
/// For now, we only support Linux
pub fn new_device_manager() -> Box<dyn InputManager> {
    Box::new(linux::LinuxInputManager::new())
}
