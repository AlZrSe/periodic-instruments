# User Stories — Task 001: Foundation Application (Basic VST3 Support)

## Context
This is the foundational task for the Periodic Instruments VST3 plugin. It establishes the Rust project structure, build system, and minimal VST3 plugin that can load in a DAW. All subsequent tasks (element implementations, DSP engines, UI, effects, etc.) depend on this foundation.

---

## User Stories

### US-001: Developer Project Setup
**As a** developer joining the project  
**I want** a Cargo.toml with all required dependencies (nice-plug, iced, VST3 SDK, etc.) and a working build configuration  
**So that** I can clone the repo and run `cargo build --release` to produce a working VST3 plugin without manual configuration

**Details:**
- Dependencies must include: `nice-plug` (plugin framework), `iced` (GUI via nice-plug), `vst3-sys` or equivalent VST3 SDK bindings
- Build must work on Windows (MSVC) and Linux (GNU)
- VST3 SDK path should be configurable via environment variable or Cargo config

---

### US-002: Minimal VST3 Plugin Loads in DAW
**As a** plugin developer / DAW user  
**I want** the built VST3 plugin to be recognized and load successfully in any VST3-compatible DAW (Reaper, Bitwig, Ableton Live, Cubase, etc.)  
**So that** I can verify the foundation works before adding DSP/UI complexity

**Details:**
- Plugin must register with VST3 category "Instrument | Synth"
- Plugin name must appear as "Periodic Instruments" in DAW plugin browser
- Plugin must not crash on instantiation, initialization, or termination
- Plugin must support basic VST3 plugin lifecycle (initialize, setupProcessing, setActive, terminate)

---

### US-003: Basic Parameter Structure Exists
**As a** plugin developer  
**I want** a basic parameter structure in place (element selector, four macro knobs)  
**So that** subsequent tasks can extend parameters without refactoring the parameter system

**Details:**
- Parameter 0: Element Selector (atomic number 1–94, default Hydrogen = 1)
- Parameter 1: Mass macro (0.0–1.0, default 0.5)
- Parameter 2: Reactivity macro (0.0–1.0, default 0.5)
- Parameter 3: State macro (0.0–1.0, default 0.5)
- Parameter 4: Radiance macro (0.0–1.0, default 0.5)
- All parameters must be automatable and support MIDI learn
- Parameter info (name, label, units, range, default) must be properly exposed to host

---

### US-004: Project Structure Matches Architecture
**As a** developer  
**I want** the source tree to match the planned architecture (`src/core`, `src/elements`, `src/fx`, `src/ui`, `src/plugin`)  
**So that** code organization is consistent from day one and team members know where to add code

**Details:**
- `src/core/` — DSP core, voice manager, modulation (empty modules OK for now)
- `src/elements/` — per-element definitions & factory (empty modules OK for now)
- `src/fx/` — effects chain implementations (empty modules OK for now)
- `src/ui/` — plugin UI and periodic-table view (empty modules OK for now)
- `src/plugin/` — VST3 wrapper, entry point (contains the actual plugin implementation)
- Each directory should have a `mod.rs` declaring its modules

---

### US-005: Audio Processing Stub Works
**As a** plugin developer  
**I want** the audio process callback to exist and pass silence without crashing  
**So that** the plugin produces valid (silent) audio output and can be tested in a DAW signal chain

**Details:**
- `process()` callback must handle input/output buffers correctly
- Must support variable block sizes and sample rates
- Must be allocation-free on audio thread (no `Vec::new`, `Box::new`, etc. in process)
- Must handle bypass state correctly (pass-through or silence)

---

### US-006: GUI Entry Point Exists
**As a** developer  
**I want** the iced-based GUI entry point to be wired into nice-plug  
**So that** the UI can be built incrementally in later tasks

**Details:**
- GUI doesn't need to render anything yet — just the view/editor registration
- Must use nice-plug's iced integration
- Editor size should be resizable (e.g., default 1200×800, min 800×600)

---

## Non-Functional Requirements

| Requirement | Detail |
|-------------|--------|
| **Build reproducibility** | `cargo build --release` must succeed on clean clone |
| **Cross-platform** | Windows (x86_64-pc-windows-msvc) and Linux (x86_64-unknown-linux-gnu) |
| **VST3 SDK** | Must use official Steinberg VST3 SDK (not a fork) |
| **Rust version** | Stable Rust (MSRV: latest stable at project start) |
| **No audio-thread allocations** | Enforced by code review and clippy lints |
| **Plugin ID** | Unique VST3 class ID (FUID) for "Periodic Instruments" |
| **Versioning** | Plugin version 0.1.0 (semver) |

---

## Out of Scope for This Task
- Any actual synthesis/DSP engines (subtractive, FM, wavetable, granular, physical modeling)
- Per-element sound definitions or factory
- Effects chain implementations
- Periodic table UI view
- Preset browser
- MIDI learn UI / automation UI
- MPE support
- Resource loading (samples, wavetables, IRs)
- Tests beyond "builds and loads"

---

## Dependencies
- None (this is the first task)
- Blocks all subsequent tasks

---

## Definition of Ready (for SWE)
- [ ] This spec is reviewed and approved
- [ ] VST3 SDK is available in the build environment
- [ ] Rust toolchain is installed
- [ ] Target DAW for testing is identified