// Platform abstraction module

pub mod linux;

use crate::input::InputManager;
use crate::output::keyboard::VirtualKeyboard;

/// Create a device manager for the current platform
/// For now, we only support Linux
pub fn new_input_manager() -> Box<dyn InputManager> {
    Box::new(linux::LinuxInputManager::new())
}

/// Create a virtual keyboard for the current platform
pub fn new_virtual_keyboard(name: &str) -> anyhow::Result<Box<dyn VirtualKeyboard>> {
    Ok(Box::new(linux::LinuxVirtualKeyboard::new(name)?))
}
