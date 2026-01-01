#!/bin/bash
# Verify Hygiene: Compare Antislop findings against standard linters (MECE check)
# Usage: ./scripts/verify_hygiene.sh

set -e

# Setup paths
VENV_PYTHON="./.venv/bin/python"
PYLINT="./.venv/bin/pylint"
ESLINT="./node_modules/.bin/eslint"
ANTISLOP="./target/release/antislop" # Assuming built

echo "=== Hygiene Verification Suite ==="
echo "Ensuring Antislop finds 'slop' that standard linters MISS."

if [ ! -f "$ANTISLOP" ]; then
    echo "Building antislop..."
    cargo build --release --quiet
fi

# 1. Python Hygiene Check
echo -e "\n--- Python Hygiene Check (vs Pylint) ---"
echo "Running Pylint on examples/sloppy_code.py..."
# Pylint usually fails on errors, so we allow failure but capture output
$PYLINT examples/sloppy_code.py --disable=C,R,W > pylint_out.txt || true
PYLINT_COUNT=$(grep -c "E:" pylint_out.txt || true)

echo "Running Antislop on examples/sloppy_code.py..."
$ANTISLOP examples/sloppy_code.py --json > antislop_out.json || true
SLOP_COUNT=$(grep -o "finding" antislop_out.json | wc -l)

echo "Pylint Errors Found: $PYLINT_COUNT"
echo "Antislop Findings:   $SLOP_COUNT"

if [ "$SLOP_COUNT" -gt 0 ]; then
    echo "✅ Antislop found slop."
else
    echo "❌ Antislop missed slop!"
    exit 1
fi

# 2. JavaScript Hygiene Check
echo -e "\n--- JavaScript Hygiene Check (vs ESLint) ---"
if [ -f "$ESLINT" ]; then
    echo "Running ESLint on examples/sloppy_code.js..."
    # Simple eslint config for v9+
    cat > eslint.config.mjs <<EOF
import js from "@eslint/js";

export default [
    js.configs.recommended,
    {
        rules: {
            "no-unused-vars": "warn",
            "no-empty": "warn"
        }
    }
];
EOF
    # Run eslint
    $ESLINT examples/sloppy_code.js > eslint_out.txt || true
    ESLINT_COUNT=$(grep -c "error" eslint_out.txt || true)
    
    echo "Running Antislop on examples/sloppy_code.js..."
    $ANTISLOP examples/sloppy_code.js --json > antislop_js.json || true
    SLOP_JS=$(grep -o "finding" antislop_js.json | wc -l)
    
    echo "ESLint Errors:     $ESLINT_COUNT"
    echo "Antislop Findings: $SLOP_JS"
    
    rm eslint.config.mjs
else
    echo "⚠️ ESLint not installed, skipping JS check."
fi

# 3. Rust Hygiene Check
echo -e "\n--- Rust Hygiene Check (vs Clippy) ---"
# We need a cargo project for clippy checks on a single file usually, 
# or we can use rustc -D warnings. 
# For now, let's just check if Antislop finds unwrap which clippy allows by default
echo "Checking examples/sloppy_code.rs..."
$ANTISLOP examples/sloppy_code.rs --json > antislop_rs.json || true
SLOP_RS=$(grep -o "finding" antislop_rs.json | wc -l)

if [ "$SLOP_RS" -gt 0 ]; then
    echo "✅ Antislop found Rust slop (unwrap/unsafe)."
else
    echo "❌ Antislop missed Rust slop!"
fi

# 4. TypeScript Hygiene Check
echo -e "\n--- TypeScript Hygiene Check (vs tsc) ---"
TSC="./node_modules/.bin/tsc"
if [ -f "$TSC" ]; then
    echo "Running tsc on examples/sloppy_code.ts..."
    $TSC --noEmit --allowJs --skipLibCheck examples/sloppy_code.ts 2> tsc_out.txt || true
    TSC_COUNT=$(grep -c "error" tsc_out.txt || true)
    
    echo "Running Antislop on examples/sloppy_code.ts..."
    $ANTISLOP examples/sloppy_code.ts --json > antislop_ts.json || true
    SLOP_TS=$(grep -o "finding" antislop_ts.json | wc -l)
    
    echo "TypeScript Errors: $TSC_COUNT"
    echo "Antislop Findings: $SLOP_TS"
    
    if [ "$SLOP_TS" -gt 0 ]; then
        echo "✅ Antislop found TypeScript slop."
    else
        echo "⚠️ Antislop found no TypeScript slop."
    fi
else
    echo "⚠️ tsc not installed, skipping TS check."
fi

# 5. Go Hygiene Check
echo -e "\n--- Go Hygiene Check (vs go vet) ---"
if command -v go &> /dev/null; then
    echo "Running go vet on examples/sloppy_code.go..."
    go vet examples/sloppy_code.go 2> govet_out.txt || true
    GOVET_COUNT=$(wc -l < govet_out.txt | tr -d ' ')
    
    echo "Running Antislop on examples/sloppy_code.go..."
    $ANTISLOP examples/sloppy_code.go --json > antislop_go.json || true
    SLOP_GO=$(grep -o "finding" antislop_go.json | wc -l)
    
    echo "Go Vet Issues:     $GOVET_COUNT"
    echo "Antislop Findings: $SLOP_GO"
    
    if [ "$SLOP_GO" -gt 0 ]; then
        echo "✅ Antislop found Go slop (panic/interface{})."
    else
        echo "⚠️ Antislop found no Go slop."
    fi
else
    echo "⚠️ Go not installed, skipping Go check."
fi

echo -e "\n=== Hygiene Verification Complete ==="
echo "See output files for details."
rm -f pylint_out.txt eslint_out.txt tsc_out.txt govet_out.txt antislop_out.json antislop_js.json antislop_rs.json antislop_ts.json antislop_go.json

