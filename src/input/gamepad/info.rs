// Gamepad information
use super::types::{GamepadCapability, GamepadType};

/// Information about a detected gamepad
#[derive(Debug, Clone)]
pub struct GamepadInfo {
    pub path: String,
    pub name: String,
    pub gamepad_type: GamepadType,
    pub vendor_id: u16,
    pub vendor_name: String,
    pub product_id: u16,
    pub capabilities: Vec<GamepadCapability>,
}
