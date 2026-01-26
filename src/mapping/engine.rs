use std::collections::HashMap;

use anyhow::Result;

use crate::event::{
    AxisCode, AxisDirection, ButtonCode, InputEvent, KeyboardCode, KeyboardEventType, OutputEvent,
};

pub struct MappingEngine {
    button_rules: HashMap<ButtonCode, KeyboardCode>,
    axis_rules: HashMap<(AxisCode, AxisDirection), KeyboardCode>,
    axis_states: HashMap<AxisCode, i32>, // Track current axis values
}

impl MappingEngine {
    pub fn new_hardcoded() -> Self {
        let mut button_rules = HashMap::new();
        let mut axis_rules = HashMap::new();

        // Button mappings
        button_rules.insert(ButtonCode::South, KeyboardCode::S);
        button_rules.insert(ButtonCode::East, KeyboardCode::D);
        button_rules.insert(ButtonCode::West, KeyboardCode::A);

        // DPad mappings
        axis_rules.insert((AxisCode::DPadY, AxisDirection::Negative), KeyboardCode::Up);
        axis_rules.insert((AxisCode::DPadY, AxisDirection::Positive), KeyboardCode::Down);
        axis_rules.insert((AxisCode::DPadX, AxisDirection::Negative), KeyboardCode::Left);
        axis_rules.insert((AxisCode::DPadX, AxisDirection::Positive), KeyboardCode::Right);

        tracing::info!(
            "Mapping engine initialized with {} button rules, {} axis rules",
            button_rules.len(),
            axis_rules.len()
        );

        Self { button_rules, axis_rules, axis_states: HashMap::new() }
    }

    pub fn process(&mut self, event: &InputEvent) -> Result<Vec<OutputEvent>> {
        match event {
            InputEvent::Button { code, pressed, .. } => self.process_button(*code, *pressed),
            InputEvent::Axis { code, value, .. } => self.process_axis(*code, *value),
            InputEvent::Sync { .. } => Ok(vec![]),
        }
    }

    fn process_button(&self, code: ButtonCode, pressed: bool) -> Result<Vec<OutputEvent>> {
        if let Some(&target_key) = self.button_rules.get(&code) {
            let event = OutputEvent::Keyboard {
                code: target_key,
                event_type: if pressed {
                    KeyboardEventType::Press
                } else {
                    KeyboardEventType::Release
                },
            };
            Ok(vec![event])
        } else {
            Ok(vec![])
        }
    }

    fn process_axis(&mut self, code: AxisCode, new_value: i32) -> Result<Vec<OutputEvent>> {
        // Skip if not a DPad axis or if in deadzone
        if !matches!(code, AxisCode::DPadX | AxisCode::DPadY) {
            return Ok(vec![]);
        }

        let old_value = self.axis_states.get(&code).copied().unwrap_or(0);
        self.axis_states.insert(code, new_value);

        let mut events = Vec::new();

        // Detect direction changes and generate press/release events
        let old_direction = Self::value_to_direction(old_value);
        let new_direction = Self::value_to_direction(new_value);

        // Release old direction if it changed
        if old_direction != new_direction
            && let Some(old_dir) = old_direction
            && let Some(&target_key) = self.axis_rules.get(&(code, old_dir))
        {
            events.push(OutputEvent::Keyboard {
                code: target_key,
                event_type: KeyboardEventType::Release,
            });
        }

        // Press new direction if: active
        if old_direction != new_direction
            && let Some(new_dir) = new_direction
            && let Some(&target_key) = self.axis_rules.get(&(code, new_dir))
        {
            events.push(OutputEvent::Keyboard {
                code: target_key,
                event_type: KeyboardEventType::Press,
            });
        }

        Ok(events)
    }

    fn value_to_direction(value: i32) -> Option<AxisDirection> {
        const THRESHOLD: i32 = 0;

        if value > THRESHOLD {
            Some(AxisDirection::Positive)
        } else if value < -THRESHOLD {
            Some(AxisDirection::Negative)
        } else {
            None // Centered/neutral
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{AxisCode, ButtonCode};

    #[test]
    fn test_mapping_engine_hardcoded_press() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_press(ButtonCode::South);

        let result = engine.process(&input).unwrap();

        assert_eq!(result.len(), 1);
        let OutputEvent::Keyboard { code, event_type } = result[0];
        assert_eq!(code, KeyboardCode::S);
        assert_eq!(event_type, KeyboardEventType::Press);
    }

    #[test]
    fn test_mapping_engine_hardcoded_release() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_release(ButtonCode::East);

        let result = engine.process(&input).unwrap();

        assert_eq!(result.len(), 1);
        let OutputEvent::Keyboard { code, event_type } = result[0];
        assert_eq!(code, KeyboardCode::D);
        assert_eq!(event_type, KeyboardEventType::Release);
    }

    #[test]
    fn test_unmapped_button() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::button_press(ButtonCode::North); // North is not in hardcoded rules

        let result = engine.process(&input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_axis_passthrough_returns_none() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::axis_move(AxisCode::LeftX, 100);

        let result = engine.process(&input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_sync_returns_none() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::sync();

        let result = engine.process(&input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_dpad_up_press() {
        let mut engine = MappingEngine::new_hardcoded();
        let input = InputEvent::axis_move(AxisCode::DPadY, -1);

        let events = engine.process(&input).unwrap();
        assert_eq!(events.len(), 1);

        let OutputEvent::Keyboard { code, event_type } = events[0];
        assert_eq!(code, KeyboardCode::Up);
        assert_eq!(event_type, KeyboardEventType::Press);
    }

    #[test]
    fn test_dpad_release() {
        let mut engine = MappingEngine::new_hardcoded();

        // Press up
        engine.process(&InputEvent::axis_move(AxisCode::DPadY, -1)).unwrap();

        // Release (return to center)
        let events = engine.process(&InputEvent::axis_move(AxisCode::DPadY, 0)).unwrap();

        assert_eq!(events.len(), 1);
        let OutputEvent::Keyboard { code, event_type } = events[0];
        assert_eq!(code, KeyboardCode::Up);
        assert_eq!(event_type, KeyboardEventType::Release);
    }

    #[test]
    fn test_dpad_direction_change() {
        let mut engine = MappingEngine::new_hardcoded();

        // Press up
        engine.process(&InputEvent::axis_move(AxisCode::DPadY, -1)).unwrap();

        // Change to down (should release up, press down)
        let events = engine.process(&InputEvent::axis_move(AxisCode::DPadY, 1)).unwrap();

        assert_eq!(events.len(), 2);

        let OutputEvent::Keyboard { code: code1, event_type: type1 } = events[0];
        assert_eq!(code1, KeyboardCode::Up);
        assert_eq!(type1, KeyboardEventType::Release);

        let OutputEvent::Keyboard { code: code2, event_type: type2 } = events[1];
        assert_eq!(code2, KeyboardCode::Down);
        assert_eq!(type2, KeyboardEventType::Press);
    }
}
