# Acceptance Criteria — Task 021: CI with Headless Plugin Host for Testing Ready VST3

## Overview
Each criterion is traceable to a User Story (US-021.X) and must be verifiable by QA with objective evidence (CI logs, artifacts, local test runs). All criteria must pass for QA to mark this task as verified.

---

## AC-021: Headless Plugin Host Selection and Provisioning
**Traceability:** US-021.2 (CI Engineer Reliable Headless Host Selection)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-021.1 | Decision document records evaluated options (VST3PluginTestHost, JUCE AudioPluginHost, custom Rust host) with pros/cons and selected choice | File inspection | `docs/adr/0002-headless-host-selection.md` or similar |
| AC-021.2 | Selected host version pinned to exact release/commit (e.g., `steinberg/vst3plugin-test-host@v1.2.3` or JUCE commit hash) | File inspection | CI workflow or script shows pinned version |
| AC-021.3 | SHA256 checksum verified on download (fail build if mismatch) | CI log inspection | Workflow step shows `sha256sum -c` or equivalent passing |
| AC-021.4 | Host binary cached between CI runs (GitHub Actions `actions/cache`) | CI log inspection | Cache hit/miss logs; subsequent runs faster |
| AC-021.5 | Host binary works on Windows (`.exe`) and Linux (ELF binary) | CI matrix test | Both OS jobs pass host download/verify step |
| AC-021.6 | No host binary committed to repo (only fetched at CI/build time) | Repo inspection | `git ls-files` shows no host binary in repo |

---

## AC-022: CI Workflow Integration
**Traceability:** US-021.1 (Developer Fast Feedback Loop), US-021.4 (On-Call Engineer CI Stability)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-022.1 | GitHub Actions workflow (`.github/workflows/ci.yml` or `headless-test.yml`) exists and runs on `push` and `pull_request` | File inspection + CI trigger | Workflow file; CI run on test push |
| AC-022.2 | Job matrix includes `windows-latest` and `ubuntu-latest` | File inspection | `strategy.matrix.os: [windows-latest, ubuntu-latest]` |
| AC-022.3 | Job runs `cargo build --release` then headless test suite | CI log inspection | Build step + test step both present |
| AC-022.4 | Total job time (build + test) < 120 seconds on clean run | CI timing | GitHub Actions job duration < 2 min |
| AC-022.5 | Job fails (red) if any headless test case fails | CI log inspection | Failed test → job failure |
| AC-022.6 | JUnit XML report generated and uploaded as artifact | CI log + artifact | `test-results-<os>.xml` artifact present |
| AC-022.7 | On failure: host logs, stderr, stdout uploaded as artifacts | CI artifact inspection | `headless-host-logs-<os>.zip` or similar on failed run |
| AC-022.8 | Timeout per test case ≤ 30 seconds (prevents hung jobs) | CI log inspection | No single test runs > 30s; timeout kills child process |
| AC-022.9 | Retry logic: transient failures retried once (max 1 retry) | CI log inspection | "Retrying..." log on flaky failure; max 2 attempts |

---

## AC-023: Plugin Discovery and Instantiation Tests
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-023.1 | Host discovers `PeriodicInstruments.vst3` in build output directory | Test log | "Found plugin: Periodic Instruments" or similar |
| AC-023.2 | Plugin instantiates (factory → component → controller) without crash | Test log | Instantiation success message; exit code 0 |
| AC-023.3 | Plugin component initializes (`initialize()`) successfully | Test log | `initialize` called, returns `kResultOk` |
| AC-023.4 | Plugin controller created and parameter info queried | Test log | Parameter count = 5; names match spec |
| AC-023.5 | Plugin terminates cleanly (`terminate()`) after test | Test log | `terminate` called; no leaks reported by host |

---

## AC-024: Audio Processing Tests
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-024.1 | `setupProcessing()` called with valid `ProcessSetup` (sample rate, block size, symbolic sample size) | Test log | Setup params logged; returns `kResultOk` |
| AC-024.2 | Process callback executes at block sizes: 64, 128, 256, 512, 1024, 2048, 4096 | Test log | Each block size logged; process called N times |
| AC-024.3 | Process callback executes at sample rates: 44100, 48000, 88200, 96000, 176400, 192000 | Test log | Each sample rate logged; process called |
| AC-024.4 | Output buffers contain valid floats (no NaN, no Inf, no denormals) | Test assertion | Assertion passes; log shows "Output validation OK" |
| AC-024.5 | Output is silence (all zeros) for current stub implementation | Test assertion | RMS = 0.0; peak = 0.0 |
| AC-024.6 | `setActive(true)` → process → `setActive(false)` cycle works | Test log | Active toggles logged; no crash |
| AC-024.7 | Process called with variable numInputs/numOutputs (stereo in/out) | Test log | Buffer config logged; process handles correctly |

---

