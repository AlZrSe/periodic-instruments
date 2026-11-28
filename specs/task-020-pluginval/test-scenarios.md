# Test Scenarios — Task 020: Pluginval Integration Testing

Each scenario is a **specific, executable test case** for QA verification. Scenarios are grouped by acceptance criterion. All scenarios must be executed on **both Windows ("win" runner) and Linux ("linux" internal runner)**.

---

## TS-020.1: CI Workflow and pluginval Setup

### TS-020.1.1: Workflow File Exists and Is Valid YAML
**Precondition:** Repository cloned, GitHub Actions enabled  
**Steps:**
1. `cat .github/workflows/pluginval.yml`
2. Validate YAML syntax: `yamllint .github/workflows/pluginval.yml` (or GitHub UI)
**Expected:** File exists, valid YAML, contains `on:`, `jobs:`, `runs-on:` for windows and linux
**Evidence:** File contents; lint pass output

### TS-020.1.2: pluginval Binary Downloads and Caches on Windows
**Precondition:** Clean GitHub Actions "win" runner  
**Steps:**
1. Trigger workflow (push to feature branch)
2. Observe "Download pluginval" step
3. Check cache hit/miss on second run
**Expected:** Binary downloaded from `https://github.com/steinbergmedia/vst3pluginval/releases/download/v<version>/pluginval-win.zip` (or equivalent); cached on subsequent runs
**Evidence:** CI log showing download + cache step; `ls` of cached path

### TS-020.1.3: pluginval Binary Downloads and Caches on Linux
**Precondition:** Clean internal "linux" runner  
**Steps:**
1. Trigger workflow
2. Observe "Download pluginval" step for Linux
3. Check cache hit/miss on second run
**Expected:** Binary downloaded from `https://github.com/steinbergmedia/vst3pluginval/releases/download/v<version>/pluginval-linux.zip` (or built from source); cached
**Evidence:** CI log showing download/build + cache step

### TS-020.1.4: VST3 Bundle Available from Task 001 Artifact
**Precondition:** Task 001 build workflow completed successfully  
**Steps:**
1. Check workflow `needs: build` or artifact download step
2. Verify `target/release/PeriodicInstruments.vst3` exists at pluginval step
**Expected:** Bundle present at expected path; `moduleinfo.json` valid
**Evidence:** CI log: `ls -la target/release/PeriodicInstruments.vst3/`

---

## TS-020.2: pluginval Strictness Level 5 (Host Compatibility Baseline)

### TS-020.2.1: Level 5 Passes on Windows
**Precondition:** VST3 bundle built, pluginval installed on "win" runner  
**Steps:**
1. Run: `pluginval --strictness-level 5 target/release/PeriodicInstruments.vst3`
2. Capture exit code and full output
**Expected:** Exit code 0; all level 5 test suites pass:
- Plugin Discovery
- Factory Loading
- Class Registration
- Initialize/Terminate
- Setup Processing
- Set Active
- Parameter Enumeration
- Audio Processing (basic)
- Editor Creation (headless)
**Evidence:** Full pluginval stdout/stderr; exit code

### TS-020.2.2: Level 5 Passes on Linux
**Precondition:** VST3 bundle built, pluginval installed on "linux" runner  
**Steps:**
1. Run: `xvfb-run -a pluginval --strictness-level 5 target/release/PeriodicInstruments.vst3` (or native headless)
2. Capture exit code and full output
**Expected:** Exit code 0; same test suites pass as Windows
**Evidence:** Full pluginval stdout/stderr; exit code

### TS-020.2.3: Plugin Discovery and Factory Loading Verified
**Precondition:** Level 5 run completed  
**Steps:** Inspect pluginval output for discovery phase
**Expected:** Log contains:
```
[INFO] Discovering plugin...
[INFO] Factory loaded: Periodic Instruments
[INFO] Class found: Periodic Instruments (FUID: <valid>)
[INFO] Category: Instrument|Synth
```
**Evidence:** Relevant log lines

