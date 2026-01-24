// Controller information
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
