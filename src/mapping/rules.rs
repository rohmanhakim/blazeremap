use crate::{event::ButtonCode, mapping::types::TargetType, output::types::KeyboardCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingRule {
    pub source_code: ButtonCode,
    pub target_code: KeyboardCode,
    pub target_type: TargetType,
}

impl MappingRule {
    pub fn keyboard(source_code: ButtonCode, target_code: KeyboardCode) -> Self {
        Self { source_code, target_code, target_type: TargetType::Keyboard }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_rule_keyboard_creation() {
        let rule = MappingRule::keyboard(ButtonCode::South, KeyboardCode::Space);

        assert_eq!(rule.source_code, ButtonCode::South);
        assert_eq!(rule.target_code, KeyboardCode::Space);
        assert_eq!(rule.target_type, TargetType::Keyboard);
    }

    #[test]
    fn test_mapping_rule_equality() {
        let rule1 = MappingRule::keyboard(ButtonCode::South, KeyboardCode::Space);
        let rule2 = MappingRule::keyboard(ButtonCode::South, KeyboardCode::Space);
        let rule3 = MappingRule::keyboard(ButtonCode::East, KeyboardCode::E);

        assert_eq!(rule1, rule2);
        assert_ne!(rule1, rule3);
    }
}
