# BlazeRemap Integration Tests

## Test Types

### Unit Tests
Located in source files (`src/**/*.rs`).

Run with:
```bash
cargo test --lib
```

### Integration Tests
Located in `tests/types_test.rs`.

Run with:
```bash
cargo test --test types_test
```

### Hardware Tests
Located in `tests/hardware_test.rs`.

**Prerequisites:**
- Physical gamepad(s) connected
- User in `input` group: `sudo usermod -a -G input $USER`
- `/dev/input` devices readable

Run with:
```bash
# All hardware tests
cargo test --test hardware_test -- --ignored --nocapture

# Specific test
cargo test --test hardware_test test_dualshock4_detection -- --ignored --nocapture

# Or use the helper script
./scripts/test-hardware.sh
```

## What Each Test Does

### `test_detect_real_gamepad`
Verifies at least one gamepad is detected.

### `test_gamepad_info_validity`
Validates that detected gamepadss have:
- Non-empty names
- Valid device paths
- Non-zero vendor IDs
- Identified gamepad types

### `test_no_false_positives`
Ensures keyboards, mice, and other devices aren't detected as gamepads.

### `test_dualshock4_detection`
Specific test for DualShock 4 (skipped if not connected).

### `test_xbox_detection`
Specific test for Xbox controllers (skipped if not connected).

### `test_elite_paddle_detection`
Tests Elite controller paddle detection (skipped if not connected).

### `test_detection_performance`
Ensures detection completes in < 1 second.

### `test_repeated_detection`
Tests that multiple detections work and are fast.

## Troubleshooting

### "No gamepads detected"
- Is a gamepad connected and powered on?
- Check with: `ls -la /dev/input/event*`
- Try: `evtest` to see if the gamepad is visible

### "Permission denied"
```bash
# Add user to input group
sudo usermod -a -G input $USER

# Log out and back in, then verify
groups | grep input

# Or run tests with sudo (not recommended)
sudo cargo test --test hardware_test -- --ignored --nocapture
```

### "Test failed on CI"
Hardware tests are marked `#[ignore]` and won't run in CI by default.
