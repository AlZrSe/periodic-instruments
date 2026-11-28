# Test Scenarios — Task 021: CI with Headless Plugin Host for Testing Ready VST3

Each scenario is a **specific, executable test case** for QA verification. Scenarios are grouped by acceptance criterion. Test IDs use prefix `TS-H` (Headless) to distinguish from Task 001's `TS-` prefix.

---

## TS-H01: Headless Host Provisioning

### TS-H01.1: Host Binary Download and Verify (Windows)
**Precondition:** Windows runner, network access  
**Steps:**
1. CI workflow step executes host download script
2. Download pinned `VST3PluginTestHost.exe` (or selected host) from official release URL
3. Compute SHA256 of downloaded file
4. Compare with expected checksum in script
**Expected:** Download succeeds; checksum matches; binary executable
**Evidence:** CI log shows "Download OK", "Checksum verified", file size > 1 MB

### TS-H01.2: Host Binary Download and Verify (Linux)
**Precondition:** Linux runner, network access  
**Steps:**
1. CI workflow step executes host download script
2. Download pinned host binary (AppImage, tarball, or built from source at pinned commit)
3. Compute SHA256 of downloaded file
3. Compare with expected checksum
**Expected:** Download succeeds; checksum matches; binary executable (`chmod +x`)
**Evidence:** CI log shows "Download OK", "Checksum verified"

### TS-H01.3: Host Binary Cached Between Runs
**Precondition:** First CI run completed (cache populated)  
**Steps:**
1. Trigger second CI run (push empty commit)
2. Observe cache step
**Expected:** Cache hit; download step skipped or near-instant
**Evidence:** CI log shows "Cache hit for headless-host"; download time < 5s

### TS-H01.4: Host Version Check
**Precondition:** Host binary available  
**Steps:**
1. Run host with `--version` or `-v` flag
2. Parse output
**Expected:** Version string matches pinned version exactly
**Evidence:** Log shows "Host version: X.Y.Z (expected X.Y.Z)"

---

## TS-H02: CI Workflow Execution

### TS-H02.1: Full CI Pipeline on Push (Windows)
**Precondition:** Clean GitHub Actions `windows-latest` runner  
**Steps:**
1. Push commit to feature branch
2. Workflow triggers: checkout → setup Rust → cache Cargo → build release → download host → run headless tests
**Expected:** All steps green; total duration < 120s
**Evidence:** GitHub Actions job summary; `test-results-windows.xml` artifact

### TS-H02.2: Full CI Pipeline on Push (Linux)
**Precondition:** Clean GitHub Actions `ubuntu-latest` runner  
**Steps:** Same as TS-H02.1 on Linux
**Expected:** All steps green; total duration < 120s
**Evidence:** GitHub Actions job summary; `test-results-linux.xml` artifact

### TS-H02.3: PR Triggers Workflow
**Precondition:** Open PR against `main`  
**Steps:** Create PR; observe Actions tab
**Expected:** Workflow runs on PR; status check appears on PR
**Evidence:** PR checks show "Headless Plugin Host Tests" pending/passing

### TS-H02.4: Failure Artifacts Uploaded
**Precondition:** Force a test failure (e.g., modify test to expect wrong parameter count)  
**Steps:** Push failing commit; wait for job failure
**Expected:** Job fails; artifacts `headless-host-logs-windows.zip` and/or `headless-host-logs-linux.zip` available
**Evidence:** Artifacts downloadable; contain host stderr/stdout, JUnit XML

### TS-H02.5: Timeout Enforcement
**Precondition:** Test case that hangs (infinite loop in mock)  
**Steps:** Run headless test with 30s timeout configured
**Expected:** Test killed at 30s; marked as failure with "timeout" message; runner continues
**Evidence:** Log shows "Test 'hanging_test' timed out after 30s"; job doesn't hang

### TS-H02.6: Retry on Transient Failure
**Precondition:** Flaky network during host download (simulated)  
**Steps:** Configure retry=1; first download fails, second succeeds
**Expected:** First attempt fails; retry succeeds; job passes
**Evidence:** Log shows "Attempt 1 failed: ... Retrying... Attempt 2 succeeded"

