#!/usr/bin/env bash
# Local pluginval test runner for Linux/macOS
# Usage: ./scripts/run-pluginval.sh [vst3-path] [strictness-level] [pluginval-version]
#
# Examples:
#   ./scripts/run-pluginval.sh
#   ./scripts/run-pluginval.sh target/release/Periodic\ Instruments.vst3 5
#   ./scripts/run-pluginval.sh target/release/Periodic\ Instruments.vst3 7 v0.3.8

set -euo pipefail

VST3_PATH="${1:-target/release/Periodic Instruments.vst3}"
STRICTNESS_LEVEL="${2:-7}"
PLUGINVAL_VERSION="${3:-v0.3.8}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="$PROJECT_ROOT/.cache/pluginval-$PLUGINVAL_VERSION"
PLUGINVAL_DIR="$CACHE_DIR/linux"
PLUGINVAL_EXE="$PLUGINVAL_DIR/pluginval"

echo "=== pluginval Local Test Runner ==="
echo "VST3 Path: $VST3_PATH"
echo "Strictness Level: $STRICTNESS_LEVEL"
echo "Pluginval Version: $PLUGINVAL_VERSION"
echo "Cache Dir: $CACHE_DIR"
echo ""

# Check if VST3 bundle exists
if [ ! -d "$VST3_PATH" ]; then
    echo "ERROR: VST3 bundle not found at '$VST3_PATH'"
    echo "Build the plugin first with: cargo build --release"
    exit 1
fi

# Download and install pluginval if not cached
if [ ! -f "$PLUGINVAL_EXE" ]; then
    echo "Downloading pluginval $PLUGINVAL_VERSION for Linux..."
    mkdir -p "$PLUGINVAL_DIR"
    URL="https://github.com/steinbergmedia/vst3sdk/releases/download/$PLUGINVAL_VERSION/pluginval-linux.zip"
    echo "URL: $URL"
    curl -L -o "$PLUGINVAL_DIR/pluginval.zip" "$URL"
    unzip -o "$PLUGINVAL_DIR/pluginval.zip" -d "$PLUGINVAL_DIR"
    PLUGINVAL_EXE=$(find "$PLUGINVAL_DIR" -name "pluginval" -type f | head -1)
    if [ -z "$PLUGINVAL_EXE" ]; then
        echo "ERROR: pluginval not found after extraction"
        exit 1
    fi
    chmod +x "$PLUGINVAL_EXE"
    echo "Pluginval installed to: $PLUGINVAL_EXE"
else
    echo "Using cached pluginval: $PLUGINVAL_EXE"
fi

# Check for Xvfb (required for headless GUI tests on Linux)
if command -v xvfb-run >/dev/null 2>&1; then
    XVFB_CMD="xvfb-run -a"
    echo "Xvfb available - running headless"
else
    XVFB_CMD=""
    echo "WARNING: xvfb-run not found - GUI tests may fail on headless systems"
fi

# Run pluginval
echo ""
echo "=== Running pluginval (strictness level $STRICTNESS_LEVEL) ==="
$XVFB_CMD "$PLUGINVAL_EXE" --strictness-level "$STRICTNESS_LEVEL" "$VST3_PATH"
EXIT_CODE=$?

echo ""
echo "=== pluginval exited with code: $EXIT_CODE ==="
exit $EXIT_CODE