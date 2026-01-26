// Virtual Keyboard Module

use crate::{
    event::KeyboardCode, output::keyboard::VirtualKeyboard,
    platform::linux::converter::keyboard_code_to_evdev_key,
};
use anyhow::{Context, Result};
use evdev::{AttributeSet, EventType, InputEvent as EvdevEvent, KeyCode, uinput::VirtualDevice};
use std::path::PathBuf;

/// Concrete virtual keyboard backed by /dev/uinput
pub struct LinuxVirtualKeyboard {
    device: VirtualDevice,
}

impl LinuxVirtualKeyboard {
    /// Create a new virtual keyboard device
    pub fn new(name: &str) -> Result<Self> {
        // Build a key set including all common keyboard keys
        let mut keys = AttributeSet::<KeyCode>::new();
        for code in KeyCode::KEY_ESC.code()..=KeyCode::KEY_MICMUTE.code() {
            keys.insert(KeyCode::new(code));
        }

        // Build virtual device
        let device = VirtualDevice::builder()?
            .name(name)
            .with_keys(&keys)?
            .build()
            .context("Failed to create virtual keyboard")?;

        tracing::info!("Virtual keyboard created: {}", name);

        Ok(Self { device })
    }

    // Low-level helpers operating on key codes
    fn press_key_code(&mut self, code: u16) -> Result<()> {
        let key = KeyCode::new(code);
        self.device.emit(&[
            EvdevEvent::new(EventType::KEY.0, key.code(), 1),
            EvdevEvent::new(EventType::SYNCHRONIZATION.0, 0, 0),
        ])?;
        Ok(())
    }

    fn release_key_code(&mut self, code: u16) -> Result<()> {
        let key = KeyCode::new(code);
        self.device.emit(&[
            EvdevEvent::new(EventType::KEY.0, key.code(), 0),
            EvdevEvent::new(EventType::SYNCHRONIZATION.0, 0, 0),
        ])?;
        Ok(())
    }

    fn tap_key_code(&mut self, code: u16) -> Result<()> {
        self.press_key_code(code)?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.release_key_code(code)?;
        Ok(())
    }

    pub fn sys_path(&mut self) -> Result<PathBuf> {
        self.device.get_syspath().context("Failed to get device sysfs path")
    }
}

impl Drop for LinuxVirtualKeyboard {
    fn drop(&mut self) {
        // Cleanup handled by UInputDevice drop
    }
}

// Implement the domain trait for this concrete type
impl VirtualKeyboard for LinuxVirtualKeyboard {
    fn press_key(&mut self, code: KeyboardCode) -> Result<()> {
        self.press_key_code(keyboard_code_to_evdev_key(code).code())
    }

    fn release_key(&mut self, code: KeyboardCode) -> Result<()> {
        self.release_key_code(keyboard_code_to_evdev_key(code).code())
    }

    fn tap_key(&mut self, code: KeyboardCode) -> Result<()> {
        self.tap_key_code(keyboard_code_to_evdev_key(code).code())
    }
    fn sys_path(&mut self) -> Result<std::path::PathBuf> {
        self.sys_path()
    }
}