---

## TS-H03: Plugin Discovery and Instantiation

### TS-H03.1: Plugin Bundle Found
**Precondition:** `cargo build --release` completed; `PeriodicInstruments.vst3` in `target/release/`  
**Steps:**
1. Headless test runner invokes host with plugin path
2. Host scans for VST3 bundles
**Expected:** Host logs "Found VST3: PeriodicInstruments.vst3" or similar
**Evidence:** Host stdout/log contains plugin path

### TS-H03.2: Factory Creation
**Precondition:** Plugin bundle found  
**Steps:**
1. Host calls `GetPluginFactory` / `IPluginFactory` creation
**Expected:** Factory pointer valid; no crash
**Evidence:** Test log "Factory created: 0x..."

### TS-H03.3: Component Instantiation
**Precondition:** Factory available  
**Steps:**
1. Host creates component (audio processor) via factory
2. Query `IComponent` interface
**Expected:** Component created; `initialize(nullptr)` returns `kResultOk`
**Evidence:** Log "Component initialized: kResultOk"

### TS-H03.4: Controller Instantiation
**Precondition:** Component available  
**Steps:**
1. Host creates controller via `createInstance` with `IEditController` IID
**Expected:** Controller created; parameter count = 5
**Evidence:** Log "Controller created; parameter count: 5"

### TS-H03.5: Clean Termination
**Precondition:** Component and controller instantiated  
**Steps:**
1. Host calls `terminate()` on component
2. Host releases all interfaces
**Expected:** `terminate()` returns `kResultOk`; no memory leaks reported by host
**Evidence:** Log "Component terminated: kResultOk"; host exit code 0

---

## TS-H04: Audio Processing — Block Sizes

### TS-H04.1: Setup Processing at 44.1 kHz / 64 samples
**Precondition:** Component initialized  
**Steps:**
1. Call `setupProcessing(ProcessSetup{ sampleRate=44100, maxSamplesPerBlock=64, processMode=kRealtime, symbolicSampleSize=kSample32 })`
2. Call `setActive(true)`
3. Call `process()` with 64-sample stereo buffers (10 iterations)
4. Call `setActive(false)`
**Expected:** All calls return `kResultOk`; output buffers all zeros; no NaN/Inf
**Evidence:** Log shows setup params; process called 10×; "Output validation: OK"

### TS-H04.2: Block Size 128 Samples
**Steps:** Same as TS-H04.1 with `maxSamplesPerBlock=128`, process 128-sample buffers
**Expected:** Same success criteria
**Evidence:** Log shows block size 128

### TS-H04.3: Block Size 256 Samples
**Steps:** Same with 256
**Expected:** Same success criteria
**Evidence:** Log shows block size 256

### TS-H04.4: Block Size 512 Samples
**Steps:** Same with 512
**Expected:** Same success criteria
**Evidence:** Log shows block size 512

### TS-H04.5: Block Size 1024 Samples
**Steps:** Same with 1024
**Expected:** Same success criteria
**Evidence:** Log shows block size 1024

### TS-H04.6: Block Size 2048 Samples
**Steps:** Same with 2048
**Expected:** Same success criteria
**Evidence:** Log shows block size 2048

### TS-H04.7: Block Size 4096 Samples
**Steps:** Same with 4096
**Expected:** Same success criteria
**Evidence:** Log shows block size 4096

### TS-H04.8: Rapid Block Size Changes
**Precondition:** Component active  
**Steps:**
1. Loop: setupProcessing with block sizes [64, 4096, 128, 2048, 512]
2. Process 1 block at each size
**Expected:** No crashes; each setup returns `kResultOk`
**Evidence:** Log shows each size transition

---

## TS-H05: Audio Processing — Sample Rates

### TS-H05.1: Sample Rate 44100 Hz
**Precondition:** Component initialized  
**Steps:** `setupProcessing(sampleRate=44100, maxSamplesPerBlock=256)` → process 5 blocks
**Expected:** Success; output silence
**Evidence:** Log "Sample rate: 44100"

