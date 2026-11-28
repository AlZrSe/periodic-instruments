# ADR 0002: Headless Plugin Host Selection for CI Testing

## Status
Accepted

## Context
We need a headless plugin host to run fast smoke tests (< 2 min) on every CI push/PR for the Periodic Instruments VST3 plugin. The host must:
- Load and instantiate the VST3 plugin
- Exercise audio processing at various block sizes and sample rates
- Test parameter enumeration, get/set, automation
- Test state save/restore
- Test MIDI event handling
- Test editor/view creation (headless)
- Provide process isolation (plugin crash doesn't kill test runner)
- Output JUnit XML for CI integration
- Work on Windows and Linux
- Be pinned to exact version with SHA256 verification

## Decision
We select **`vst3-host` (crates.io, version 0.11)** as our headless plugin host, used via a custom test binary (`tests/headless_host.rs`).

### Options Evaluated

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| **Steinberg `VST3PluginTestHost`** | Official, comprehensive | Large binary (~50MB), no Rust API, hard to pin/verify checksums, limited process isolation | ❌ Rejected |
| **JUCE `AudioPluginHost` headless** | Mature, widely used | C++, large dependency, complex build, no Rust integration | ❌ Rejected |
| **Custom Rust host using `vst3-sys`/`nih-plug`** | Full control, Rust-native | Significant development effort, maintenance burden | ⚠️ Considered |
| **`vst3-host` crate** | Rust-native, process isolation built-in, small (~5MB), active maintenance, async API, JUnit-friendly | Newer crate, less battle-tested | ✅ **Selected** |

### Why `vst3-host`?
1. **Rust-native**: Integrates seamlessly with our Rust codebase and Cargo workflow
2. **Process isolation**: Built-in `process-isolation` feature spawns plugins in separate processes — crashes won't kill the test runner
3. **Small footprint**: ~5MB binary vs 50MB+ for Steinberg/JUCE hosts
4. **Async API**: Works naturally with Tokio for timeouts and concurrent test execution
5. **Cross-platform**: Supports Windows, Linux, macOS
6. **Pin-able**: Available on crates.io with exact version pinning and Cargo.lock checksums
7. **VST3 focused**: Purpose-built for VST3, no CLAP/AU baggage
8. **Active maintenance**: Recent releases, responsive maintainer

## Consequences

### Positive
- Minimal new dependencies (only `vst3-host` + test utilities as dev-dependencies)
- Fast CI execution (< 30s expected for full test suite)
- Process isolation meets crash-safety requirements
- JUnit XML generation via `junit-report` crate
- Local parity: same binary runs in CI and locally via `cargo test --features headless`

### Negative
- Newer crate (0.11) — less ecosystem maturity than Steinberg/JUCE hosts
- API may evolve; we pin to exact version
- Requires Rust toolchain in CI (already present)

### Risks & Mitigations
| Risk | Mitigation |
|------|------------|
| `vst3-host` has bugs with our plugin | Fallback: implement minimal custom host using `vst3` crate |
| Process isolation doesn't work on some CI runners | Test on both Windows/Linux; document known limitations |
| API changes in future versions | Pin exact version in `Cargo.lock`; only upgrade deliberately |

## Implementation Details

### Test Binary (`tests/headless_host.rs`)
- Uses `vst3-host` with `process-isolation` feature
- Loads `target/release/Periodic Instruments.vst3`
- Runs test suites matching acceptance criteria (AC-023 through AC-030)
- Outputs JUnit XML to `test-results.xml`
- Prints human-readable summary to stdout
- Exit code 0 on all pass, 1 on any failure

### CI Integration (`.github/workflows/ci.yml`)
- Matrix: `windows-latest`, `ubuntu-latest`
- Steps: checkout → setup Rust → cache Cargo → build release → run headless tests
- Timeout: 30s per test case, 120s total job
- Artifacts: JUnit XML always, logs/dumps on failure
- Retry: 1 retry on transient failures

### Local Developer Parity
- Command: `cargo test --features headless` or `./scripts/test-headless.sh`
- Downloads/caches same `vst3-host` version via Cargo
- Produces `test-results-local.xml`
- Works offline after first run (Cargo cache)

## Version Pinning
- `vst3-host = "0.11"` in `[dev-dependencies]`
- Exact version resolved in `Cargo.lock` with SHA256 checksums
- No binary committed to repo — fetched at build time via Cargo

## References
- `vst3-host` crate: https://crates.io/crates/vst3-host
- `vst3-host` repo: https://github.com/HelgeSverre/rust-vst3-host
- Acceptance criteria: `specs/task-021-headless-plugin-host/acceptance-criteria.md`
- Test scenarios: `specs/task-021-headless-plugin-host/test-scenarios.md`