### TS-020.2.4: Parameter Enumeration Verified (5 Parameters)
**Precondition:** Level 5 run completed  
**Steps:** Inspect pluginval output for parameter enumeration
**Expected:** Log shows exactly 5 parameters with correct info:
1. Element Selector (int, 1–94, default 1)
2. Mass (float, 0.0–1.0, default 0.5)
3. Reactivity (float, 0.0–1.0, default 0.5)
4. State (float, 0.0–1.0, default 0.5)
5. Radiance (float, 0.0–1.0, default 0.5)
All marked automatable
**Evidence:** Parameter dump from pluginval output

### TS-020.2.5: Basic Audio Processing Verified
**Precondition:** Level 5 run completed  
**Steps:** Inspect pluginval output for processing test
**Expected:** Log shows processing at default block size/sample rate without error:
```
[INFO] Testing process() at blockSize=512, sampleRate=44100...
[INFO] Process test passed
```
**Evidence:** Processing test log lines

### TS-020.2.6: Editor Creation Verified (Headless)
**Precondition:** Level 5 run completed  
**Steps:** Inspect pluginval output for editor test
**Expected:** Log shows editor/view created without GUI:
```
[INFO] Creating editor (headless)...
[INFO] IPlugView created successfully
[INFO] Editor test passed
```
**Evidence:** Editor test log lines

---

## TS-020.3: pluginval Strictness Level 7 (Extended Testing)

### TS-020.3.1: Level 7 Passes on Windows
**Precondition:** VST3 bundle built, pluginval installed on "win" runner  
**Steps:**
1. Run: `pluginval --strictness-level 7 target/release/PeriodicInstruments.vst3`
2. Capture exit code and full output
**Expected:** Exit code 0; all level 7 test suites pass (includes level 5 + extended)
**Evidence:** Full pluginval stdout/stderr; exit code

### TS-020.3.2: Level 7 Passes on Linux
**Precondition:** VST3 bundle built, pluginval installed on "linux" runner  
**Steps:**
1. Run: `xvfb-run -a pluginval --strictness-level 7 target/release/PeriodicInstruments.vst3`
2. Capture exit code and full output
**Expected:** Exit code 0; same test suites pass as Windows
**Evidence:** Full pluginval stdout/stderr; exit code

### TS-020.3.3: State Save/Restore Round-Trip Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for state tests
**Expected:** Log shows:
```
[INFO] Testing state save/restore...
[INFO] getState() returned <N> bytes
[INFO] setState() restored parameters
[INFO] Parameter comparison: all match
[INFO] State test passed
```
**Evidence:** State test log lines

### TS-020.3.4: Parameter Automation Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for automation tests
**Expected:** Log shows each parameter automated through range:
```
[INFO] Automating parameter 0 (Element Selector): 1 → 94 → 1
[INFO] Automating parameter 1 (Mass): 0.0 → 1.0 → 0.5
[INFO] Automating parameter 2 (Reactivity): 0.0 → 1.0 → 0.5
[INFO] Automating parameter 3 (State): 0.0 → 1.0 → 0.5
[INFO] Automating parameter 4 (Radiance): 0.0 → 1.0 → 0.5
[INFO] Automation test passed
```
**Evidence:** Automation test log lines

### TS-020.3.5: MIDI Event Handling Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for MIDI tests
**Expected:** Log shows:
```
[INFO] Testing MIDI note on/off (channel 0, note 60, velocity 100)
[INFO] Testing MIDI CC (controller 1, value 64) → Mass parameter
[INFO] Testing MIDI CC (controller 2, value 64) → Reactivity parameter
[INFO] Testing MIDI CC (controller 3, value 64) → State parameter
[INFO] Testing MIDI CC (controller 4, value 64) → Radiance parameter
[INFO] Testing pitch bend (channel 0, value 8192)
[INFO] MIDI test passed
```
**Evidence:** MIDI test log lines

### TS-020.3.6: Variable Block Size Stress Test Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for block size tests
**Expected:** Log shows testing at all standard block sizes:
```
[INFO] Testing block size: 64
[INFO] Testing block size: 128
[INFO] Testing block size: 256
[INFO] Testing block size: 512
[INFO] Testing block size: 1024
[INFO] Testing block size: 2048
[INFO] Testing block size: 4096
[INFO] Block size stress test passed
```
**Evidence:** Block size test log lines