### TS-H05.2: Sample Rate 48000 Hz
**Steps:** `setupProcessing(sampleRate=48000, ...)`
**Expected:** Success
**Evidence:** Log "Sample rate: 48000"

### TS-H05.3: Sample Rate 88200 Hz
**Steps:** `setupProcessing(sampleRate=88200, ...)`
**Expected:** Success
**Evidence:** Log "Sample rate: 88200"

### TS-H05.4: Sample Rate 96000 Hz
**Steps:** `setupProcessing(sampleRate=96000, ...)`
**Expected:** Success
**Evidence:** Log "Sample rate: 96000"

### TS-H05.5: Sample Rate 176400 Hz
**Steps:** `setupProcessing(sampleRate=176400, ...)`
**Expected:** Success
**Evidence:** Log "Sample rate: 176400"

### TS-H05.6: Sample Rate 192000 Hz
**Steps:** `setupProcessing(sampleRate=192000, ...)`
**Expected:** Success
**Evidence:** Log "Sample rate: 192000"

---

## TS-H06: Audio Processing — Output Validation

### TS-H06.1: Silence Output Verification
**Precondition:** Any valid setup  
**Steps:** Process 100 blocks; collect all output samples
**Expected:** Every sample == 0.0 (exact); RMS = -inf dB; Peak = 0.0
**Evidence:** Test assertion passes; log "Silence verified: 12800 samples all zero"

### TS-H06.2: NaN/Inf Detection
**Precondition:** Any valid setup  
**Steps:** Process 10 blocks; scan output for `is_nan()` or `is_infinite()`
**Expected:** Zero NaN/Inf samples
**Evidence:** Log "NaN check: 0 found; Inf check: 0 found"

### TS-H06.3: Denormal Detection (Optional)
**Precondition:** Any valid setup  
**Steps:** Process 10 blocks; scan for denormal floats (|x| < 1e-38 && x != 0)
**Expected:** Zero denormals (or documented acceptable count)
**Evidence:** Log "Denormal check: X found"

### TS-H06.4: Active/Inactive Transitions
**Steps:**
1. `setActive(true)` → process 5 blocks → `setActive(false)`
2. Repeat 10× rapidly
**Expected:** No clicks (N/A for silence), no crashes, all transitions `kResultOk`
**Evidence:** Log shows 10 activate/deactivate cycles

### TS-H06.5: Buffer Pointer Validation
**Steps:** Process with valid input/output pointers; verify host doesn't pass null
**Expected:** No null pointer dereference
**Evidence:** No crash; log "Buffer pointers valid"

---

## TS-H07: Parameter Enumeration and Info

### TS-H07.1: Parameter Count
**Precondition:** Controller instantiated  
**Steps:** Call `getParameterCount()`
**Expected:** Returns 5
**Evidence:** Log "Parameter count: 5"

### TS-H07.2: Parameter Info — Element Selector (Index 0)
**Steps:** `getParameterInfo(0, info)`
**Expected:**
- `id` = 0 (or 1-based per VST3)
- `title` = "Element"
- `shortTitle` = "El"
- `units` = ""
- `stepCount` = 94 (discrete)
- `defaultNormalizedValue` = 0.0 (maps to 1) or 1/93
- `flags` = `kCanAutomate | kIsList`
**Evidence:** Log shows full ParameterInfo struct

### TS-H07.3: Parameter Info — Mass (Index 1)
**Steps:** `getParameterInfo(1, info)`
**Expected:**
- `title` = "Mass"
- `stepCount` = 0 (continuous)
- `defaultNormalizedValue` = 0.5
- `flags` = `kCanAutomate`
**Evidence:** Log shows ParameterInfo

### TS-H07.4: Parameter Info — Reactivity (Index 2)
**Steps:** `getParameterInfo(2, info)`
**Expected:** Title "Reactivity", default 0.5, continuous, automatable
**Evidence:** Log

### TS-H07.5: Parameter Info — State (Index 3)
**Steps:** `getParameterInfo(3, info)`
**Expected:** Title "State", default 0.5, continuous, automatable
**Evidence:** Log

### TS-H07.6: Parameter Info — Radiance (Index 4)
**Steps:** `getParameterInfo(4, info)`
**Expected:** Title "Radiance", default 0.5, continuous, automatable
**Evidence:** Log

