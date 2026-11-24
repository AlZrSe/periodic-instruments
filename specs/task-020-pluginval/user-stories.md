# User Stories — Task 020: Pluginval Integration Testing

## Context
This task establishes automated integration testing for the Periodic Instruments VST3 plugin using **pluginval**, the official VST3 validation tool from Steinberg. It replaces the manual DAW testing from Task 001 with CI-integrated, headless validation that runs on every commit. The pluginval tool exercises the full VST3 plugin lifecycle, parameter handling, audio processing, state persistence, and MIDI support — catching regressions that manual testing would miss.

Dependency: Task 001 (Foundation) must be `done` and its VST3 bundle available. This task uses the `PeriodicInstruments.vst3` produced by Task 001.

---

## User Stories

### US-020.1: pluginval Installed and Configured in CI
**As a** developer  
**I want** pluginval to be downloaded, installed, and available in the CI pipeline (Windows and Linux runners)  
**So that** validation runs automatically on every push and PR without manual setup

**Details:**
- Windows runner ("win"): download pluginval Windows binary from GitHub releases
- Linux runner ("linux"): download pluginval Linux binary or build from source
- Binary cached between runs for speed
- Added to PATH or invoked via explicit path in CI steps

---

### US-020.2: Plugin Passes pluginval at Strictness Level 5 (Host Compatibility Baseline)
**As a** plugin developer  
**I want** the VST3 plugin to pass pluginval validation at strictness level 5  
**So that** the plugin meets the minimum host compatibility requirements for VST3 hosts

**Details:**
- Level 5 tests: plugin discovery, basic lifecycle, parameter enumeration, basic audio processing, editor creation
- This is the "host compatibility baseline" — plugins that pass level 5 should load in any VST3 host
- CI must fail if pluginval returns non-zero exit code
- Run on both Windows and Linux CI runners

---

### US-020.3: Plugin Passes pluginval at Strictness Level 7 (Extended Testing)
**As a** plugin developer  
**I want** the VST3 plugin to pass pluginval validation at strictness level 7  
**So that** extended compliance checks (state persistence, MIDI, automation, stress tests) are verified

**Details:**
- Level 7 adds: state save/restore, parameter automation, MIDI event handling, variable block sizes, variable sample rates, bypass handling, multiple instantiation
- This catches edge cases that level 5 misses
- CI must fail if pluginval returns non-zero exit code
- Run on both Windows and Linux CI runners

---

### US-020.4: Headless Execution on Windows and Linux CI
**As a** CI/CD engineer  
**I want** pluginval to run in fully headless mode on both Windows ("win" runner) and Linux ("linux" internal runner)  
**So that** no GUI dependencies or display servers are required in CI

**Details:**
- Use `--headless` flag or equivalent environment (Xvfb on Linux if needed)
- Windows runner "win" runs natively headless
- Linux internal runner uses Xvfb or native headless mode
- No manual intervention required

---

### US-020.5: Variable Block Size and Sample Rate Stress Testing
**As a** plugin developer  
**I want** pluginval to stress-test the plugin across variable block sizes (64–4096) and sample rates (44.1k–192k)  
**So that** audio processing robustness is verified under real-world host configurations

**Details:**
- Block sizes tested: 64, 128, 256, 512, 1024, 2048, 4096
- Sample rates tested: 44100, 48000, 88200, 96000, 192000 Hz
- Part of pluginval level 7 stress tests
- Must not crash, glitch, or allocate on audio thread

---

### US-020.6: Parameter Automation and MIDI Handling Verification
**As a** plugin developer  
**I want** pluginval to verify parameter automation (ramp, discrete steps) and MIDI event handling (note on/off, CC, pitch bend)  
**So that** DAW automation and MIDI learn functionality works correctly

**Details:**
- Parameter automation: smooth ramps, discrete jumps, boundary values
- MIDI: note on/off with velocity, control changes (for macro knobs), pitch bend, channel pressure
- MPE support where applicable (future-proofing)
- Part of pluginval level 7

---

### US-020.7: State Save/Restore and Project Persistence
**As a** plugin developer  
**I want** pluginval to verify that plugin state (parameters, program data) saves and restores correctly  
**So that** DAW project save/load preserves the user's sound

**Details:**
- Tests `getState` / `setState` (IBStream)
- Tests program/bank change persistence
- Verifies parameter values match after round-trip
- Part of pluginval level 7

---

### US-020.8: CI Gate — Fail Build on Validation Failure
**As a** developer  
**I want** the CI pipeline to fail (red build) if pluginval returns any non-zero exit code  
**So that** broken VST3 compliance blocks merge and is visible immediately

**Details:**
- Both level 5 and level 7 must pass
- Failure on either platform (Windows or Linux) fails the build
- Clear error output in CI logs for debugging
- No "allow failure" — this is a hard gate

---

## Non-Functional Requirements

| Requirement | Detail |
|-------------|--------|
| **CI Integration** | GitHub Actions workflow (`.github/workflows/pluginval.yml`) |
| **Windows Runner** | GitHub-hosted `windows-latest` (label: "win") |
| **Linux Runner** | Self-hosted/internal Linux runner (label: "linux") |
| **pluginval Version** | Latest stable release (pinned to specific version for reproducibility) |
| **VST3 Bundle Source** | `target/release/PeriodicInstruments.vst3` from Task 001 build artifact |
| **Artifact Retention** | pluginval logs uploaded as CI artifacts on failure |
| **Execution Time** | < 5 minutes per platform per strictness level |
| **Caching** | pluginval binary cached via `actions/cache` |

---

## Out of Scope for This Task
- Actual DSP engine tests (synthesis correctness) — covered by unit tests in `src/core/`
- UI visual regression testing — separate task
- Performance benchmarking — separate task
- DAW-specific behavioral testing (Bitwig, Live, Reaper quirks) — manual QA
- MPE full certification — future task

---

## Dependencies
- **Task 001 (Foundation)** — MUST be `done` and closed; provides the VST3 bundle
- **VST3 SDK** — Available in build environment (from Task 001)
- **Rust toolchain** — Stable (from Task 001)

---

## Definition of Ready (for SWE)
- [ ] This spec is reviewed and approved by PM
- [ ] Task 001 is `done` and VST3 bundle builds successfully in CI
- [ ] GitHub Actions workflow skeleton exists or CI infrastructure ready
- [ ] pluginval release URL identified for Windows and Linux binaries
- [ ] Self-hosted Linux runner ("linux") is configured and accessible