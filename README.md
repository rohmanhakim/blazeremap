# BlazeRemap

BlazeRemap is a high-performance Linux input remapper designed to translate physical gamepad signals into virtual keyboard events. By operating at the kernel level via `evdev` and `uinput`, it allows you to use any controller in games or applications that only support keyboard input, with near-zero latency.

## Current Features

- **Hexagonal Architecture**: Core remapping logic is decoupled from Linux-specific APIs, ensuring high testability and reliability.
- **Kernel-Level Emulation**: Uses `uinput` to create a virtual keyboard that is recognized globally across the OS (TTY, X11, and Wayland).
- **Stateful Axis Mapping**: Intelligently handles D-pad and analog movements to simulate binary key presses and releases without "stuck keys."
- **Low Latency**: Synchronous, blocking event loop designed for gaming, featuring microsecond-precision latency tracking.
- **Device Discovery**: Automatic detection of connected gamepads with hardware identification (Vendor/Product IDs).
- **TOML Profiles**: Simple, human-readable configuration for button and axis mappings.

## CLI Usage & Examples

### Detect Gamepads
List all compatible controllers connected to your system.
```bash
blazeremap detect
```
**Output Example:**
```text
Detecting gamepads...

Found 1 gamepad(s):

[0] Xbox Wireless Controller (/dev/input/event3)
 ├─ Type: Xbox One
 ├─ Vendor:
 │  ├─ ID: 045E
 │  └─ Name: Microsoft
 ├─ Product ID: 02EA
 └─ Capabilities:
    └─ Force Feedback
```

### Run Remapper
Start the remapping daemon using either auto-detection or a specific device path.
```bash
blazeremap run --device /dev/input/event3
```
**Output Example:**
```text
Opening device: /dev/input/event3
Loading hardcoded mappings...
Creating virtual keyboard...

BlazeRemap is now running!
Mappings:
  D-pad button → Arrow
  South button → S
  West button → A
  East button → D

[INFO] Stats: 100 events | avg: 42µs (0.04ms) | min: 12µs | max: 156µs
```

### Debug Events
Monitor raw input events from a device to verify button codes.
```bash
blazeremap read /dev/input/event3
```
**Output Example:**
```text
[   0.00000ms][Δ        0µs] Button(South, Pressed)
[  45.12000ms][Δ    45120µs] Button(South, Released)
```

### Test Virtual Keyboard
Verify that the `uinput` module is working correctly by emitting a space key every second.
```bash
blazeremap test-keyboard
```

## Planned Features

The following features are partially implemented in the codebase (structs/detection logic) or are on the immediate roadmap:

- [ ] **Force Feedback (Rumble) Support**: Logic for detecting rumble capabilities exists; wiring the output feedback loop is planned.
- [ ] **Xbox Elite Paddles**: Detection code for rear paddles is present; mapping support is pending.
- [ ] **Custom Deadzones**: Ability to define per-axis deadzones in the TOML profile to combat stick drift.
- [ ] **Macro Engine**: Support for simple sequences (e.g., mapping one button to `Alt+Tab`).
- [ ] **Multi-Device Support**: Running one daemon instance to manage multiple controllers simultaneously.
- [ ] **Cross-Platform Adapters**: While currently Linux-only, the architecture is designed to support Windows (RawInput) and macOS (IOKit) in the future.

## Requirements

- **Linux Kernel**: Requires `evdev` and `uinput` support.
- **Permissions**: You must have read/write access to `/dev/input/*` and `/dev/uinput`. This is typically achieved by adding your user to the `input` and `uinput` groups:
  ```bash
  sudo usermod -aG input,uinput $USER
  ```
