# BlazeRemap

Linux keyboard-to-gamepad remapping software.

## Features

- ✅ Gamepad detection (Xbox, PlayStation, generic gamepads)
- ✅ Vendor identification (Microsoft, Sony, etc.)
- ✅ Capability detection (Force Feedback, Elite Paddles)
- ✅ Command-line interface
- ⏳ Button remapping (Phase 2)
- ⏳ Profile management (Phase 2)

## Requirements

- Linux kernel 5.15+
- User in `input` group
- Rust 1.70+ (for building)

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/rohmanhakim/blazeremap
cd blazeremap

# Build
cargo build --release

# Install (optional)
cargo install --path .
```

### Setup Permissions
```bash
# Add user to input group
sudo usermod -a -G input $USER

# Create udev rule for uinput
echo 'KERNEL=="uinput", MODE="0660", GROUP="uinput"' | \
  sudo tee /etc/udev/rules.d/99-blazeremap.rules

# Load uinput module
sudo modprobe uinput
echo "uinput" | sudo tee -a /etc/modules

# Log out and back in for group changes to take effect
```

## Usage

### Detect Gamepads
```bash
blazeremap detect

# With verbose output
blazeremap detect --verbose
```

### Example Output
```
Detecting gamepads...

Found 28 input devices total
Found gamepad: Wireless Controller
✓ Detected: Wireless Controller (DualShock 4) - [ForceFeedback]
Found 1 gamepads (0 errors)

Found 1 gamepad(s):

[0] Wireless Controller (/dev/input/event25)
 ├─ Type: DualShock 4
 ├─ Vendor:
 │  ├─ ID: 054C
 │  └─ Name: Sony
 ├─ Product ID: 09CC
 └─ Capabilities:
    └─ Force Feedback
```

## Development

### Running Tests
```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test types_test

# Hardware tests (requires gamepad connected)
cargo test --test hardware_test -- --ignored --nocapture

# All tests
./test_hardware.sh
```

### Project Structure
```
src/
├── main.rs              # Binary entry point
├── lib.rs               # Library root
├── app.rs               # App composition
├── cli/                 # CLI commands
│   ├── mod.rs
│   └── detect.rs
├── device/              # Device abstractions
│   ├── mod.rs
│   ├── manager.rs
│   └── gamepad/
└── platform/            # Platform implementations
    ├── mod.rs
    └── linux/
        ├── mod.rs
        ├── errors.rs
        ├── gamepad.rs
        └── device_manager.rs
```

## Supported Gamepads

| Gamepad | Detection | Capabilities |
|------------|-----------|--------------|
| Xbox One | ✅ | Force Feedback |
| Xbox Series X/S | ✅ | Force Feedback |
| Xbox Elite | ✅ | Force Feedback, Paddles |
| DualShock 4 | ✅ | Force Feedback |
| DualSense (PS5) | ✅ | Force Feedback |
| Generic HID | ✅ | Varies |
