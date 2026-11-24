# Acceptance Criteria — Task 001: Foundation Application (Basic VST3 Support)

## Overview
Each criterion is traceable to a User Story (US-XXX) and must be verifiable by QA with objective evidence (build logs, DAW screenshots, parameter dumps).

---

## AC-001: Cargo.toml and Build Configuration
**Traceability:** US-001 (Developer Project Setup)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-001.1 | `Cargo.toml` exists at repo root with `[package]` metadata (name, version, edition, authors) | File inspection | `cat Cargo.toml` |
| AC-001.2 | Dependencies include: `nice-plug` (with `vst3` and `iced` features), `vst3-sys`, `nih-plug` or equivalent VST3 framework | File inspection | `grep -A 20 '\[dependencies\]' Cargo.toml` |
| AC-001.3 | Build profile `release` configured with `lto = true`, `opt-level = 3`, `codegen-units = 1` | File inspection | `grep -A 10 '\[profile.release\]' Cargo.toml` |
| AC-001.4 | `cargo build --release` completes without errors on clean clone (Windows MSVC) | Command execution | Build log showing `Finished release [optimized] target(s)` |
| AC-001.5 | `cargo build --release` completes without errors on clean clone (Linux GNU) | Command execution (CI) | CI pipeline log |
| AC-001.6 | VST3 SDK path configurable via `VST3_SDK` env var or `build.rs` | File inspection + test | `build.rs` reads `VST3_SDK` or `vst3-sys` finds SDK |

---

## AC-002: VST3 Plugin Loads in DAW
**Traceability:** US-002 (Minimal VST3 Plugin Loads in DAW)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-002.1 | Build produces `PeriodicInstruments.vst3` bundle (Windows: `.dll` in `Contents/x86_64-win/`, Linux: `.so` in `Contents/x86_64-linux/`) | File inspection | `ls target/release/*.vst3/Contents/` |
| AC-002.2 | Plugin registers with VST3 category `Instrument | Synth` (subcategory `Synth`) | Host plugin scanner / `vst3plugininspector` | Scanner output showing category |
| AC-002.3 | Plugin name displayed as "Periodic Instruments" in DAW plugin browser | DAW test (Reaper, Bitwig, or vst3plugininspector) | Screenshot or inspector output |
| AC-002.4 | Plugin instantiates without crash (constructor, `initialize`, `setupProcessing`) | DAW test + debug log | DAW log / no crash dump |
| AC-002.5 | Plugin activates/deactivates (`setActive(true/false)`) without crash | DAW test (enable/disable plugin) | DAW log |
| AC-002.6 | Plugin terminates cleanly (`terminate()`) on track deletion or DAW close | DAW test | DAW log |
| AC-002.7 | Unique VST3 class ID (FUID) assigned and registered in `pluginfactory` | Code inspection | `src/plugin/factory.rs` or equivalent |

---

## AC-003: Basic Parameter Structure
**Traceability:** US-003 (Basic Parameter Structure Exists)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-003.1 | Parameter 0: Element Selector — integer 1–94, default 1 (Hydrogen), label "Element", unit "" | Parameter dump / DAW automation lane | Host parameter list |
| AC-003.2 | Parameter 1: Mass — float 0.0–1.0, default 0.5, label "Mass", unit "" | Parameter dump / DAW automation lane | Host parameter list |
| AC-003.3 | Parameter 2: Reactivity — float 0.0–1.0, default 0.5, label "Reactivity", unit "" | Parameter dump / DAW automation lane | Host parameter list |
| AC-003.4 | Parameter 3: State — float 0.0–1.0, default 0.5, label "State", unit "" | Parameter dump / DAW automation lane | Host parameter list |
| AC-003.5 | Parameter 4: Radiance — float 0.0–1.0, default 0.5, label "Radiance", unit "" | Parameter dump / DAW automation lane | Host parameter list |
| AC-003.6 | All 5 parameters exposed to host as automatable (`ParameterInfo::canAutomate = true`) | Code inspection + host verification | Automation lane visible in DAW |
| AC-003.7 | All 5 parameters support MIDI learn (CC mapping) | DAW test (MIDI learn workflow) | MIDI CC assignment works |
| AC-003.8 | Parameter info includes: min, max, default, step count (for discrete), flags | Code inspection | `ParameterInfo` struct population |

