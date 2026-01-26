# System Overview

BlazeRemap is a Linux-based input remapping daemon designed to translate physical gamepad inputs—including buttons and axes—into virtual keyboard events. By intercepting raw hardware signals and synthesizing keystrokes through the kernel's uinput interface, the system enables users to utilize controllers in environments or applications that natively support only keyboard input. It prioritizes low-latency execution and a clean separation between hardware-specific event handling and domain-specific mapping logic.

## Non-goals
- **Reverse Remapping**: The system does not attempt to map keyboard or mouse inputs into virtual gamepad signals.
- **Complex Macro Scripting**: There is no support for timed sequences, combos, or conditional scripting; mappings are currently direct or state-based translations.
- **Graphical Configuration**: The system is strictly CLI-driven and does not provide a GUI for mapping management.
- **Cross-Platform Portability**: The architecture is fundamentally coupled to Linux-specific subsystems (`evdev` and `uinput`) and does not aim for Windows or macOS support.

## High-level Architecture
```text
[ Physical Gamepad ] -> (/dev/input/event*) -> [ Input Manager ]
                                                      |
                                                      v
[ Virtual Keyboard ] <- ( /dev/uinput ) <------- [ Event Loop ] <--- [ Mapping Engine ] <--- [ Profile ]
```

## Primary Data Flow
1. **Detection**: The Input Manager identifies and opens physical hardware devices via `evdev`.
2. **Ingestion**: The Event Loop polls the device, converting raw hardware interrupts into normalized domain events.
3. **Translation**: The Mapping Engine evaluates the normalized events against a loaded Profile (TOML-based) to determine the intended virtual output.
4. **Synthesis**: The Virtual Keyboard receives instructions to emit specific keycodes, which are injected into the kernel's input subsystem via `uinput`.

## Key Invariants
- **Axis State Persistence**: The system must track the state of analog axes (like D-Pads) to ensure that a "Release" event is synthesized when an axis returns to neutral or switches direction, preventing stuck keys.
- **Single Profile Active**: A single execution instance maintains exactly one active mapping profile per device.
- **Event Ordering**: Output events must be emitted in the same chronological order as their causal input events to maintain input integrity.

## Operational Constraints
- **Device Permissions**: The process requires read access to `/dev/input/` and write access to `/dev/uinput`, typically requiring specific group memberships (`input`, `uinput`) or elevated privileges.
- **Exclusive Access**: While the system reads gamepad events, it does not necessarily "grab" the device exclusively unless configured, meaning raw gamepad events may still reach other applications.
- **Kernel Dependency**: Relies on the availability of the `uinput` kernel module being loaded.

## Assumptions
- The hardware device is recognized by the Linux kernel as a standard `evdev` input device.
- The user has provided or is using a profile compatible with the specific layout of their controller.
- The host system provides a stable monotonic clock for latency tracking and event synchronization.