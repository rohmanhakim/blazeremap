// Linux device manager implementation
use super::controller::{extract_controller_info, is_game_controller};
use super::errors::classify_error;
use crate::device::{DetectionResult, DeviceError, DeviceManager};

pub struct LinuxDeviceManager {
    // Fields can be added later if needed
}

impl LinuxDeviceManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LinuxDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager for LinuxDeviceManager {
    fn list_controllers(&self) -> anyhow::Result<DetectionResult> {
        use evdev::enumerate;

        let devices: Vec<_> = enumerate().collect();

        println!("Found {} input devices total", devices.len());

        let mut result = DetectionResult { controller_info: Vec::new(), errors: Vec::new() };

        for (path, device) in devices {
            if is_game_controller(&device) {
                let path_str = path.to_string_lossy().to_string();
                match extract_controller_info(&device, &path_str) {
                    Ok(info) => {
                        println!(
                            "✓ Detected: {} ({}) - {:?}",
                            info.name, info.controller_type, info.capabilities
                        );
                        result.controller_info.push(info);
                    }
                    Err(err) => {
                        let error_type = classify_error(&err);
                        let device_err = DeviceError::new(path_str, error_type, err);
                        println!("✗ Error: {}", device_err);
                        result.errors.push(device_err);
                    }
                }
            }
        }

        println!(
            "Found {} controllers ({} errors)",
            result.controller_info.len(),
            result.errors.len()
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        let manager = LinuxDeviceManager::new();
        let result = manager.list_controllers();

        assert!(result.is_ok());

        println!("Result: {:?}", result);
    }
}
