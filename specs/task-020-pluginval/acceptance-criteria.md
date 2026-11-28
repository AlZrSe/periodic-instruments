# Acceptance Criteria — Task 020: Pluginval Integration Testing

## Overview
Each criterion is traceable to a User Story (US-020.X) and must be verifiable by QA with objective evidence (CI logs, pluginval output, artifact uploads). All criteria must pass on **both Windows ("win" runner) and Linux ("linux" internal runner)**.

---

## AC-020.1: pluginval Installation and Configuration in CI
**Traceability:** US-020.1 (pluginval Installed and Configured in CI)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.1.1 | GitHub Actions workflow file exists at `.github/workflows/pluginval.yml` | File inspection | `cat .github/workflows/pluginval.yml` |
| AC-020.1.2 | Workflow triggers on `push` to `main` and `pull_request` | File inspection | `on:` section in workflow |
| AC-020.1.3 | Windows job runs on `runs-on: [self-hosted, win]` or `windows-latest` | File inspection | `jobs.windows.runs-on` |
| AC-020.1.4 | Linux job runs on `runs-on: [self-hosted, linux]` (internal runner) | File inspection | `jobs.linux.runs-on` |
| AC-020.1.5 | pluginval binary downloaded from official GitHub releases (pinned version) | File inspection + CI log | `curl -L` or `actions/download-artifact` step |
| AC-020.1.6 | pluginval binary cached between runs via `actions/cache` | File inspection + CI log | Cache hit/miss in workflow run |
| AC-020.1.7 | VST3 bundle path resolved from Task 001 build artifact (`target/release/PeriodicInstruments.vst3`) | File inspection + CI log | Artifact download or build step output |

---

## AC-020.2: Plugin Passes pluginval Strictness Level 5 (Host Compatibility Baseline)
**Traceability:** US-020.2 (Plugin Passes pluginval at Strictness Level 5)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.2.1 | Command `pluginval --strictness-level 5 <vst3-path>` exits with code 0 on Windows | CI execution | CI log showing `exit code: 0` |
| AC-020.2.2 | Command `pluginval --strictness-level 5 <vst3-path>` exits with code 0 on Linux | CI execution | CI log showing `exit code: 0` |
| AC-020.2.3 | Plugin discovery test passes (factory found, class registered) | pluginval output | Log lines: "Plugin discovered", "Factory loaded" |
| AC-020.2.4 | Basic lifecycle test passes (initialize → setupProcessing → setActive → terminate) | pluginval output | Log lines: "initialize OK", "setupProcessing OK", "setActive OK", "terminate OK" |
| AC-020.2.5 | Parameter enumeration test passes (5 parameters with correct metadata) | pluginval output | Log lines showing 5 parameters with names/ranges |
| AC-020.2.6 | Basic audio processing test passes (process callback, variable block sizes) | pluginval output | Log lines: "Processing test passed" |
| AC-020.2.7 | Editor creation test passes (view created, headless) | pluginval output | Log lines: "Editor test passed" or "IPlugView created" |
| AC-020.2.8 | No crashes, assertions, or sanitizer errors during level 5 run | CI log + sanitizer | Clean ASAN/TSAN/MSAN output if enabled |

---

## AC-020.3: Plugin Passes pluginval Strictness Level 7 (Extended Testing)
**Traceability:** US-020.3 (Plugin Passes pluginval at Strictness Level 7)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.3.1 | Command `pluginval --strictness-level 7 <vst3-path>` exits with code 0 on Windows | CI execution | CI log showing `exit code: 0` |
| AC-020.3.2 | Command `pluginval --strictness-level 7 <vst3-path>` exits with code 0 on Linux | CI execution | CI log showing `exit code: 0` |
| AC-020.3.3 | State save/restore test passes (getState/setState round-trip) | pluginval output | Log lines: "State save/restore test passed" |
| AC-020.3.4 | Parameter automation test passes (ramp, discrete, boundary values) | pluginval output | Log lines: "Automation test passed" |
| AC-020.3.5 | MIDI event handling test passes (note on/off, CC, pitch bend) | pluginval output | Log lines: "MIDI test passed" |
| AC-020.3.6 | Variable block size stress test passes (64–4096 samples) | pluginval output | Log lines: "Block size test passed" or list of tested sizes |
| AC-020.3.7 | Variable sample rate stress test passes (44.1k–192k Hz) | pluginval output | Log lines: "Sample rate test passed" or list of tested rates |
| AC-020.3.8 | Bypass handling test passes (bypass on/off during processing) | pluginval output | Log lines: "Bypass test passed" |
| AC-020.3.9 | Multiple instantiation test passes (independent instances) | pluginval output | Log lines: "Multiple instances test passed" |
| AC-020.3.10 | No crashes, assertions, or sanitizer errors during level 7 run | CI log + sanitizer | Clean ASAN/TSAN/MSAN output if enabled |

