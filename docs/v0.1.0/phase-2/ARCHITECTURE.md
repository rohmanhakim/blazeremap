# Architecture

BlazeRemap is structured following a **Ports and Adapters (Hexagonal)** architecture. This design decouples the core remapping logic from the low-level Linux input subsystems, facilitating testability through mocks and ensuring that business rules remain isolated from I/O details.

## Module Responsibilities

- **Domain Core (`event`, `mapping`)**: Contains the "pure" logic and definitions. `event` defines the shared language (Input/Output events) and the orchestrating `EventLoop`. `mapping` contains the stateless engine that transforms signals based on configuration profiles.
- **Port Interfaces (`input`, `output`)**: Defines the contracts (traits) for hardware interaction. These modules represent the "Ports" that the core uses to communicate with the outside world without knowing the implementation details.
- **Platform Adapters (`platform`)**: Implements the port traits for specific environments. Currently, this houses the Linux-specific `evdev` (reader) and `uinput` (writer) logic.
- **User Interface (`cli`)**: The driver layer that interprets user commands via `clap` and provides feedback.
- **Composition Root (`app`, `main`)**: Responsible for bootstrapping the application, performing dependency injection by instantiating concrete platform adapters and passing them into the core event loop.

## Ownership Boundaries

- **The Event Loop**: Acts as the primary owner of the active session. It holds ownership of the `MappingEngine` and unique ownership (via `Box<dyn T>`) of the input and output device handles.
- **The Input Manager**: Responsible for the lifecycle of device discovery. It produces owned `Gamepad` instances but does not retain ownership of them after they are opened.
- **Mapping Engine**: Owns the state of the current transformation rules and the transient state of analog axes (to handle direction-change logic).

## Dependency Direction Rules

Dependencies flow **inward** toward the domain logic:
1. `main` / `app` -> `cli`
2. `cli` -> `platform`, `mapping`, `event`
3. `platform` -> `input`, `output`, `event`
4. `event` -> `mapping`
5. `mapping` is a leaf dependency (depends only on internal types).

## Allowed vs. Forbidden Couplings

- **Allowed**: The `platform` module is allowed to depend on `evdev` and `nix` crates; it is the only module where hardware-specific types (like `evdev::Device`) should exist.
- **Forbidden**: The `mapping` module must never depend on `platform` or any I/O-related crates. It must remain a pure function of `InputEvent -> Vec<OutputEvent>`.
- **Forbidden**: Core domain types in `event` must not have dependencies on the `cli` or argument parsing logic.

## Cross-Cutting Concerns

- **Logging & Tracing**: Handled via the `tracing` crate. The `EventLoop` records latency metrics and event flows, which are collected by the `tracing-subscriber` initialized in `main`.
- **Error Handling**: Uses `thiserror` for defining structured, recoverable errors in the library modules and `anyhow` for context-rich error reporting in the `cli` and `app` layers.
- **Telemetry**: The `EventLoop` maintains internal counters for event frequency and processing latency, periodically logging performance statistics to help diagnose lag in the input pipeline.