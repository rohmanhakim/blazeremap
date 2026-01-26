use crate::event::{AxisCode, AxisDirection, ButtonCode, KeyboardCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingRule {
    ButtonToKey { source: ButtonCode, target: KeyboardCode },
    AxisDirectionToKey { source: AxisCode, direction: AxisDirection, target: KeyboardCode },
}

impl MappingRule {
    pub fn button_to_key(source: ButtonCode, target: KeyboardCode) -> Self {
        Self::ButtonToKey { source, target }
    }

    pub fn axis_direction_to_key(
        source: AxisCode,
        direction: AxisDirection,
        target: KeyboardCode,
    ) -> Self {
        Self::AxisDirectionToKey { source, direction, target }
    }
}

#[cfg(test)]
mod tests {
    use crate::mapping::{MappingRule::AxisDirectionToKey, rules::MappingRule::ButtonToKey};

    use super::*;

    #[test]
    fn test_mapping_button_to_keyboard_creation() {
        let rule = MappingRule::button_to_key(ButtonCode::South, KeyboardCode::Space);

        assert_eq!(rule, ButtonToKey { source: ButtonCode::South, target: KeyboardCode::Space });
    }

    #[test]
    fn test_mapping_axis_to_keyboard_creation() {
        let rule = MappingRule::axis_direction_to_key(
            AxisCode::DPadY,
            AxisDirection::Positive,
            KeyboardCode::Up,
        );

        assert_eq!(
            rule,
            AxisDirectionToKey {
                source: AxisCode::DPadY,
                direction: AxisDirection::Positive,
                target: KeyboardCode::Up
            }
        );
    }

    #[test]
    fn test_mapping_rule_equality() {
        let rule1 = MappingRule::button_to_key(ButtonCode::South, KeyboardCode::Space);
        let rule2 = MappingRule::button_to_key(ButtonCode::South, KeyboardCode::Space);
        let rule3 = MappingRule::button_to_key(ButtonCode::East, KeyboardCode::E);

        assert_eq!(rule1, rule2);
        assert_ne!(rule1, rule3);
    }
}
