#!/bin/bash
set -e

echo "===================="
echo " Pre-Commit Checks"
echo "===================="
echo

echo "1. Formatting..."
cargo fmt -- --check || {
    echo "❌ Format check failed. Run: cargo fmt"
    exit 1
}

echo "2. Linting..."
cargo clippy -- -D warnings || {
    echo "❌ Clippy found issues"
    exit 1
}

echo "3. Unit tests..."
cargo test --lib --quiet || {
    echo "❌ Unit tests failed"
    exit 1
}

echo "4. Integration tests..."
cargo test --test types_test --quiet || {
    echo "❌ Integration tests failed"
    exit 1
}

echo "5. CLI tests..."
cargo test --test cli_test --quiet || {
    echo "❌ CLI tests failed"
    exit 1
}

echo "6. Build release..."
cargo build --release --quiet || {
    echo "❌ Release build failed"
    exit 1
}

echo
echo "✅ All checks passed! Ready to commit."
