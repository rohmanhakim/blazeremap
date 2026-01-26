pub mod engine;
pub mod profile;
pub mod rules;
pub mod types;

pub use engine::MappingEngine;
pub use rules::MappingRule;
pub use rules::MappingRule::AxisDirectionToKey;
pub use rules::MappingRule::ButtonToKey;

use serde::Deserialize;
use serde::Serialize;

use crate::mapping::types::TargetType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    /// Source button name (for readability)
    pub source_name: String,

    /// Source direction (up, right, left, down)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_direction: Option<String>,

    /// Target type
    pub target_type: TargetType, // "keyboard", "mouse", "gamepad"

    /// Target key name (for readability)
    pub target_name: String,
}
