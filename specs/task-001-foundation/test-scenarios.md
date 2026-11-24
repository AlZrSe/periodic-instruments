# Test Scenarios — Task 001: Foundation Application (Basic VST3 Support)

Each scenario is a **specific, executable test case** for QA verification. Scenarios are grouped by acceptance criterion.

---

## TS-001: Build System Verification

### TS-001.1: Clean Build on Windows (MSVC)
**Precondition:** Windows 10/11, Rust stable (MSVC), VST3 SDK installed at `%VST3_SDK%` or default location  
**Steps:**
1. `git clone <repo> periodic-instruments`
2. `cd periodic-instruments`
3. `cargo build --release`
**Expected:** Build completes successfully; `target/release/Periodic Instruments.vst3` exists
**Evidence:** Build log last 20 lines; `dir target\release\Periodic Instruments.vst3`

### TS-001.2: Clean Build on Linux (GNU)
**Precondition:** Ubuntu 20.04+, Rust stable (GNU), VST3 SDK at `/usr/local/vst3sdk` or `$VST3_SDK`  
**Steps:**
1. `git clone <repo> periodic-instruments`
2. `cd periodic-instruments`
3. `cargo build --release`
**Expected:** Build completes successfully; `target/release/Periodic Instruments.vst3` exists
**Evidence:** Build log last 20 lines; `ls -la target/release/Periodic\ Instruments.vst3/`

### TS-001.3: Cargo Check Passes
**Steps:** `cargo check --release`
**Expected:** No errors, no warnings about unused dependencies
**Evidence:** `cargo check --release 2>&1 | grep -E "(error|warning: unused)"`

### TS-001.4: Dependency Audit
**Steps:** `cargo tree -d` (check for duplicate dependencies)
**Expected:** No duplicate core dependencies (nice-plug, vst3-sys, iced, parking_lot)
**Evidence:** Output of `cargo tree -d`

---

## TS-002: Project Structure Verification

### TS-002.1: Source Tree Layout
**Steps:** `find src -type f -name "*.rs" | sort`
**Expected:** At minimum:
```
src/core/mod.rs
src/elements/mod.rs
src/fx/mod.rs
src/plugin/mod.rs
src/ui/mod.rs
src/lib.rs (or src/main.rs for plugin entry)
```
**Evidence:** File listing output

### TS-002.2: Module Declarations
**Steps:** `cat src/*/mod.rs`
**Expected:** Each mod.rs exists and contains valid Rust module declarations (can be empty `// modules go here`)
**Evidence:** Contents of each mod.rs

---

## TS-003: VST3 Bundle Validation

### TS-003.1: Windows Bundle Structure
**Steps:** 
```
dir target\release\Periodic Instruments.vst3\Contents
dir target\release\Periodic Instruments.vst3\Contents\x86_64-win
dir target\release\Periodic Instruments.vst3\Contents\Resources
```
**Expected:**
- `Contents/x86_64-win/Periodic Instruments.dll` exists
- `Contents/Resources/moduleinfo.json` exists and is valid JSON
**Evidence:** Directory listings; `cat Contents/Resources/moduleinfo.json`

### TS-003.2: Linux Bundle Structure
**Steps:**
```
ls -la target/release/Periodic\ Instruments.vst3/Contents/
ls -la target/release/Periodic\ Instruments.vst3/Contents/x86_64-linux/
ls -la target/release/Periodic\ Instruments.vst3/Contents/Resources/
```
**Expected:**
- `Contents/x86_64-linux/libPeriodic Instruments.so` exists
- `Contents/Resources/moduleinfo.json` exists and is valid JSON
**Evidence:** Directory listings; `cat Contents/Resources/moduleinfo.json`

