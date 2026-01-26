use anyhow::Result;

use crate::event::KeyboardCode;

/// Domain trait: abstract virtual keyboard operations
#[cfg_attr(test, mockall::automock)]
pub trait VirtualKeyboard {
    /// Press a key by its code
    fn press_key(&mut self, code: KeyboardCode) -> Result<()>;
    /// Release a key by its code
    fn release_key(&mut self, code: KeyboardCode) -> Result<()>;
    /// Tap a key (press then release)
    fn tap_key(&mut self, code: KeyboardCode) -> Result<()>;
    /// Get sysfs path (for debugging)
    fn sys_path(&mut self) -> Result<std::path::PathBuf>;
}
