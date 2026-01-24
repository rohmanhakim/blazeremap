use anyhow::Result;

/// Domain trait: abstract virtual keyboard operations
pub trait VirtualKeyboard {
    /// Press a key by its code
    fn press_key(&mut self, code: u16) -> Result<()>;
    /// Release a key by its code
    fn release_key(&mut self, code: u16) -> Result<()>;
    /// Tap a key (press then release)
    fn tap_key(&mut self, code: u16) -> Result<()>;
    /// Get sysfs path (for debugging)
    fn sys_path(&mut self) -> Result<std::path::PathBuf>;
}