## AC-025: Parameter Tests
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-025.1 | All 5 parameters enumerated with correct info (ID, name, label, min, max, default, flags) | Test log | Parameter table logged; matches US-003 from Task 001 |
| AC-025.2 | `getParameterValue` returns default for all parameters initially | Test assertion | Values: Element=1, Mass=0.5, Reactivity=0.5, State=0.5, Radiance=0.5 |
| AC-025.3 | `setParameterValue` / `performEdit` updates parameter and `getParameterValue` reflects change | Test assertion | Set Mass=0.75 → get returns ~0.75 |
| AC-025.4 | Parameter automation simulation: ramp 0→1→0 over 100 process blocks | Test log + assertion | No crash; values tracked correctly |
| AC-025.5 | Parameter bounds enforced (clamping or rejection for out-of-range) | Test assertion | Set Mass=1.5 → clamped to 1.0 or rejected |
| AC-025.6 | Element selector only accepts integer values 1–94 | Test assertion | Set 1.5 → 1 or 2; set 95 → 94; set 0 → 1 |

---

## AC-026: State Save/Restore (Preset Chunk) Tests
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-026.1 | `getState()` returns non-empty chunk after parameter changes | Test log | Chunk size > 0 bytes logged |
| AC-026.2 | `setState(chunk)` restores parameters to saved values | Test assertion | Modify params → getState → reset params → setState → verify restored |
| AC-026.3 | State chunk round-trip works across multiple save/load cycles (3×) | Test log | 3 cycles logged; all pass |
| AC-026.4 | Default state (fresh plugin) can be saved and restored | Test assertion | Fresh plugin → getState → setState → params unchanged |

---

## AC-027: MIDI Event Handling Tests
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-027.1 | Note On (channel 1, note 60, velocity 100) delivered to process callback | Test log | Event received in `process()` event list |
| AC-027.2 | Note Off (channel 1, note 60) delivered | Test log | Event received |
| AC-027.3 | CC messages (CC 1 Mod Wheel, CC 7 Volume, CC 74 Brightness) delivered | Test log | Events received |
| AC-027.4 | Pitch Bend messages delivered | Test log | Event received |
| AC-027.5 | MIDI events at sample offsets within block handled correctly | Test assertion | Events at offset 0, 32, 63 in 64-block all received |

---

## AC-028: Editor/View Creation Tests (Headless)
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-028.1 | `createView("editor")` returns valid `IPlugView*` without GUI subsystem init | Test log | View created; no GPU/display required |
| AC-028.2 | `isPlatformTypeSupported()` returns true for platform-native type (Win32/HWND, Linux/X11 or headless) | Test log | Platform type check logged |
| AC-028.3 | `attached()` / `removed()` lifecycle calls work without crash | Test log | Attach/remove logged; no crash |
| AC-028.4 | `onSize()` called with valid rect; view resizes | Test log | Size change logged |
| AC-028.5 | View destroyed cleanly (`destroy()`) | Test log | Destroy called; no leaks |

---

## AC-029: Process Isolation and Crash Safety
**Traceability:** US-021.3 (QA Engineer Comprehensive Smoke Test Coverage), US-021.4 (On-Call Engineer CI Stability)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-029.1 | Plugin runs in separate child process from test runner | Code inspection | `std::process::Command` or similar spawns host |
| AC-029.2 | Plugin crash (segfault, panic, abort) → test failure, not runner crash | Fault injection test | Injected crash → test fails cleanly; runner continues |
| AC-029.3 | Child process timeout enforced (30s); killed if exceeded | Test log | Hung plugin → killed; test fails with timeout message |
| AC-029.4 | Crash dumps/minidumps captured on Windows; core dumps on Linux | Artifact inspection | `.dmp` or `core` file in failure artifacts |

---

## AC-030: Local Developer Parity
**Traceability:** US-021.5 (Local Developer Parity)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-030.1 | Single command runs full headless test suite locally (e.g., `cargo test --features headless` or `just test-headless`) | Local execution | Command completes; same tests run |
| AC-030.2 | Same host binary version downloaded/cached locally as CI | Local execution + version check | Log shows same version/hash |
| AC-030.3 | Local run produces JUnit XML (`test-results-local.xml`) and human summary | File inspection | XML file exists; readable summary in terminal |
| AC-030.4 | Works offline after first run (cached binary, no network) | Local execution (disconnect network) | Second run succeeds without network |
| AC-030.5 | Local run time < 60 seconds (no build, just test) on warm cache | Local timing | `time` output < 60s |

---

## Non-Functional Acceptance Criteria

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| NF-AC-021 | No new runtime dependencies added to plugin crate | `Cargo.toml` inspection | `dependencies` section unchanged |
| NF-AC-022 | Test-only dependencies in `[dev-dependencies]` or separate test crate | `Cargo.toml` inspection | Dev deps only |
| NF-023 | `cargo clippy --all-targets --all-features -- -D warnings` passes (including test code) | Command execution | Clippy clean output |
| NF-024 | `cargo fmt --check` passes (including test code) | Command execution | fmt clean output |
| NF-025 | No `unsafe` in test code without justification comment | `grep -r unsafe tests/ src/**/test*` | Only justified unsafe |

---

## Definition of Done (Task 021)
All AC-021 through AC-030 and NF-AC-021 through NF-AC-025 criteria **must pass** for QA to mark this task as verified. PM acceptance review follows QA verification.

**Summary checklist for PM acceptance:**
- [ ] CI workflow runs on push/PR, matrix Windows+Linux, < 2 min
- [ ] Headless host pinned, checksummed, cached
- [ ] All test categories pass: instantiation, audio, params, state, MIDI, editor, crash safety
- [ ] JUnit XML + artifacts on failure
- [ ] Local parity command works offline after first run
- [ ] No plugin runtime deps added; clippy/fmt clean