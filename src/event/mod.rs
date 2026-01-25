//! Event processing module
//!
//! Defines event types for gamepad input remapping.

mod input;
mod output;
mod time;
mod types;

pub use input::types::InputEvent;
pub use output::types::{KeyboardCode, KeyboardEventType, OutputEvent};
pub use time::*;
pub use types::*;
