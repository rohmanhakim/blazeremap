# Architectural Decisions

This document records the major architectural and technology decisions made during the development of BlazeRemap. These decisions shape the system's constraints, performance characteristics, and future extensibility.

## Decision: Hexagonal Architecture (Ports and Adapters)
Context:
The application needs to interact with low-level Linux hardware APIs (`evdev`, `uinput`) while maintaining testable and maintainable business logic for remapping.

Decision:
We adopted a Hexagonal architecture where the core logic (`mapping`, `event`) is decoupled from the platform implementation (`platform/linux`). Communication happens through well-defined traits (`Gamepad`, `VirtualKeyboard`, `InputManager`).

Alternatives considered:
- **Monolithic design**: Mixing `evdev` calls directly with remapping logic. This was rejected because it makes unit testing impossible without real hardware.
- **Platform-specific crates**: Creating separate crates for Linux. This was deemed overkill for the current project scale.

Why this matters:
It allows the core remapping logic to be tested using mocks (via `mockall`). It also provides a clear path for potential future support of other platforms or input protocols without rewriting the mapping engine.

## Decision: Blocking Synchronous Event Loop
Context:
Low latency is the primary requirement for a gamepad remapper. Input lag must be minimized.

Decision:
The system uses a dedicated thread with a blocking `read_event` loop. The `evdev` file descriptor is polled synchronously to ensure the fastest possible path from hardware interrupt to virtual event synthesis.

Alternatives considered:
- **Async/Await (Tokio)**: While powerful, the overhead of an async runtime was considered unnecessary for a tool that typically manages a single device and a single event stream.
- **Worker Thread Pool**: Spawning threads per event. Rejected due to context switching overhead and ordering concerns.

Why this matters:
Synchronous execution provides deterministic latency and simplifies the implementation of state-dependent mapping (like axis tracking) without worrying about complex concurrency primitives.

## Decision: Linux Uinput for Virtual Input
Context:
The system must inject keystrokes into the OS as if they came from a physical keyboard.

Decision:
We use the Linux `uinput` kernel module via the `evdev` crate. This allows the creation of a virtual device that the kernel treats as a standard input source.

Alternatives considered:
- **X11/Wayland Synthetic Events**: These only work within the graphical session and are often blocked by security features in modern desktop environments (especially Wayland).
- **Direct Library Injection**: Using `LD_PRELOAD` to intercept input calls. Too fragile and application-specific.

Why this matters:
`uinput` works at the kernel level, meaning BlazeRemap works across TTYs, X11, Wayland, and even inside most containers or virtual machines, providing "true" hardware emulation.

## Decision: Internal State Tracking for Analog Axes
Context:
Gamepad D-pads and thumbsticks often report analog values or specific axis codes, while keyboards require discrete "Press" and "Release" signals.

Decision:
The `MappingEngine` maintains an internal `axis_states` map. It tracks the current position of axes to detect when they cross neutral thresholds or change directions, synthesizing the appropriate keyboard events dynamically.

Alternatives considered:
- **Stateless Mapping**: Simply sending a key press on every move. This results in "stuck" keys because the OS never receives a release signal.
- **Time-based Polling**: Checking axis state at intervals. Rejected because it introduces unnecessary lag and jitter.

Why this matters:
This ensures "Release Symmetry"—the invariant that every synthesized key press is eventually followed by a release, preventing ghost inputs or stuck keys in games.

## Decision: TOML for Profile Configuration
Context:
Users need a way to define and share custom controller mappings.

Decision:
Profiles are stored in TOML format. The `serde` and `toml` crates handle serialization and deserialization.

Alternatives considered:
- **JSON**: Easier for machines but harder for humans to edit manually without errors (lack of comments, strict syntax).
- **YAML**: Human-readable but can be complex and has known security/performance edge cases in many parsers.

Why this matters:
TOML is the standard configuration language for the Rust ecosystem. It provides a clean, hierarchy-friendly syntax that is easy for users to edit in any text editor.