### TS-H07.7: Invalid Parameter Index
**Steps:** `getParameterInfo(5, info)` (out of bounds)
**Expected:** Returns `kResultFalse` or similar error
**Evidence:** Log "Invalid index handled: kResultFalse"

---

## TS-H08: Parameter Get/Set

### TS-H08.1: Default Values on Fresh Controller
**Steps:** For each param index 0–4: `getParamNormalized(index)`
**Expected:**
- Index 0 (Element): normalized value corresponding to 1 (Hydrogen)
- Index 1–4: 0.5 ± 0.001
**Evidence:** Log "Defaults: [0.0107, 0.5, 0.5, 0.5, 0.5]" (example)

### TS-H08.2: Set Mass to 0.75
**Steps:** `setParamNormalized(1, 0.75)` → `getParamNormalized(1)`
**Expected:** Get returns 0.75 ± 0.001
**Evidence:** Log "Mass set to 0.75, read back 0.750"

### TS-H08.3: Set Reactivity to 0.25
**Steps:** `setParamNormalized(2, 0.25)` → get
**Expected:** Returns 0.25 ± 0.001
**Evidence:** Log

### TS-H08.4: Set State to 1.0
**Steps:** `setParamNormalized(3, 1.0)` → get
**Expected:** Returns 1.0
**Evidence:** Log

### TS-H08.5: Set Radiance to 0.0
**Steps:** `setParamNormalized(4, 0.0)` → get
**Expected:** Returns 0.0
**Evidence:** Log

### TS-H08.6: Set Element to 79 (Gold)
**Steps:** `setParamNormalized(0, 78/93.0)` → get → convert to plain value
**Expected:** Plain value = 79
**Evidence:** Log "Element set to 79 (Gold)"

### TS-H08.7: Out-of-Range Clamping — Mass 1.5
**Steps:** `setParamNormalized(1, 1.5)` → get
**Expected:** Returns 1.0 (clamped)
**Evidence:** Log "Clamped to 1.0"

### TS-H08.8: Out-of-Range Clamping — Mass -0.5
**Steps:** `setParamNormalized(1, -0.5)` → get
**Expected:** Returns 0.0 (clamped)
**Evidence:** Log "Clamped to 0.0"

### TS-H08.9: Element Selector Integer Steps Only
**Steps:** Set normalized values corresponding to 1.5, 2.7, 93.9; read plain values
**Expected:** Plain values are integers 1, 2, 94 (or nearest valid)
**Evidence:** Log shows integer results

---

## TS-H09: Parameter Automation Simulation

### TS-H09.1: Ramp Mass 0→1→0 Over 100 Blocks
**Precondition:** Component active, processing at 256 samples/block  
**Steps:**
1. For block 0–49: `setParamNormalized(1, block/49.0)` before each `process()`
2. For block 50–99: `setParamNormalized(1, 1.0 - (block-50)/49.0)` before each `process()`
3. Verify no crash; parameter value tracked
**Expected:** 100 process calls complete; parameter follows ramp
**Evidence:** Log shows parameter values at blocks 0, 25, 49, 50, 75, 99

### TS-H09.2: Simultaneous Multi-Parameter Automation
**Steps:** Ramp all 4 macros simultaneously with different waveforms (sine, saw, square, triangle) over 200 blocks
**Expected:** No crash; all parameters updated per block
**Evidence:** Log summary "200 blocks processed with 4-param automation"

---

## TS-H10: State Save/Restore

### TS-H10.1: Get State After Parameter Changes
**Precondition:** Controller with non-default parameters (Mass=0.8, Reactivity=0.3, Element=6)  
**Steps:** Call `getState(&stream)` → read stream size
**Expected:** Stream size > 0; valid IBStream
**Evidence:** Log "State chunk size: 1234 bytes"

### TS-H10.2: Set State Restores Parameters
**Precondition:** State chunk from TS-H10.1  
**Steps:**
1. Reset all parameters to defaults
2. Call `setState(stream)` with saved chunk
3. Read all parameters
**Expected:** Parameters match saved values (Mass=0.8, Reactivity=0.3, Element=6)
**Evidence:** Log "Restored: Mass=0.800, Reactivity=0.300, Element=6"

