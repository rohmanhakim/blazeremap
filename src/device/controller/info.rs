// Controller information and trait definition

use super::types::{ControllerCapability, ControllerType};

/// Information about a detected controller
#[derive(Debug, Clone)]
pub struct ControllerInfo {
    pub path: String,
    pub name: String,
    pub controller_type: ControllerType,
    pub vendor_id: u16,
    pub vendor_name: String,
    pub product_id: u16,
    pub capabilities: Vec<ControllerCapability>,
}

/// Controller trait - represents a physical game controller
pub trait Controller {
    /// Get detailed info about the controller
    fn get_info(&self) -> &ControllerInfo;

    /// Close releases the device
    fn close(self) -> anyhow::Result<()>;
}