---

## AC-020.4: Headless Execution on Both Platforms
**Traceability:** US-020.4 (Headless Execution on Windows and Linux CI)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.4.1 | Windows job runs without GUI/display dependencies | CI log | No "display" or "X server" errors |
| AC-020.4.2 | Linux job runs headless (Xvfb or native headless mode) | CI log + workflow | `xvfb-run` or `DISPLAY=:99` in workflow |
| AC-020.4.3 | No manual intervention required (fully automated) | CI execution | Workflow completes unattended |

---

## AC-020.5: Variable Block Size and Sample Rate Coverage
**Traceability:** US-020.5 (Variable Block Size and Sample Rate Stress Testing)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.5.1 | Block sizes tested include: 64, 128, 256, 512, 1024, 2048, 4096 | pluginval output (level 7) | Log lines enumerating tested block sizes |
| AC-020.5.2 | Sample rates tested include: 44100, 48000, 88200, 96000, 192000 | pluginval output (level 7) | Log lines enumerating tested sample rates |
| AC-020.5.3 | Process callback handles all combinations without crash | pluginval output | "Stress test passed" or equivalent |

---

## AC-020.6: Parameter Automation and MIDI Handling
**Traceability:** US-020.6 (Parameter Automation and MIDI Handling Verification)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.6.1 | Parameter automation test exercises all 5 parameters | pluginval output (level 7) | Log showing each parameter automated |
| AC-020.6.2 | MIDI note on/off handled correctly (velocity, channel) | pluginval output (level 7) | Log: "MIDI note test passed" |
| AC-020.6.3 | MIDI CC handled for macro parameters (Mass, Reactivity, State, Radiance) | pluginval output (level 7) | Log: "MIDI CC test passed" |
| AC-020.6.4 | Pitch bend and channel pressure handled (if implemented) | pluginval output (level 7) | Log: "Pitch bend test passed" |

---

## AC-020.7: State Save/Restore Verification
**Traceability:** US-020.7 (State Save/Restore and Project Persistence)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.7.1 | `getState` produces valid IBStream data | pluginval output (level 7) | Log: "getState test passed" |
| AC-020.7.2 | `setState` restores all 5 parameter values exactly | pluginval output (level 7) | Log: "setState test passed" + parameter comparison |
| AC-020.7.3 | Program/bank change persists state | pluginval output (level 7) | Log: "Program change test passed" |

---

## AC-020.8: CI Gate — Hard Failure on Validation Error
**Traceability:** US-020.8 (CI Gate — Fail Build on Validation Failure)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-020.8.1 | Workflow job fails (red) if pluginval exit code != 0 | CI execution (test with broken plugin) | Failed workflow run screenshot |
| AC-020.8.2 | Failure on Windows fails the overall workflow | CI execution | Workflow conclusion: "failure" |
| AC-020.8.3 | Failure on Linux fails the overall workflow | CI execution | Workflow conclusion: "failure" |
| AC-020.8.4 | pluginval stdout/stderr uploaded as artifact on failure | CI log + artifacts | Artifact "pluginval-logs-windows" or "pluginval-logs-linux" |
| AC-020.8.5 | No `continue-on-error: true` for pluginval steps | File inspection | Workflow YAML shows no continue-on-error |

---

## Non-Functional Acceptance Criteria

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| NF-AC-020.1 | pluginval version pinned (e.g., `v0.3.8`) for reproducibility | File inspection | Workflow uses specific release URL or hash |
| NF-AC-020.2 | Total CI time for pluginval < 10 minutes (both platforms, both levels) | CI timing | Workflow run duration |
| NF-AC-020.3 | Cache key includes pluginval version and OS | File inspection | `actions/cache` key format |
| NF-AC-020.4 | Workflow uses `permissions: contents: read` (least privilege) | File inspection | `permissions:` block in workflow |
| NF-AC-020.5 | VST3 bundle from Task 001 used (not rebuilt in this workflow) | File inspection | Artifact download or `needs: build` dependency |

---

## Definition of Done (Task 020)
All AC-020.X and NF-AC-020.X criteria **must pass on both Windows and Linux** for QA to mark this task as verified. PM acceptance review follows QA verification.

**Pass requires:**
- ✅ pluginval level 5 passes on Windows ("win" runner)
- ✅ pluginval level 5 passes on Linux ("linux" internal runner)
- ✅ pluginval level 7 passes on Windows ("win" runner)
- ✅ pluginval level 7 passes on Linux ("linux" internal runner)
- ✅ CI workflow fails fast on any validation error
- ✅ Logs uploaded as artifacts on failure
- ✅ Execution time within budget (< 10 min total)