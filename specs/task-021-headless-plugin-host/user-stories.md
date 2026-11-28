# User Stories — Task 021: CI with Headless Plugin Host for Testing Ready VST3

## Context
This task adds CI integration using a headless plugin host (hph) for fast smoke testing of the built VST3 bundle. It complements the deeper pluginval validation (Issue #20) by providing rapid feedback (< 2 min) on every commit/PR. The foundation plugin (Issue #1) builds and loads with 5 parameters and silence output — this task ensures that basic plugin viability is verified automatically in CI on every push.

---

## User Stories

### US-021.1: Developer Fast Feedback Loop
**As a** developer working on the plugin  
**I want** a headless plugin host to run in CI on every push/PR  
**So that** I get fast feedback (< 2 min) on whether the VST3 bundle loads, initializes, processes audio, and handles parameters — without waiting for full pluginval runs

**Details:**
- Must run on GitHub Actions (Windows `windows-latest`, Linux `ubuntu-latest`)
- Must complete in < 120 seconds total (build + test)
- Must fail the CI job if plugin crashes, fails to load, or produces NaN/Inf output
- Should run alongside (not instead of) pluginval; pluginval remains the deep validation gate

---

### US-021.2: CI Engineer Reliable Headless Host Selection
**As a** CI/CD engineer  
**I want** a well-defined, pinned headless plugin host with verified checksums  
**So that** CI runs are reproducible, secure, and don't break due to upstream changes

**Details:**
- Options: Steinberg `VST3PluginTestHost` (CLI), JUCE `AudioPluginHost` headless, or custom Rust host using `vst3-sys`/`nih-plug`
- Must pin exact version/commit/hash
- Must verify SHA256 checksum on download
- Must cache downloaded host binary between CI runs
- Must work on both Windows (`.exe`) and Linux (binary)

---

### US-021.3: QA Engineer Comprehensive Smoke Test Coverage
**As a** QA engineer  
**I want** the headless host to exercise critical plugin pathways  
**So that** regressions in loading, processing, parameters, state, MIDI, and editor are caught early

**Details:**
Test matrix must cover:
- Plugin discovery and instantiation
- Audio processing at variable block sizes (64, 128, 256, 512, 1024, 2048, 4096)
- Audio processing at variable sample rates (44100, 48000, 88200, 96000, 176400, 192000)
- Parameter enumeration, get/set, automation simulation
- State save/restore (preset chunk)
- MIDI note on/off, CC, pitch bend handling
- Editor/view creation and destruction (headless, no GPU)
- Process isolation: plugin crash doesn't kill test runner

---

### US-021.4: On-Call Engineer CI Stability
**As an** on-call engineer  
**I want** the headless host CI job to be flake-resistant and debuggable  
**So that** I don't get paged for false positives and can diagnose real failures quickly

**Details:**
- Retry logic for transient failures (max 1 retry)
- Artifact upload on failure: host logs, crash dumps, minidumps
- Clear pass/fail output with JUnit XML for GitHub Actions annotations
- Timeout per test case (30s default) to prevent hung jobs
- Runs in separate process from test runner (crash isolation)

---

### US-021.5: Local Developer Parity
**As a** developer  
**I want** to run the exact same headless test suite locally  
**So that** I can reproduce CI failures and iterate quickly without pushing

**Details:**
- Single command: `cargo test --features headless` or `just test-headless` or `./scripts/test-headless.sh`
- Downloads/caches host binary locally (same version as CI)
- Produces same output format (JUnit XML, human-readable summary)
- Works offline after first run (cached binary)

---

## Non-Functional Requirements

| Requirement | Detail |
|-------------|--------|
| **CI runtime** | < 120 seconds total (including host download/cache) on GitHub Actions |
| **Cross-platform** | Windows (x86_64-pc-windows-msvc) and Linux (x86_64-unknown-linux-gnu) |
| **Host binary** | Pinned version, SHA256 verified, cached (~5-20 MB) |
| **Process isolation** | Plugin runs in child process; crash → test failure, not runner crash |
| **Test output** | JUnit XML + human summary; artifact upload on failure |
| **Rust MSRV** | Stable Rust (same as project) |
| **Dependencies** | No new runtime deps for plugin; test-only deps OK |

---

## Out of Scope for This Task
- Full pluginval integration (Issue #20 — complementary, deeper validation)
- GUI rendering tests (requires GPU/display; headless only tests view creation)
- Performance benchmarking (separate task)
- Multi-instance stress testing (separate task)
- MPE-specific deep testing (covered by pluginval)
- Preset library validation (content, not mechanism)

---

## Dependencies
- **Depends on:** Issue #1 (Foundation VST3 plugin structure — DONE, merged)
- **Complements:** Issue #20 (pluginval integration testing — OPEN)
- **Blocks:** None directly, but enables reliable CI for all future tasks

---

## Definition of Ready (for SWE)
- [ ] This spec is reviewed and approved
- [ ] Headless host candidate evaluated and selected (decision documented)
- [ ] GitHub Actions workflow skeleton exists (`.github/workflows/ci.yml` or similar)
- [ ] VST3 SDK available in CI build environment (already from Task 001)