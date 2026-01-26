#!/bin/bash
# Run hardware integration tests

echo "🎮 BlazeRemap Hardware Integration Tests"
echo "========================================"
echo ""
echo "Prerequisites:"
echo "  • At least one gamepad connected"
echo "  • User in 'input' group"
echo "  • /dev/input devices accessible"
echo ""

# Check if user is in input group
if ! groups | grep -q input; then
    echo "⚠️  WARNING: User not in 'input' group"
    echo "   Run: sudo usermod -a -G input $USER"
    echo ""
fi

# Run tests
echo "Running hardware tests..."
echo ""

cargo test --test detect_hardware_test -- --ignored --nocapture
cargo test --test latency_hardware_test -- --ignored --nocapture

echo ""
echo "✅ Hardware tests complete!"
