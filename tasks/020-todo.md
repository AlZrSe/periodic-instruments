# Task 020: Pluginval Integration Testing

## Status: TODO

## Description
Set up pluginval-based integration testing for the Periodic Instruments VST3 plugin. This covers the DAW integration tests moved from Task 001.

## Dependencies
- Task 001 (Foundation Application) must be `done` and closed
- Requires built VST3 bundle from Task 001

## Acceptance Criteria
- [ ] pluginval installed and configured in CI
- [ ] Plugin passes pluginval at strictness level 5 (host compatibility baseline)
- [ ] Plugin passes pluginval at strictness level 7 (extended testing)
- [ ] Test runs in headless mode on Windows and Linux CI
- [ ] CI fails if pluginval returns non-zero exit code
- [ ] Variable block size testing via pluginval stress tests
- [ ] Variable sample rate testing via pluginval
- [ ] Parameter automation testing via pluginval
- [ ] State save/restore testing via pluginval
- [ ] MIDI handling testing via pluginval

## Test Scenarios (from Task 001, moved here)

### TS-004: Plugin Discovery & Loading
- Plugin loads in headless host without crash
- Correct category: Instrument|Synth
- Correct name: "Periodic Instruments"
- Unique FUID registered

### TS-005: Parameter Verification
- All 5 parameters visible with correct metadata
- Parameter automation works (ramp, discrete steps)
- MIDI learn functional (parameter mapping)

### TS-006: Audio Processing
- Silence output verified (foundation build)
- Variable block sizes (64, 128, 256, 512, 1024, 2048, 4096)
- Variable sample rates (44.1k, 48k, 88.2k, 96k, 192k)
- Bypass state handled correctly

### TS-007: Plugin Lifecycle
- Instantiate → activate → process → deactivate → terminate
- Project save/load restores parameter values
- Multiple instantiations work independently

### TS-008: Editor/UI
- Editor opens without crash (headless mode)
- Editor bounds/resize handling
- Editor close without crash

## Implementation Notes

### pluginval Setup
```bash
# Download latest release
# Add to PATH or use full path in CI

# Run validation
pluginval --strictness-level 5 path/to/PeriodicInstruments.vst3
pluginval --strictness-level 7 path/to/PeriodicInstruments.vst3
```

### CI Integration (GitHub Actions example)
```yaml
- name: Run pluginval
  run: |
    ./pluginval --strictness-level 5 target/release/PeriodicInstruments.vst3
    ./pluginval --strictness-level 7 target/release/PeriodicInstruments.vst3
```

### Alternative: headless-plugin-host (hph)
- Lightweight alternative for quick smoke tests
- Can be used alongside pluginval for faster feedback

## Labels
- integration
- testing
- pluginval
- ci
- ready-for-grooming