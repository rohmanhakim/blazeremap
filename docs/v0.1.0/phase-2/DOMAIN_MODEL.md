# Domain Model

This document describes the core domain concepts, rules, and logic governing BlazeRemap. It focuses on the business logic of input remapping rather than the technical implementation details of Linux subsystems.

## Terminology Mapping

| Code Name | Business Meaning |
| :--- | :--- |
| **Gamepad** | A physical input device (controller) capable of generating button and axis signals. |
| **Virtual Keyboard** | A software-defined input device that simulates a physical keyboard to the operating system. |
| **Mapping** | A definition of a relationship between a specific gamepad signal and a keyboard response. |
| **Profile** | A named configuration containing a set of Mappings and behavior settings (e.g., vibration intensity). |
| **Input Event** | A normalized representation of a physical interaction (pressing a button, moving a stick). |
| **Output Event** | A synthesized system instruction (pressing a key, releasing a key). |
| **Axis** | An analog input providing a range of values (e.g., thumbsticks, triggers, D-pads). |
| **Button** | A digital binary input (Pressed or Released). |
| **Deadzone** | An "ignore zone" for analog values near neutral to prevent accidental or jittery input. |

## Core Entities

### 1. The Gamepad (Source)
The Gamepad represents the user's physical hardware. It is characterized by its capabilities (how many buttons, does it have rumble, does it have paddles) and its current state. The system treats a Gamepad as a stream of raw signals that must be normalized into a standard dialect.

### 2. The Virtual Keyboard (Sink)
The Virtual Keyboard is the system's presence in the OS input stack. It exists to receive synthesized instructions and "act" as if a human pressed a physical key. It is agnostic of where the instructions come from.

### 3. The Mapping Engine (Translator)
The core logic engine. It maintains a lookup table of "Rules." When an Input Event arrives, the Engine determines which (if any) Output Events should be generated. It is responsible for translating analog "range" data into digital "binary" signals when mapping sticks to keys.

### 4. The Profile (Configuration)
A persistent set of user preferences. A Profile defines the "personality" of the remapper for a specific session or game.

## State Machines & Lifecycle

### Axis-to-Keyboard State Machine
Unlike buttons, which have a 1:1 relationship with keys, analog axes require state tracking to simulate binary keys correctly.

1.  **Neutral State**: The axis is centered. No output is active.
2.  **Threshold Crossing (Active)**: When the axis moves beyond the deadzone into a defined direction (e.g., D-Pad Up), a **Press** event is emitted for the target key.
3.  **Directional Swap**: If the axis moves directly from Positive to Negative (bypassing a prolonged Neutral state), the system must emit a **Release** for the old direction and a **Press** for the new direction in the same processing cycle.
4.  **Threshold Crossing (Neutral)**: When the axis returns to the deadzone, a **Release** event is emitted for the active key.

### Session Lifecycle
1.  **Discovery**: The system scans for compatible hardware based on device capabilities (must have both buttons and axes).
2.  **Acquisition**: The system opens a communication channel to the hardware.
3.  **Transformation Loop**: A continuous cycle of `Read Physical -> Translate -> Write Virtual`.
4.  **Termination**: Triggered by user exit or hardware disconnection. The system ensures the virtual device is destroyed to prevent ghost inputs.

## Invariants & Business Rules

### Invariants
-   **Release Symmetry**: Every synthesized "Press" event must be followed by a "Release" event before that same key can be "Pressed" again. This prevents "stuck keys" in the operating system.
-   **Input Normalization**: Regardless of whether a gamepad identifies a button as `0x130` or `0x131`, it is mapped to a domain-standard `ButtonCode` (e.g., `South`) before reaching the Mapping Engine.
-   **Strict Ordering**: Events are processed FIFO (First-In, First-Out). The relative timing of inputs must be preserved to support rapid "chording" (holding multiple buttons).

### Business Rules
-   **Hardware Filtering**: Devices that identify as mice, keyboards, or audio devices are explicitly ignored during discovery, even if they report having buttons/axes, to prevent "remapping the remapper."
-   **Deadzone Enforcement**: Analog inputs must be filtered through a deadzone to prevent "stick drift" from triggering unintentional keyboard events.
-   **Unmapped Pass-through**: If a physical button has no mapping defined in the active Profile, it is silently discarded; it does not block other inputs or cause errors.