### TS-H10.3: Three Round-Trip Cycles
**Steps:** Repeat TS-H10.1 → TS-H10.2 three times with different parameter sets
**Expected:** All three cycles pass
**Evidence:** Log "Cycle 1 OK, Cycle 2 OK, Cycle 3 OK"

### TS-H10.4: Default State Round-Trip
**Precondition:** Fresh controller (defaults)  
**Steps:** `getState` → `setState` → verify all defaults
**Expected:** Parameters unchanged
**Evidence:** Log "Default state round-trip OK"

### TS-H10.5: Component State (Processor Side)
**Precondition:** Component active with controller-linked parameters  
**Steps:** `component.getState()` → `component.setState()` → verify
**Expected:** Processor state also round-trips (if implemented)
**Evidence:** Log "Component state round-trip OK"

---

## TS-H11: MIDI Event Handling

### TS-H11.1: Note On Event
**Precondition:** Component active, process setup done  
**Steps:**
1. Create `Event` list with one Note On: `busIndex=0, sampleOffset=0, type=NoteOn, noteOn={channel=0, note=60, velocity=100, tuning=0}`
2. Call `process()` with event list
3. Verify event received in process data
**Expected:** Process callback sees event; no crash
**Evidence:** Log "Process received Note On: ch0 note60 vel100"

### TS-H11.2: Note Off Event
**Steps:** Same with Note Off at sampleOffset=32
**Expected:** Event received at correct offset
**Evidence:** Log "Note Off at offset 32"

### TS-H11.3: CC Event (Mod Wheel CC1)
**Steps:** `type=Controller, controller={channel=0, ccNumber=1, value=0.5}`
**Expected:** Event received
**Evidence:** Log "CC1 value 0.5"

### TS-H11.4: CC Event (Volume CC7)
**Steps:** `controller={ccNumber=7, value=0.8}`
**Expected:** Event received
**Evidence:** Log

### TS-H11.5: Pitch Bend Event
**Steps:** `type=PitchBend, pitchBend={channel=0, value=0.5}` (center)
**Expected:** Event received
**Evidence:** Log "Pitch bend center"

### TS-H11.6: Multiple Events in One Block
**Steps:** 5 events at offsets 0, 16, 32, 48, 63 in 64-sample block
**Expected:** All 5 received in correct order
**Evidence:** Log "5 events processed at offsets [0,16,32,48,63]"

### TS-H11.7: Event List with Zero Events
**Steps:** Call `process()` with empty event list (`numEvents=0`)
**Expected:** Process succeeds; no crash
**Evidence:** Log "Empty event list OK"

---

## TS-H12: Editor/View Creation (Headless)

### TS-H12.1: Create View — Editor
**Precondition:** Controller instantiated  
**Steps:** `createView("editor")` → check returned `IPlugView*`
**Expected:** Non-null pointer; `isPlatformTypeSupported()` returns true for headless/kPlatformTypeHIView/kPlatformTypeWindow
**Evidence:** Log "View created: 0x...; platform supported: true"

### TS-H12.2: View Attached/Removed
**Steps:**
1. `view->attached(nullptr, kPlatformTypeHeadless)` (or platform-native null parent)
2. `view->removed()`
**Expected:** Both return `kResultOk`; no crash
**Evidence:** Log "Attached OK; Removed OK"

### TS-H12.3: View onSize
**Steps:** `view->onSize(ViewRect{0,0,1200,800})`
**Expected:** Returns `kResultOk`
**Evidence:** Log "onSize(1200x800) OK"

### TS-H12.4: View GetSize
**Steps:** `view->getSize(&rect)`
**Expected:** Returns `kResultOk`; rect matches last onSize or default
**Evidence:** Log "getSize: 1200x800"

### TS-H12.5: View Destroy
**Steps:** `view->destroy()`
**Expected:** Returns `kResultOk`; view pointer invalid after
**Evidence:** Log "View destroyed OK"