---

## AC-004: Project Structure Matches Architecture
**Traceability:** US-004 (Project Structure Matches Architecture)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-004.1 | Directory `src/core/` exists with `mod.rs` | File inspection | `ls src/core/` |
| AC-004.2 | Directory `src/elements/` exists with `mod.rs` | File inspection | `ls src/elements/` |
| AC-004.3 | Directory `src/fx/` exists with `mod.rs` | File inspection | `ls src/fx/` |
| AC-004.4 | Directory `src/ui/` exists with `mod.rs` | File inspection | `ls src/ui/` |
| AC-004.5 | Directory `src/plugin/` exists with `mod.rs` (contains VST3 entry point) | File inspection | `ls src/plugin/` |
| AC-004.6 | `src/lib.rs` or `src/main.rs` declares all modules and re-exports public API | File inspection | `cat src/lib.rs` |

---

## AC-005: Audio Processing Stub
**Traceability:** US-005 (Audio Processing Stub Works)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-005.1 | `process()` callback implemented with correct signature (input/output buffers, event list) | Code inspection | `src/plugin/processor.rs` or equivalent |
| AC-005.2 | Process handles variable block sizes (1–4096 samples) without crash | DAW test (change buffer size) | DAW log / no crash |
| AC-005.3 | Process handles sample rates 44.1k, 48k, 88.2k, 96k, 192k | DAW test (change project sample rate) | DAW log / no crash |
| AC-005.4 | Zero allocations on audio thread (no `Vec::new`, `Box::new`, `String::new`, format! in hot path) | Code inspection + `cargo clippy` | Clippy clean on `allocation_free` lints |
| AC-005.5 | Bypass state handled: when bypassed, output = input (pass-through) or silence (instrument) | DAW test (toggle bypass) | Audio passes through or silences |
| AC-005.6 | Output buffers are cleared (silence) when not bypassed (instrument produces no sound yet) | DAW test (measure output) | Silent output verified |

---

## AC-006: GUI Entry Point
**Traceability:** US-006 (GUI Entry Point Exists)

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| AC-006.1 | Editor registration in plugin factory (`createView` returns `IPlugView*`) | Code inspection | `src/plugin/editor.rs` or equivalent |
| AC-006.2 | Uses nice-plug's iced integration (`nice_plug::iced::IcedEditor`) | Code inspection | Import and usage in editor module |
| AC-006.3 | Default editor size 1200×800, minimum 800×600 | Code inspection | `size_hint` or equivalent |
| AC-006.4 | Editor opens in DAW without crash (empty view OK) | DAW test (open plugin UI) | Screenshot of empty window |

---

## Non-Functional Acceptance Criteria

| # | Criterion | Verification Method | Evidence Required |
|---|-----------|---------------------|-------------------|
| NF-AC-001 | Plugin version 0.1.0 in `Cargo.toml` and VST3 metadata | File inspection | `version = "0.1.0"` |
| NF-AC-002 | Unique FUID generated and used (not placeholder) | Code inspection | `uid::from("...")` or equivalent |
| NF-AC-003 | `cargo clippy --all-targets --all-features -- -D warnings` passes | Command execution | Clippy output |
| NF-AC-004 | `cargo fmt --check` passes | Command execution | fmt output |
| NF-AC-005 | No `unsafe` code outside VST3 FFI boundaries (or documented justification) | Code inspection | `grep -r unsafe src/` |

---

## Definition of Done (Task 001)
All AC-XXX and NF-AC-XXX criteria **must pass** for QA to mark this task as verified. PM acceptance review follows QA verification.