#!/usr/bin/env bash
# Local headless plugin host test runner
# Usage: ./scripts/test-headless.sh [plugin_path] [--junit-output path]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Default plugin path
DEFAULT_PLUGIN_PATH="target/release/Periodic Instruments.vst3"
PLUGIN_PATH="$DEFAULT_PLUGIN_PATH"
JUNIT_OUTPUT="test-results-local.xml"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --junit-output)
            JUNIT_OUTPUT="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [plugin_path] [--junit-output path]"
            echo "  plugin_path: Path to VST3 bundle (default: $DEFAULT_PLUGIN_PATH)"
            echo "  --junit-output: Path for JUnit XML output (default: test-results-local.xml)"
            exit 0
            ;;
        -*)
            echo "Unknown option: $1"
            exit 1
            ;;
        *)
            PLUGIN_PATH="$1"
            shift
            ;;
    esac
done

echo "================================================================"
echo "PERIODIC INSTRUMENTS — LOCAL HEADLESS PLUGIN HOST TEST"
echo "================================================================"
echo "Project root: $PROJECT_ROOT"
echo "Plugin path:  $PLUGIN_PATH"
echo "JUnit output: $JUNIT_OUTPUT"
echo "================================================================"

cd "$PROJECT_ROOT"

# Check if plugin exists
if [[ ! -e "$PLUGIN_PATH" ]]; then
    echo "ERROR: Plugin not found at: $PLUGIN_PATH"
    echo "Build it first with: cargo build --release"
    exit 1
fi

# Check if we're in a git repo and get commit info
if git rev-parse --git-dir > /dev/null 2>&1; then
    COMMIT=$(git rev-parse --short HEAD)
    BRANCH=$(git rev-parse --abbrev-ref HEAD)
    echo "Git commit:  $COMMIT ($BRANCH)"
fi

# Run the test binary
# The headless_host test is compiled with --features headless
echo "Running headless tests..."
echo ""

# Use cargo test to build and run the integration test
# --features headless enables the test binary
# --test headless_host runs our specific test
# -- --test-threads=1 ensures sequential execution
# --nocapture shows output in real-time
cargo test --features headless --test headless_host -- \
    --test-threads=1 \
    --nocapture \
    --junit-output "$JUNIT_OUTPUT" \
    "$PLUGIN_PATH"

TEST_EXIT_CODE=$?

echo ""
echo "================================================================"
if [[ $TEST_EXIT_CODE -eq 0 ]]; then
    echo "✓ ALL TESTS PASSED"
else
    echo "✗ SOME TESTS FAILED (exit code: $TEST_EXIT_CODE)"
fi
echo "JUnit XML written to: $JUNIT_OUTPUT"
echo "================================================================"

exit $TEST_EXIT_CODE