### TS-020.3.7: Variable Sample Rate Stress Test Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for sample rate tests
**Expected:** Log shows testing at all standard sample rates:
```
[INFO] Testing sample rate: 44100
[INFO] Testing sample rate: 48000
[INFO] Testing sample rate: 88200
[INFO] Testing sample rate: 96000
[INFO] Testing sample rate: 192000
[INFO] Sample rate stress test passed
```
**Evidence:** Sample rate test log lines

### TS-020.3.8: Bypass Handling Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for bypass test
**Expected:** Log shows:
```
[INFO] Testing bypass: process() with bypass=true
[INFO] Testing bypass: process() with bypass=false
[INFO] Testing bypass toggle during processing
[INFO] Bypass test passed
```
**Evidence:** Bypass test log lines

### TS-020.3.9: Multiple Instantiation Verified
**Precondition:** Level 7 run completed  
**Steps:** Inspect pluginval output for multi-instance test
**Expected:** Log shows:
```
[INFO] Creating 2nd instance...
[INFO] Creating 3rd instance...
[INFO] Processing on all instances independently
[INFO] Parameter changes isolated per instance
[INFO] Multiple instances test passed
```
**Evidence:** Multi-instance test log lines

---

## TS-020.4: Headless Execution Verification

### TS-020.4.1: Windows Runner Executes Without GUI
**Precondition:** Workflow running on "win" runner  
**Steps:** Check CI log for any GUI-related errors
**Expected:** No errors about "display", "window", "CreateWindow", "HWND", or X server
**Evidence:** Clean CI log (grep for GUI terms returns nothing)

### TS-020.4.2: Linux Runner Executes Headless (Xvfb or Native)
**Precondition:** Workflow running on "linux" runner  
**Steps:** Check workflow YAML for `xvfb-run` or native headless flag; check CI log
**Expected:** Either `xvfb-run -a` wraps pluginval, or pluginval runs with `--headless` flag (if supported)
**Evidence:** Workflow YAML snippet; CI log showing Xvfb start or headless mode

---

## TS-020.5: CI Gate — Failure Behavior

### TS-020.5.1: Workflow Fails on pluginval Non-Zero Exit (Windows)
**Precondition:** Temporarily modify plugin to fail validation (e.g., remove a parameter)  
**Steps:**
1. Push broken plugin to feature branch
2. Observe Windows job result
**Expected:** Windows job shows red (failure); workflow concludes "failure"
**Evidence:** Screenshot of failed workflow run; pluginval exit code != 0 in log

### TS-020.5.2: Workflow Fails on pluginval Non-Zero Exit (Linux)
**Precondition:** Same broken plugin  
**Steps:** Observe Linux job result
**Expected:** Linux job shows red (failure); workflow concludes "failure"
**Evidence:** Screenshot of failed workflow run

### TS-020.5.3: pluginval Logs Uploaded as Artifacts on Failure
**Precondition:** Failed workflow run from TS-020.5.1/5.2  
**Steps:** Check workflow run artifacts
**Expected:** Artifacts named `pluginval-logs-windows` and/or `pluginval-logs-linux` available for download; contain full stdout/stderr
**Evidence:** Artifact list screenshot; downloaded log contents

### TS-020.5.4: No continue-on-error for pluginval Steps
**Precondition:** Workflow file inspected  
**Steps:** `grep -A 5 -B 5 'pluginval' .github/workflows/pluginval.yml`
**Expected:** No `continue-on-error: true` on any pluginval step
**Evidence:** Workflow YAML excerpt

---

## TS-020.6: Performance and Reproducibility

### TS-020.6.1: Execution Time Within Budget
**Precondition:** Successful workflow run (both platforms, both levels)  
**Steps:** Check workflow run timing (GitHub Actions UI or `gh run view`)
**Expected:** Total pluginval time < 10 minutes (both platforms combined, both levels)
**Evidence:** Timing breakdown per job

### TS-020.6.2: pluginval Version Pinned
**Precondition:** Workflow file inspected  
**Steps:** Check download URL or version variable
**Expected:** Specific version (e.g., `v0.3.8`) used, not `latest`
**Evidence:** Workflow YAML showing pinned version

