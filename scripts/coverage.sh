#!/usr/bin/env bash
# Generate test coverage report using cargo-llvm-cov
# Requires: cargo install cargo-llvm-cov

set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  AntiSlop Coverage Report"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "cargo-llvm-cov is not installed."
    echo "Install with: cargo install cargo-llvm-cov"
    exit 1
fi

# Generate coverage
echo "Generating coverage report..."
cargo llvm-cov --all-features --html --output-dir coverage

echo ""
echo "Coverage report generated: coverage/html/index.html"
echo ""

# Also print summary to console
cargo llvm-cov --all-features --summary-only

echo ""
echo "Target: 80% line coverage"
