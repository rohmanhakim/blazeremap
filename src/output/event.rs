use std::fmt::{Display, Formatter, Result};

use crate::output::types::{KeyboardCode, KeyboardEventType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputEvent {
    Keyboard {
        code: KeyboardCode,
        event_type: KeyboardEventType, // press, release, hold
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    Keyboard,
    Mouse,
    Gamepad,
}

impl Display for OutputEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Keyboard { code, event_type } => {
                write!(f, "Keyboard: {:?} ({:?})", code, event_type)
            }
        }
    }
}