### TS-020.6.3: Cache Key Includes Version and OS
**Precondition:** Workflow file inspected  
**Steps:** Check `actions/cache` key for pluginval
**Expected:** Key format like `pluginval-${{ runner.os }}-v0.3.8`
**Evidence:** Workflow YAML cache step

---

## TS-020.7: Cross-Platform Parity

### TS-020.7.1: Identical Test Results Windows ↔ Linux
**Precondition:** Both platforms passed level 5 and level 7  
**Steps:** Compare pluginval output summaries
**Expected:** Same test counts, same pass/fail pattern, no platform-specific failures
**Evidence:** Side-by-side log comparison (summary lines)

---

## Test Execution Checklist

| Scenario | Windows ("win") | Linux ("linux") | Evidence Collected |
|----------|-----------------|-----------------|-------------------|
| TS-020.1.1 | ☐ | ☐ | Workflow file |
| TS-020.1.2 | ☐ | N/A | CI log + cache |
| TS-020.1.3 | N/A | ☐ | CI log + cache |
| TS-020.1.4 | ☐ | ☐ | Artifact check |
| TS-020.2.1 | ☐ | N/A | pluginval output + exit code |
| TS-020.2.2 | N/A | ☐ | pluginval output + exit code |
| TS-020.2.3 | ☐ | ☐ | Discovery log lines |
| TS-020.2.4 | ☐ | ☐ | Parameter dump |
| TS-020.2.5 | ☐ | ☐ | Processing log |
| TS-020.2.6 | ☐ | ☐ | Editor log |
| TS-020.3.1 | ☐ | N/A | pluginval output + exit code |
| TS-020.3.2 | N/A | ☐ | pluginval output + exit code |
| TS-020.3.3 | ☐ | ☐ | State test log |
| TS-020.3.4 | ☐ | ☐ | Automation log |
| TS-020.3.5 | ☐ | ☐ | MIDI test log |
| TS-020.3.6 | ☐ | ☐ | Block size log |
| TS-020.3.7 | ☐ | ☐ | Sample rate log |
| TS-020.3.8 | ☐ | ☐ | Bypass log |
| TS-020.3.9 | ☐ | ☐ | Multi-instance log |
| TS-020.4.1 | ☐ | N/A | Grep for GUI errors |
| TS-020.4.2 | N/A | ☐ | Workflow YAML + log |
| TS-020.5.1 | ☐ | N/A | Failed run screenshot |
| TS-020.5.2 | N/A | ☐ | Failed run screenshot |
| TS-020.5.3 | ☐ | ☐ | Artifact download |
| TS-020.5.4 | ☐ | ☐ | Workflow YAML |
| TS-020.6.1 | ☐ | ☐ | Timing data |
| TS-020.6.2 | ☐ | ☐ | Version pin |
| TS-020.6.3 | ☐ | ☐ | Cache key |
| TS-020.7.1 | ☐ | ☐ | Comparison matrix |

---

## Pass/Fail Criteria

**Task passes QA** if:
- All ☐ checkboxes marked ✓ (pass) on **both Windows and Linux**
- pluginval level 5 exits 0 on both platforms
- pluginval level 7 exits 0 on both platforms
- CI workflow fails (red) when pluginval fails (verified by TS-020.5.1/5.2)
- No critical failures (crashes, sanitizer errors, assertion failures)
- Execution time within budget (< 10 min total)

**Task fails QA** if:
- Any critical failure on either platform
- pluginval level 5 or 7 fails on either platform
- CI workflow does not fail on pluginval error
- Headless execution not working (GUI errors on Linux)
- Logs not uploaded as artifacts on failure
- pluginval version not pinned

---

## Notes for QA Engineer
- Run tests on **fresh CI runners** (no cached VST3 bundle from local builds)
- Use the **exact VST3 bundle** produced by Task 001 CI build (artifact download)
- Capture **full pluginval stdout/stderr** — do not truncate
- On Linux, verify **Xvfb is running** before pluginval: `ps aux | grep Xvfb`
- If pluginval supports `--json` output, capture that for structured parsing
- Document **exact pluginval version** tested (e.g., `v0.3.8`)
- Report **any warnings** in pluginval output even if exit code is 0
- For TS-020.5.1/5.2 (failure injection), **revert the breakage** after verification