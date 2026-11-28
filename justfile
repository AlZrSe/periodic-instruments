# Justfile for Periodic Instruments
# Run `just` to see available recipes

# Default recipe
default: help

# Help
help:
	@just --list

# --- Build ---

# Build release VST3 plugin
build:
	cargo build --release

# Build with debug symbols
build-debug:
	cargo build --profile release-with-debug

# Clean build artifacts
clean:
	cargo clean

# --- Test ---

# Run unit tests
test:
	cargo test --all-features

# Run headless plugin host tests (requires built plugin)
test-headless: build
	./scripts/test-headless.sh

# Run headless tests with custom plugin path
test-headless-custom:
	@echo "Usage: just test-headless-custom PLUGIN_PATH=/path/to/plugin.vst3"
	./scripts/test-headless.sh '{{PLUGIN_PATH}}'

# Run headless tests and open JUnit XML in browser (if xmllint available)
test-headless-report: test-headless
	@if command -v xmllint >/dev/null 2>&1; then \
		xmllint --format test-results-local.xml | less; \
	else \
		cat test-results-local.xml; \
	fi

# Run headless tests on Windows
test-headless-windows: build
	scripts\test-headless.bat

# --- CI Simulation ---

# Simulate CI build and test locally (Linux-like)
ci-local: build test test-headless clippy fmt

# --- Code Quality ---

# Run clippy with all features
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
fmt:
	cargo fmt --all --check

# Fix formatting
fmt-fix:
	cargo fmt --all

# Run security audit
audit:
	cargo install cargo-audit --locked 2>/dev/null || true
	cargo audit

# --- Documentation ---

# Build and open docs
docs:
	cargo doc --all-features --no-deps --open

# --- Release ---

# Check version in Cargo.toml
version:
	@grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Tag release (usage: just tag version=0.1.0)
tag:
	git tag -a "v{{version}}" -m "Release v{{version}}"
	git push origin "v{{version}}"

# --- Development ---

# Watch for changes and rebuild (requires cargo-watch)
watch:
	cargo install cargo-watch --locked 2>/dev/null || true
	cargo watch -x "build --release"

# Run with debug logging
run-debug:
	RUST_LOG=periodic_instruments=debug cargo run --release 2>&1 | head -100

# --- VST3 Validation ---

# Run pluginval if available (separate task #20)
pluginval:
	@echo "pluginval integration is Task #20 - not yet implemented"
	@echo "See: https://github.com/steinbergmedia/vst3pluginval"

# --- Cross-platform ---

# Check Windows target
check-windows:
	rustup target add x86_64-pc-windows-msvc
	cargo check --target x86_64-pc-windows-msvc

# Check Linux target
check-linux:
	rustup target add x86_64-unknown-linux-gnu
	cargo check --target x86_64-unknown-linux-gnu

# --- Utilities ---

# Show dependency tree
deps:
	cargo tree --all-features

# Check for outdated dependencies
outdated:
	cargo install cargo-outdated --locked 2>/dev/null || true
	cargo outdated

# Show crate size breakdown
size:
	cargo install cargo-bloat --locked 2>/dev/null || true
	cargo bloat --release --crates