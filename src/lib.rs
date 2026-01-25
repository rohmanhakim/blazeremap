// BlazeRemap - gamepad-to-keyboard remapping library
//! This is the main library crate for BlazeRemap.
//!
//! # Architecture
//!
//! BlazeRemap follows a hexagonal/ports-and-adapters architecture:
//!
//! - `device`: Core domain logic (gamepads, capabilities, traits)
//! - `platform`: Platform-specific implementations (Linux evdev)
//! - `cli`: User interface layer (CLI commands)
//! - `app`: Application composition and wiring

// Public modules
pub mod app;
pub mod cli;
pub mod event;
pub mod input;
pub mod mapping;
pub mod output;
pub mod platform;

// Re-export commonly used types
pub use input::gamepad::{Gamepad, GamepadInfo, GamepadType};
pub use input::{InputDetectionResult, InputManager};
