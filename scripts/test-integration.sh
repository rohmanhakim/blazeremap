#!/bin/bash
# scripts/test_integration.sh

set -e

echo "================================================"
echo "BlazeRemap Virtual Keyboard Integration Tests"
echo "================================================"
echo ""

# Check for root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Error: Integration tests require root access"
    echo "   Run with: sudo ./scripts/test_integration.sh"
    exit 1
fi

# Check for /dev/uinput
if [ ! -e /dev/uinput ]; then
    echo "❌ Error: /dev/uinput not found"
    echo "   Run: sudo modprobe uinput"
    exit 1
fi

echo "✓ Running as root"
echo "✓ /dev/uinput available"
echo ""

# Run tests
echo "Running integration tests..."
cargo test --test virtual_keyboard_integration_test -- --ignored --nocapture

echo ""
echo "================================================"
echo "✓ All integration tests passed!"
echo "================================================"
