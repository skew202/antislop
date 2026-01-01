#!/usr/bin/env bash
# QA Script for antislop
# Run this before committing to ensure all quality checks pass

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Antislop QA Suite"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Format check
echo -n "📝 Checking format... "
if cargo fmt --all -- --check 2>/dev/null; then
    echo -e "${GREEN}✓ Pass${NC}"
else
    echo -e "${RED}✗ Fail${NC}"
    echo "  Run: cargo fmt --all"
    exit 1
fi

# Clippy
echo -n "🔍 Running clippy... "
if cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    echo -e "${GREEN}✓ Pass${NC}"
else
    echo -e "${RED}✗ Fail${NC}"
    exit 1
fi

# Tests
echo -n "🧪 Running tests... "
if cargo test --all-features --quiet 2>/dev/null; then
    echo -e "${GREEN}✓ Pass${NC}"
else
    echo -e "${RED}✗ Fail${NC}"
    exit 1
fi

# Doc generation
echo -n "📚 Building docs... "
if cargo doc --all-features --no-deps --quiet 2>/dev/null; then
    echo -e "${GREEN}✓ Pass${NC}"
else
    echo -e "${RED}✗ Fail${NC}"
    exit 1
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "  ${GREEN}All checks passed!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
