use std::collections::HashMap;

use anyhow::Result;

use crate::{
    event::{ButtonCode, InputEvent},
    mapping::{MappingRule, types::TargetType},
    output::{
        event::OutputEvent,
        types::{KeyboardCode, KeyboardEventType},
    },
};

pub struct MappingEngine {
    rules: HashMap<ButtonCode, MappingRule>,
}

impl MappingEngine {
    pub fn new_hardcoded() -> Self {
        let mut rules = HashMap::new();

        // Hardcoded mappings for testing:
        // South button -> Space
        rules.insert(
            ButtonCode::South,
            MappingRule::keyboard(ButtonCode::South, KeyboardCode::Space),
        );

        // East button -> E
        rules.insert(ButtonCode::East, MappingRule::keyboard(ButtonCode::East, KeyboardCode::E));

        // West button -> R
        rules.insert(ButtonCode::West, MappingRule::keyboard(ButtonCode::West, KeyboardCode::R));

        tracing::info!("Mapping engine initialized with {} rules", rules.len());

        Self { rules }
    }

    /// Process an input event and generate output events
    pub fn process(&self, event: &InputEvent) -> Result<Option<OutputEvent>> {
        match event {
            InputEvent::Button { code, pressed, .. } => {
                // Look up mapping rule by button code
                if let Some(rule) = self.rules.get(code) {
                    let output = match rule.target_type {
                        TargetType::Keyboard => OutputEvent::Keyboard {
                            code: rule.target_code,
                            event_type: if *pressed {
                                KeyboardEventType::Press
                            } else {
                                KeyboardEventType::Release
                            },
                        },
                        TargetType::Mouse => {
                            // TODO: Implement mouse output
                            return Ok(None);
                        }
                        TargetType::Gamepad => {
                            // TODO: Implement gamepad output
                            return Ok(None);
                        }
                    };

                    tracing::debug!("Mapped: button {:?} -> {:?}", code, output);

                    Ok(Some(output))
                } else {
                    // No mapping for this button
                    Ok(None)
                }
            }
            InputEvent::Axis { .. } => {
                // Future: handle axis mappings
                Ok(None)
            }
            InputEvent::Sync { .. } => {
                // Skip sync events
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{AxisCode, ButtonCode};

    #[test]
    fn test_mapping_engine_hardcoded_press() {
        let engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_press(ButtonCode::South);

        let result = engine.process(&input).unwrap();

        assert!(result.is_some());
        if let Some(OutputEvent::Keyboard { code, event_type }) = result {
            assert_eq!(code, KeyboardCode::Space);
            assert_eq!(event_type, KeyboardEventType::Press);
        } else {
            panic!("Expected Keyboard output event");
        }
    }

    #[test]
    fn test_mapping_engine_hardcoded_release() {
        let engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_release(ButtonCode::East);

        let result = engine.process(&input).unwrap();

        assert!(result.is_some());
        if let Some(OutputEvent::Keyboard { code, event_type }) = result {
            assert_eq!(code, KeyboardCode::E);
            assert_eq!(event_type, KeyboardEventType::Release);
        } else {
            panic!("Expected Keyboard output event");
        }
    }

    #[test]
    fn test_unmapped_button() {
        let engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_press(ButtonCode::North); // North is not in hardcoded rules

        let result = engine.process(&input).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_axis_passthrough_returns_none() {
        let engine = MappingEngine::new_hardcoded();
        let input = InputEvent::axis_move(AxisCode::LeftX, 100);

        let result = engine.process(&input).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_sync_returns_none() {
        let engine = MappingEngine::new_hardcoded();
        let input = InputEvent::sync();

        let result = engine.process(&input).unwrap();
        assert!(result.is_none());
    }
}
