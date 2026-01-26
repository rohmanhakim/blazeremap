//! Event processing module
//!
//! Defines event types for gamepad input remapping.
//! /*

mod handler;
mod input;
mod output;
mod time;

pub use handler::EventLoop;
pub use input::types::*;
pub use output::types::*;
pub use time::*;
