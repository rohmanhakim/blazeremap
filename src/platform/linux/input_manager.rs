// Linux device manager implementation
use super::errors::classify_error;
use super::gamepad::{LinuxGamepad, extract_gamepad_info, is_gamepad};
use crate::input::{InputDetectionResult, InputDeviceError, InputManager, gamepad::Gamepad};

pub struct LinuxInputManager {
    // Fields can be added later if needed
}

impl LinuxInputManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LinuxInputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputManager for LinuxInputManager {
    fn list_gamepads(&self) -> anyhow::Result<InputDetectionResult> {
        use evdev::enumerate;

        let devices: Vec<_> = enumerate().collect();

        println!("Found {} input devices total", devices.len());

        let mut result = InputDetectionResult { gamepad_info: Vec::new(), errors: Vec::new() };

        for (path, device) in devices {
            if is_gamepad(&device) {
                let path_str = path.to_string_lossy().to_string();
                match extract_gamepad_info(&device, &path_str) {
                    Ok(info) => {
                        println!(
                            "✓ Detected: {} ({}) - {:?}",
                            info.name, info.gamepad_type, info.capabilities
                        );
                        result.gamepad_info.push(info);
                    }
                    Err(err) => {
                        let error_type = classify_error(&err);
                        let device_err = InputDeviceError::new(path_str, error_type, err);
                        println!("✗ Error: {}", device_err);
                        result.errors.push(device_err);
                    }
                }
            }
        }

        tracing::info!(
            "Found {} gamepads ({} errors)",
            result.gamepad_info.len(),
            result.errors.len()
        );

        Ok(result)
    }

    fn open_gamepad(&self, path: &str) -> anyhow::Result<Box<dyn Gamepad>> {
        let gamepad = LinuxGamepad::open(path)?;
        Ok(Box::new(gamepad))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        let manager = LinuxInputManager::new();
        let result = manager.list_gamepads();

        assert!(result.is_ok());

        println!("Result: {:?}", result);
    }
}
