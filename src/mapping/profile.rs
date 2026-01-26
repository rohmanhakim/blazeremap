// src/mapping/profile.rs
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    event::{AxisCode, AxisDirection, ButtonCode, KeyboardCode},
    mapping::{Mapping, types::TargetType},
};

/// Complete controller profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    pub mappings: Vec<Mapping>,

    #[serde(default)]
    pub settings: ProfileSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    #[serde(default = "default_vibration_enabled")]
    pub vibration_enabled: bool,

    #[serde(default = "default_vibration_intensity")]
    pub vibration_intensity: u8, // 0-100
}

fn default_vibration_enabled() -> bool {
    true
}
fn default_vibration_intensity() -> u8 {
    100
}

impl Default for ProfileSettings {
    fn default() -> Self {
        Self {
            vibration_enabled: default_vibration_enabled(),
            vibration_intensity: default_vibration_intensity(),
        }
    }
}

impl Profile {
    /// Create a default profile (hardcoded mappings)
    pub fn default_profile() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default button mappings".to_string(),
            game_name: None,
            mappings: vec![
                Mapping {
                    source_name: ButtonCode::North.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::W.to_string(),
                },
                Mapping {
                    source_name: ButtonCode::West.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::A.to_string(),
                },
                Mapping {
                    source_name: ButtonCode::South.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::S.to_string(),
                },
                Mapping {
                    source_name: ButtonCode::East.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::D.to_string(),
                },
                Mapping {
                    source_name: ButtonCode::Select.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Escape.to_string(),
                },
                Mapping {
                    source_name: ButtonCode::Start.to_string(),
                    source_direction: None,
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Enter.to_string(),
                },
                //
                Mapping {
                    source_name: AxisCode::DPadY.to_string(),
                    source_direction: Some(AxisDirection::Negative.to_string()),
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Up.to_string(),
                },
                Mapping {
                    source_name: AxisCode::DPadY.to_string(),
                    source_direction: Some(AxisDirection::Positive.to_string()),
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Down.to_string(),
                },
                Mapping {
                    source_name: AxisCode::DPadX.to_string(),
                    source_direction: Some(AxisDirection::Negative.to_string()),
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Left.to_string(),
                },
                Mapping {
                    source_name: AxisCode::DPadX.to_string(),
                    source_direction: Some(AxisDirection::Positive.to_string()),
                    target_type: TargetType::Keyboard,
                    target_name: KeyboardCode::Right.to_string(),
                },
            ],
            settings: ProfileSettings::default(),
        }
    }

    /// Save profile to TOML file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let toml_string = toml::to_string_pretty(self).context("Failed to serialize profile")?;

        std::fs::write(path, toml_string).context("Failed to write profile file")?;

        Ok(())
    }

    /// Load profile from TOML file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let toml_string = std::fs::read_to_string(path).context("Failed to read profile file")?;

        let profile: Profile =
            toml::from_str(&toml_string).context("Failed to parse profile JSON")?;

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile() {
        let profile = Profile::default_profile();
        assert_eq!(profile.name, "Default");
        assert_eq!(profile.mappings.len(), 10); // Corrected mapping count
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile::default_profile();
        let toml_string = toml::to_string_pretty(&profile).unwrap();

        let expected_toml = r#"name = "Default"
description = "Default button mappings"

[[mappings]]
source_name = "North"
target_type = "Keyboard"
target_name = "W"

[[mappings]]
source_name = "West"
target_type = "Keyboard"
target_name = "A"

[[mappings]]
source_name = "South"
target_type = "Keyboard"
target_name = "S"

[[mappings]]
source_name = "East"
target_type = "Keyboard"
target_name = "D"

[[mappings]]
source_name = "Select"
target_type = "Keyboard"
target_name = "Escape"

[[mappings]]
source_name = "Start"
target_type = "Keyboard"
target_name = "Enter"

[[mappings]]
source_name = "DPad Y"
source_direction = "Negative"
target_type = "Keyboard"
target_name = "Up"

[[mappings]]
source_name = "DPad Y"
source_direction = "Positive"
target_type = "Keyboard"
target_name = "Down"

[[mappings]]
source_name = "DPad X"
source_direction = "Negative"
target_type = "Keyboard"
target_name = "Left"

[[mappings]]
source_name = "DPad X"
source_direction = "Positive"
target_type = "Keyboard"
target_name = "Right"

[settings]
vibration_enabled = true
vibration_intensity = 100
"#;

        assert_eq!(toml_string, expected_toml);
    }

    #[test]
    fn test_profile_round_trip() {
        let profile = Profile::default_profile();

        // Serialize
        let toml_string = toml::to_string(&profile).unwrap();

        // Deserialize
        let loaded: Profile = toml::from_str(&toml_string).unwrap();

        assert_eq!(profile.name, loaded.name);
        assert_eq!(profile.mappings.len(), loaded.mappings.len());
    }

    #[test]
    fn test_profile_save_load() {
        use std::path::PathBuf;

        let profile = Profile::default_profile();
        let path = PathBuf::from("/tmp/test_profile.json");

        // Save
        profile.save_to_file(&path).unwrap();

        // Load
        let loaded = Profile::load_from_file(&path).unwrap();

        assert_eq!(profile.name, loaded.name);

        // Cleanup
        std::fs::remove_file(path).ok();
    }
}