### TS-003.3: moduleinfo.json Content
**Steps:** Inspect `moduleinfo.json`
**Expected:** Contains:
```json
{
  "factory": "Periodic Instruments",
  "classes": [
    {
      "name": "Periodic Instruments",
      "cid": "<valid FUID not all zeros>",
      "category": "Instrument|Synth",
      "version": "0.1.0",
      "vendor": "AlZrSe",
      "sdkVersion": "3.7.0"
    }
  ]
}
```
**Evidence:** Full JSON output

---

## TS-004: DAW Loading Tests

### TS-004.1: Reaper (Windows)
**Precondition:** Reaper 6+ installed, VST3 path configured  
**Steps:**
1. Copy `Periodic Instruments.vst3` to Reaper VST3 path (`%APPDATA%\REAPER\UserPlugins\` or common VST3 folder)
2. Open Reaper → Options → Preferences → Plug-ins → VST3 → Re-scan
3. Insert → Virtual Instrument → VST3: Periodic Instruments
**Expected:** Plugin appears in list; loads without error; track shows "Periodic Instruments"
**Evidence:** Screenshot of plugin browser + instantiated track

### TS-004.2: Reaper (Linux)
**Precondition:** Reaper 6+ on Linux, VST3 path configured  
**Steps:** Same as TS-004.1, using `~/.vst3/` or `/usr/lib/vst3/`
**Expected:** Same as TS-004.1
**Evidence:** Screenshot

### TS-004.3: Bitwig Studio (Windows or Linux)
**Precondition:** Bitwig 4+ installed  
**Steps:**
1. Settings → Plug-ins → Add VST3 location → Rescan
2. Create instrument track → Browse → Periodic Instruments
**Expected:** Plugin loads; shows in device panel
**Evidence:** Screenshot

### TS-004.4: Ableton Live (Windows)
**Precondition:** Live 11+ installed  
**Steps:**
1. Preferences → Plug-ins → Plug-in Sources → VST3 Custom Folder → Rescan
2. Browser → Plug-ins → VST3 → Periodic Instruments → Drag to track
**Expected:** Plugin loads; device view shows "Periodic Instruments"
**Evidence:** Screenshot

### TS-004.5: Plugin Metadata Verification
**Steps:** In any host, right-click plugin title → Info / About
**Expected:** 
- Name: "Periodic Instruments"
- Vendor: "AlZrSe"
- Version: "0.1.0"
- Unique ID (FUID) displayed
**Evidence:** Screenshot of info dialog

---

## TS-005: Parameter Verification

### TS-005.1: Parameter List Completeness
**Steps:** Open plugin automation lane / parameter list in host
**Expected:** Exactly 5 parameters listed:
1. Element Selector (1–94)
2. Mass (0–1)
3. Reactivity (0–1)
4. State (0–1)
5. Radiance (0–1)
**Evidence:** Screenshot of parameter list

### TS-005.2: Element Selector Range and Default
**Steps:** 
1. Note default value (should be 1 = Hydrogen)
2. Automate from 1 to 94
3. Verify integer steps only (no fractional values)
**Expected:** Default = 1; range 1–94; integer steps
**Evidence:** Automation lane screenshot showing range

### TS-005.3: Macro Knobs Range and Default
**Steps:** For each of Mass, Reactivity, State, Radiance:
1. Note default (should be 0.5)
2. Automate from 0.0 to 1.0
3. Verify smooth continuous range
**Expected:** Default = 0.5; range 0.0–1.0; continuous
**Evidence:** Automation lane screenshots

### TS-005.4: Parameter Automation Recording
**Steps:**
1. Arm track for automation write
2. Play transport
3. Move each parameter knob in plugin UI
4. Stop, switch to automation read
5. Play back
**Expected:** Automation recorded and plays back for all 5 parameters
**Evidence:** Automation lanes showing recorded curves

### TS-005.5: MIDI Learn (if host supports)
**Steps:** Right-click parameter → "Learn MIDI CC" → Move hardware knob/slider
**Expected:** Parameter maps to MIDI CC; movement controls parameter
**Evidence:** Video or screenshot showing MIDI learn working

---

## TS-006: Audio Processing Verification

### TS-006.1: Silence Output Test
**Precondition:** Plugin on track, MIDI notes playing (C3, velocity 100)  
**Steps:**
1. Render 10 seconds of MIDI notes
2. Inspect rendered audio file
**Expected:** All samples = 0.0 (silence) — valid for stub
**Evidence:** Audio statistics (RMS = -inf dB, peak = 0.0)

### TS-006.2: Variable Block Size Test
**Precondition:** DAW set to different buffer sizes  
**Steps:** Test at 64, 128, 256, 512, 1024 samples/block
**Expected:** No crashes, no audio glitches at any block size
**Evidence:** DAW console log for each buffer size

### TS-006.3: Variable Sample Rate Test
**Precondition:** DAW project at different sample rates  
**Steps:** Test at 44100, 48000, 88200, 96000 Hz
**Expected:** No crashes, correct silence output at all rates
**Evidence:** DAW console log for each sample rate

### TS-006.4: Extended Runtime Test
**Steps:** Play project with plugin instantiated for 5 minutes
**Expected:** No memory growth, no crashes, no audio dropouts
**Evidence:** Process monitor memory graph; DAW log

### TS-006.5: Bypass Toggle Test
**Steps:** 
1. Play audio through plugin (silence expected)
2. Toggle host bypass ON → OFF → ON → OFF rapidly (10×)
**Expected:** No clicks, pops, or crashes
**Evidence:** Audio recording of bypass toggles (should be clean silence)

---

## TS-007: Plugin Lifecycle Tests

### TS-007.1: Load/Unload Cycle
**Steps:** 
1. Insert plugin on track
2. Remove plugin from track
3. Repeat 20×
**Expected:** No crashes, no memory leaks
**Evidence:** Memory profiler (valgrind/Dr. Memory) shows no leaks

### TS-007.2: Activate/Deactivate Cycle
**Steps:** 
1. Insert plugin
2. Toggle track "Record Arm" or "Monitor" to trigger activate/deactivate
3. Repeat 20×
**Expected:** No errors in host log
**Evidence:** Host console log

### TS-007.3: Project Save/Load
**Steps:**
1. Create project with plugin on track, parameters set to non-default values
2. Save project
3. Close DAW
4. Reopen DAW, load project
**Expected:** Plugin loads with correct parameter values restored
**Evidence:** Screenshot of parameters after reload matching saved values

---

## TS-008: Editor/UI Tests

### TS-008.1: Editor Opens
**Steps:** Double-click plugin / click "Show Editor" in host
**Expected:** Editor window opens (may be minimal/empty)
**Evidence:** Screenshot

### TS-008.2: Editor Resizable
**Steps:** Drag window corners/edges to resize
**Expected:** Window resizes smoothly; content (if any) redraws correctly
**Evidence:** Video or screenshots at 3 sizes (800×600, 1200×800, 1600×1000)

### TS-008.3: Editor Close/Reopen
**Steps:** Close editor → Reopen editor (5×)
**Expected:** No crashes; window state reasonable
**Evidence:** Host log shows no errors

### TS-008.4: Default Editor Size
**Steps:** Open editor fresh (first time in session)
**Expected:** Default size ≥ 800×600, preferably ~1200×800
**Evidence:** Screenshot with window dimensions visible

---

## TS-009: Cross-Platform Parity

### TS-009.1: Windows ↔ Linux Feature Parity
**Steps:** Run TS-004 through TS-008 on both platforms
**Expected:** All tests pass on both Windows and Linux
**Evidence:** Test results matrix

| Test | Windows | Linux |
|------|---------|-------|
| Build | ✓/✗ | ✓/✗ |
| Load in Reaper | ✓/✗ | ✓/✗ |
| Load in Bitwig | ✓/✗ | ✓/✗ |
| Parameters | ✓/✗ | ✓/✗ |
| Audio process | ✓/✗ | ✓/✗ |
| Editor | ✓/✗ | ✓/✗ |
| Lifecycle | ✓/✗ | ✓/✗ |

---

## TS-010: Negative Tests (Should Fail Gracefully)

### TS-010.1: Invalid VST3 Path
**Steps:** Copy `.vst3` to non-standard location, don't add to host scan path
**Expected:** Host does not show plugin
**Evidence:** Plugin not in browser

### TS-010.2: Corrupted Bundle
**Steps:** Delete `moduleinfo.json` → rescan
**Expected:** Host ignores/rejects plugin gracefully (no crash)
**Evidence:** Host log shows warning, not crash

---

## Test Execution Checklist

| Scenario | Windows | Linux | Evidence Collected |
|----------|---------|-------|-------------------|
| TS-001.1 | ☐ | N/A | Build log |
| TS-001.2 | N/A | ☐ | Build log |
| TS-001.3 | ☐ | ☐ | Check output |
| TS-001.4 | ☐ | ☐ | cargo tree output |
| TS-002.1 | ☐ | ☐ | File listing |
| TS-002.2 | ☐ | ☐ | mod.rs contents |
| TS-003.1 | ☐ | N/A | Dir listing + JSON |
| TS-003.2 | N/A | ☐ | Dir listing + JSON |
| TS-003.3 | ☐ | ☐ | JSON content |
| TS-004.1 | ☐ | N/A | Screenshot |
| TS-004.2 | N/A | ☐ | Screenshot |
| TS-004.3 | ☐ | ☐ | Screenshot |
| TS-004.4 | ☐ | N/A | Screenshot |
| TS-004.5 | ☐ | ☐ | Screenshot |
| TS-005.1 | ☐ | ☐ | Screenshot |
| TS-005.2 | ☐ | ☐ | Screenshot |
| TS-005.3 | ☐ | ☐ | Screenshot |
| TS-005.4 | ☐ | ☐ | Screenshot |
| TS-005.5 | ☐ | ☐ | Video/Screenshot |
| TS-006.1 | ☐ | ☐ | Audio stats |
| TS-006.2 | ☐ | ☐ | Console logs |
| TS-006.3 | ☐ | ☐ | Console logs |
| TS-006.4 | ☐ | ☐ | Memory graph |
| TS-006.5 | ☐ | ☐ | Audio recording |
| TS-007.1 | ☐ | ☐ | Profiler output |
| TS-007.2 | ☐ | ☐ | Host log |
| TS-007.3 | ☐ | ☐ | Before/after screenshots |
| TS-008.1 | ☐ | ☐ | Screenshot |
| TS-008.2 | ☐ | ☐ | Screenshots |
| TS-008.3 | ☐ | ☐ | Host log |
| TS-008.4 | ☐ | ☐ | Screenshot |
| TS-009.1 | ☐ | ☐ | Results matrix |
| TS-010.1 | ☐ | ☐ | Browser check |
| TS-010.2 | ☐ | ☐ | Host log |

---

## Pass/Fail Criteria

**Task passes QA** if:
- All ☐ checkboxes marked ✓ (pass)
- No critical failures (crashes, build failures, plugin not loading)
- At least 2 DAWs tested successfully
- Both Windows and Linux verified (or CI covers both)

**Task fails QA** if:
- Any critical failure
- Plugin doesn't load in any DAW
- Build fails on either platform
- Parameters missing or incorrect
- Audio thread allocations detected

---

## Notes for QA Engineer
- Use **fresh DAW projects** for each test to avoid cached state
- Test with **actual MIDI input** (virtual or hardware) not just piano roll
- Record **screen capture** for TS-004, TS-005, TS-008 as evidence
- Run **memory profiler** for TS-006.4 and TS-007.1 (valgrind on Linux, Dr. Memory on Windows)
- Document **exact DAW versions** tested
- Report **any warnings** in host console even if not failures