### TS-H12.6: Invalid View Type
**Steps:** `createView("invalid_type_name")`
**Expected:** Returns null; no crash
**Evidence:** Log "Invalid view type returns null"

---

## TS-H13: Process Isolation and Crash Safety

### TS-H13.1: Plugin Crash Injected — Test Runner Survives
**Precondition:** Test runner spawns host as child process  
**Steps:**
1. Modify plugin to call `std::process::abort()` on 3rd process call (test-only feature flag)
2. Run headless test
**Expected:** Child process exits with signal SIGABRT / exit code 134 (Unix) or crash code (Windows); test runner catches and marks test FAILED; runner continues to next test
**Evidence:** Test log "Test 'crash_injection' FAILED: child process exited with signal 6"; subsequent tests run

### TS-H13.2: Plugin Panic — Test Runner Survives
**Steps:** Plugin panics on parameter set (test-only)
**Expected:** Child process exits with panic code; test fails cleanly
**Evidence:** Log shows panic message in child stderr; runner OK

### TS-H13.3: Timeout Kill
**Precondition:** Test case with 30s timeout  
**Steps:** Plugin enters infinite loop in `process()` (test-only)
**Expected:** After 30s, test runner kills child; marks test FAILED with timeout
**Evidence:** Log "Test 'hang_injection' FAILED: timeout after 30s"

### TS-H13.4: Crash Dump Captured (Windows)
**Precondition:** Windows runner, crash injected  
**Steps:** Run crashing test; check artifacts
**Expected:** `.dmp` minidump file in failure artifacts
**Evidence:** Artifact `crash_dump_<pid>.dmp` present

### TS-H13.5: Core Dump Captured (Linux)
**Precondition:** Linux runner, crash injected, `ulimit -c unlimited`  
**Steps:** Run crashing test; check artifacts
**Expected:** `core.<pid>` file in failure artifacts
**Evidence:** Artifact `core_dump_<pid>` present

---

## TS-H14: Local Developer Parity

### TS-H14.1: Local Test Command Exists
**Precondition:** Fresh clone, Rust toolchain, no host binary cached  
**Steps:** Run `cargo test --features headless` (or `just test-headless` / `./scripts/test-headless.sh`)
**Expected:** Downloads host, runs all tests, prints summary
**Evidence:** Terminal output shows test results

### TS-H14.2: Local Test Uses Same Host Version
**Steps:** Check version logged during local run vs CI log
**Expected:** Identical version string
**Evidence:** "Host version: X.Y.Z" matches CI

### TS-H14.3: Local JUnit XML Output
**Steps:** Run local test; check for `test-results-local.xml`
**Expected:** File exists; valid JUnit XML; contains all test cases
**Evidence:** `cat test-results-local.xml | xmllint --format -`

### TS-H14.4: Offline Mode After Warm Cache
**Precondition:** Local test run once (cache populated)  
**Steps:** Disconnect network (or `cargo test --features headless --offline` if supported); run again
**Expected:** Runs successfully without network access
**Evidence:** Second run completes; no network errors

### TS-H14.5: Local Run Time < 60s (Warm)
**Precondition:** Host cached, plugin built  
**Steps:** `time cargo test --features headless`
**Expected:** Real time < 60 seconds
**Evidence:** `time` output shows < 60s

---

## TS-H15: Cross-Platform Parity

### TS-H15.1: Windows ↔ Linux Test Results Match
**Steps:** Compare JUnit XML from Windows and Linux CI runs
**Expected:** Same test count; same pass/fail pattern (allowing for platform-specific skips documented)
**Evidence:** Diff of test case names and results

### TS-H15.2: Host Binary Platform-Specific but Version-Aligned
**Steps:** Verify Windows `.exe` and Linux binary are same logical version
**Expected:** Version strings match; feature parity
**Evidence:** Version check logs from both OS

---

## Test Execution Checklist

