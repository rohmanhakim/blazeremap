// Platform abstraction module

pub mod linux;

use crate::device::DeviceManager;

/// Create a device manager for the current platform
/// For now, we only support Linux
pub fn new_device_manager() -> Box<dyn DeviceManager> {
    Box::new(linux::LinuxDeviceManager::new())
}
