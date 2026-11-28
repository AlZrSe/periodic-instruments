# Periodic Instruments

A single VST3 plugin that turns **every chemical element up to Plutonium into a playable instrument**.
All 94 elements live inside one plugin: pick an element and it becomes its own instrument, with a unique sound engine, timbre, and built-in effects chain, derived from the element's physical and chemical properties.

> 94 elements. One plugin. One periodic table of sound — from Hydrogen to Plutonium.

## Concept

`periodic-instruments` maps the periodic table onto a complete virtual-instrument library inside a single VST3. The sound of each element is derived from its properties, with all mappings working together:

- **Frequency of emission light** — the characteristic spectral lines of the element (its emission spectrum) are translated into the fundamental frequencies, harmonics, and modulation rates of the instrument.
- **Atomic number** drives pitch range, oscillator count, and modulation depth.
- **Element category** (metal, nonmetal, noble gas, halogen, lanthanide, actinide, ...) selects the base synthesis engine.
- **Physical state at STP** (solid / liquid / gas) shapes the envelope and transient character.
- **Electronegativity & density** map to brightness, weight, and spatial width.
- **Each element** gets a signature effects chain (reverb, delay, distortion, chorus, filter, ...) tuned to its personality.

As a result, **elements sharing a group (column) of the periodic table, as well as the lanthanides, sound similar but never identical** — they share a family resemblance in timbre and behavior, yet each retains its own distinct voice.

Hydrogen is a single instrument with a **dual mode** (alkali-like / halogen-like) rather than two separate entries. The actinides live only in the f-block and are not duplicated in groups 4–7.

Each element also carries a **human feel** tied to where it is used in production: the way it sits in a mix, the genre or role it naturally fits, and the expressive playing style it invites (plucked, sustained, percussive, evolving, ...). This keeps the library musical and usable, not just a science demo.

### Property → DSP mapping

| Property | DSP parameter |
|---|---|
| Atomic mass | Decay length, low-end weight, sub amount, density (pitch stays in musical range via octave-constrained mapping) |
| Electronegativity | Filter brightness/aggression, attack speed |
| Reactivity | Envelope speed, probability of random modulation |
| Radioactivity | Granular processing, random LFO, pitch instability |
| Magnetism | Resonance, pitch glide, harmonic "attraction" |
| Emission color (glow) | Formant/spectral shift: greenish = upper-mid peak, red = low-mid warmth, blue = high airy shift, pink = soft formant shimmer |
| Valence | Harmonic count / layer count |

Metaphors used in the roster (salty, toxic, glowing, magnetic, ...) are shorthand for the DSP treatments above, not just flavor text.

## Features

- **One VST3 plugin** containing all 94 elements (H through Pu), selectable inside the plugin.
- **Element-derived synthesis**: subtractive, FM, wavetable, granular, and physical-modeling engines selected per category.
- **Per-element effects chain** with unique default routing and macro controls.
- **MIDI learn** and full parameter automation for every control.
- **Polyphonic** with up to 64 voices, **MPE** support where applicable.
- **Low CPU** footprint; DSP core written in Rust.
- **Preset browser** organized by group, period, and block of the periodic table.
- **Resizable UI** built with **iced** (via nice-plug), with a periodic-table overview for quick element switching.
- **Human feel / production role** tagging for every element.

## Instrument Roster

The full 94-element roster (sound, signature effects, and human feel / production use for every element) is documented in [`docs/elements.md`](docs/elements.md).

## Requirements

- **OS**: Windows 10/11, or Linux (Ubuntu 20.04+).
- **DAW**: Any host supporting VST3.
- **CPU**: x86_64 with SSE2; ARM64 where supported.
- **RAM**: 4 GB minimum, 8 GB recommended.
- **Disk**: ~2 GB for the full pack.

## Usage

1. Place the VST3 plugin in your DAW's VST3 folder.
2. Rescan plugins in your DAW.
3. The plugin appears under `Instruments / Periodic Instruments`.
4. Create an instrument track and load `Periodic Instruments`.
5. Select an element from the periodic-table view — it becomes the active instrument.
6. Play MIDI notes — the element's default patch loads automatically.
7. Open the **FX** tab to tweak or bypass the element's signature effects chain.
8. Use the **Periodic Table** view to switch elements without leaving the plugin window.
9. Browse presets by group (alkali metals, halogens, lanthanides, ...) or period.

### Macros

Each element exposes four macro knobs:

- **Mass** — weight, low-end, sustain.
- **Reactivity** — brightness, modulation, transient sharpness.
- **State** — envelope shape (gas = airy, liquid = flowing, solid = percussive).
- **Radiance** — effects wetness and spatial width.

## Building from Source

```bash
git clone https://github.com/AlZrSe/periodic-instruments.git
cd periodic-instruments
```

Prerequisites: Rust (stable), Cargo, and the VST3 SDK. The project uses
[nice-plug](https://codeberg.org/RustAudio/nice-plug) as its plugin framework
with an **iced** GUI.

```bash
cargo build --release
```

Built plugins are output to `target/release/`.

### Testing

#### Headless Plugin Host Tests (CI Smoke Tests)

Fast VST3 plugin validation (< 2 min) using `vst3-host` with process isolation:

```bash
# Build and run all headless tests
./scripts/test-headless.sh

# Or with just
just test-headless

# Custom plugin path and JUnit output
./scripts/test-headless.sh /path/to/plugin.vst3 --junit-output results.xml
```

On Windows:
```cmd
scripts\test-headless.bat
```

Tests cover:
- Plugin discovery and instantiation
- Audio processing at block sizes 64–4096, sample rates 44.1–192 kHz
- Output validation (silence, NaN/Inf, denormals)
- Parameter enumeration, get/set, automation
- State save/restore (preset chunks)
- MIDI event handling (Note On/Off, CC, Pitch Bend)
- Editor/view creation (headless)
- Process isolation and crash safety

Outputs JUnit XML (`test-results-local.xml`) and human-readable summary.

#### Unit Tests

```bash
cargo test --all-features
```

#### Code Quality

```bash
just clippy   # Linting
just fmt      # Format check
just audit    # Security audit
```

### Project Layout

```
periodic-instruments/
├── Cargo.toml
├── src/
│   ├── core/           # DSP core, voice manager, modulation
│   ├── elements/       # Per-element definitions & factory
│   ├── fx/             # Effects chain implementations
│   ├── ui/             # Plugin UI and periodic table view
│   └── plugin/         # VST3 wrapper
├── resources/          # Samples, wavetables, IRs
├── docs/               # Element roster, DSP notes
└── tests/
```

## Roadmap

- [ ] Foundation application: basic VST3 support and interaction with the DAW.
- [ ] Add elements constantly with each release (H through Pu).
- [ ] MPE and micro-tuning support.
- [ ] Standalone app mode.
- [ ] Preset sharing / cloud library.

## License

MIT License. See `LICENSE` for details.

## Credits

Created by **AlZrSe**. Inspired by the periodic table and the idea that every element has a voice.

---

*Periodic Instruments — hear the elements.*
