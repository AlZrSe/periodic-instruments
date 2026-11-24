# Task 020: Pluginval Integration Testing

## Status: GROOMED

## Description
Set up pluginval-based integration testing for the Periodic Instruments VST3 plugin. This covers the DAW integration tests moved from Task 001.

## Dependencies
- Task 001 (Foundation Application) must be `done` and closed
- Requires built VST3 bundle from Task 001

## Specifications Created
- `specs/task-020-pluginval/user-stories.md` — 8 user stories (US-020.1 through US-020.8)
- `specs/task-020-pluginval/acceptance-criteria.md` — 8 acceptance criteria groups (AC-020.1 through AC-020.8 + NF-AC)
- `specs/task-020-pluginval/test-scenarios.md` — 30+ test scenarios covering both platforms

## Acceptance Criteria (from specs)

### AC-020.1: pluginval Installation and Configuration in CI
- Workflow file at `.github/workflows/pluginval.yml`
- Triggers on push/PR
- Windows job on `runs-on: [self-hosted, win]`
- Linux job on `runs-on: [self-hosted, linux]`
- pluginval binary downloaded from official releases (pinned version)
- Binary cached via `actions/cache`
- VST3 bundle from Task 001 artifact

### AC-020.2: Plugin Passes pluginval Strictness Level 5
- Exit code 0 on Windows and Linux
- Plugin discovery, factory loading, class registration
- Initialize/terminate, setup processing, set active
- Parameter enumeration (5 params)
- Basic audio processing
- Editor creation (headless)

### AC-020.3: Plugin Passes pluginval Strictness Level 7
- Exit code 0 on Windows and Linux
- State save/restore round-trip
- Parameter automation (all 5 params)
- MIDI event handling (note, CC, pitch bend)
- Variable block sizes (64–4096)
- Variable sample rates (44.1k–192k)
- Bypass handling
- Multiple instantiation

### AC-020.4: Headless Execution on Both Platforms
- Windows runs without GUI dependencies
- Linux runs via Xvfb or native headless
- Fully automated, no manual intervention

### AC-020.5: Variable Block Size and Sample Rate Coverage
- Block sizes: 64, 128, 256, 512, 1024, 2048, 4096
- Sample rates: 44100, 48000, 88200, 96000, 192000

### AC-020.6: Parameter Automation and MIDI Handling
- All 5 parameters automated through range
- MIDI note on/off, CC for macros, pitch bend

### AC-020.7: State Save/Restore Verification
- getState/setState round-trip
- All 5 parameter values restored exactly
- Program/bank change persistence

### AC-020.8: CI Gate — Hard Failure on Validation Error
- Workflow fails (red) on non-zero exit code
- Failure on either platform fails overall workflow
- Logs uploaded as artifacts on failure
- No `continue-on-error: true`

## Non-Functional Criteria
- pluginval version pinned (e.g., v0.3.8)
- Total CI time < 10 minutes
- Cache key includes version and OS
- Least-privilege permissions
- Uses Task 001 build artifact (not rebuilt)

## Labels
- integration
- testing
- pluginval
- ci
- groomed

## Next Step
Ready for SWE implementation. Orchestrator should:
1. Update GitHub issue #20 label to `in-progress`
2. Set `running` label on issue #20
3. Spawn SWE agent with this spec