| Scenario | Windows CI | Linux CI | Local | Evidence Collected |
|----------|------------|----------|-------|-------------------|
| TS-H01.1 | ☐ | N/A | ☐ | Download log, checksum |
| TS-H01.2 | N/A | ☐ | ☐ | Download log, checksum |
| TS-H01.3 | ☐ | ☐ | N/A | Cache hit log |
| TS-H01.4 | ☐ | ☐ | ☐ | Version log |
| TS-H02.1 | ☐ | N/A | N/A | Full CI log, JUnit XML |
| TS-H02.2 | N/A | ☐ | N/A | Full CI log, JUnit XML |
| TS-H02.3 | ☐ | ☐ | N/A | PR check status |
| TS-H02.4 | ☐ | ☐ | N/A | Failure artifacts |
| TS-H02.5 | ☐ | ☐ | ☐ | Timeout log |
| TS-H02.6 | ☐ | ☐ | N/A | Retry log |
| TS-H03.1 | ☐ | ☐ | ☐ | Discovery log |
| TS-H03.2 | ☐ | ☐ | ☐ | Factory log |
| TS-H03.3 | ☐ | ☐ | ☐ | Component init log |
| TS-H03.4 | ☐ | ☐ | ☐ | Controller log |
| TS-H03.5 | ☐ | ☐ | ☐ | Terminate log |
| TS-H04.1–H04.7 | ☐ | ☐ | ☐ | Block size logs |
| TS-H04.8 | ☐ | ☐ | ☐ | Transition log |
| TS-H05.1–H05.6 | ☐ | ☐ | ☐ | Sample rate logs |
| TS-H06.1 | ☐ | ☐ | ☐ | Silence stats |
| TS-H06.2 | ☐ | ☐ | ☐ | NaN/Inf check |
| TS-H06.3 | ☐ | ☐ | ☐ | Denormal check |
| TS-H06.4 | ☐ | ☐ | ☐ | Active toggle log |
| TS-H06.5 | ☐ | ☐ | ☐ | Buffer validation |
| TS-H07.1–H07.7 | ☐ | ☐ | ☐ | Parameter info logs |
| TS-H08.1–H08.9 | ☐ | ☐ | ☐ | Get/set logs |
| TS-H09.1–H09.2 | ☐ | ☐ | ☐ | Automation logs |
| TS-H10.1–H10.5 | ☐ | ☐ | ☐ | State round-trip logs |
| TS-H11.1–H11.7 | ☐ | ☐ | ☐ | MIDI event logs |
| TS-H12.1–H12.6 | ☐ | ☐ | ☐ | View lifecycle logs |
| TS-H13.1–H13.5 | ☐ | ☐ | ☐ | Crash/timeout logs, dumps |
| TS-H14.1–H14.5 | N/A | N/A | ☐ | Local run output, XML |
| TS-H15.1–H15.2 | ☐ | ☐ | N/A | Cross-platform comparison |

---

## Pass/Fail Criteria

**Task passes QA** if:
- All ☐ checkboxes marked ✓ (pass) on both Windows and Linux CI
- Local parity command works (TS-H14.1–H14.5)
- No critical failures (test runner crash, host binary mismatch, timeout hangs)
- JUnit XML artifacts generated on every run
- Failure artifacts uploaded on any failure
- Total CI job time < 120s on clean runs

**Task fails QA** if:
- Any critical failure (runner crash, host not found, checksum mismatch)
- CI job time > 180s consistently
- Local command doesn't exist or fails on clean machine
- Pluginval (Issue #20) not complemented — i.e., headless tests don't cover basic viability
- Cross-platform test parity broken without documented reason

---

## Notes for QA Engineer

- **Host selection decision** must be documented before test execution (ADR in `docs/adr/`)
- **Pin exact version** — do not use `latest` tag; use specific release tag or commit hash
- **Checksums** must be stored in the test script/workflow, not fetched dynamically
- **Process isolation** is mandatory — verify child process spawning in test runner code
- **Artifacts** — configure GitHub Actions `upload-artifact` with `if: failure()` for logs/dumps
- **JUnit XML** — use `junit-report` crate or equivalent to generate standard format
- **Timeouts** — set at test-runner level (not host level) for granular control
- **Flakiness** — if any test flakes > 1% in 20 runs, investigate and fix or mark as known-flaky with retry
- **Documentation** — update `README.md` or `CONTRIBUTING.md` with local test command