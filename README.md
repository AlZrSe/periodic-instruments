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

Each element also carries a **human feel** tied to where it is used in production: the way it sits in a mix, the genre or role it naturally fits, and the expressive playing style it invites (plucked, sustained, percussive, evolving, ...). This keeps the library musical and usable, not just a science demo.

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

## Instrument Roster (selected)


| Element    | Symbol | Category         | Sound & Human Feel (production use)                       | Signature Effects               |
| ---------- | ------ | ---------------- | --------------------------------------------------------- | ------------------------------- |
| Hydrogen   | H      | Nonmetal         | Airy high sine pad; ethereal top-end layers, ambient beds | Shimmer reverb, high-pass       |
| Helium     | He     | Noble gas        | Breath-like noise pad; light, floating textures, intros   | Chorus, gentle delay            |
| Lithium    | Li     | Alkali metal     | Bright pluck; indie/electronic hooks, staccato lines      | Short plate reverb              |
| Beryllium  | Be     | Alkaline earth   | Crisp mallet; percussion-like accents, bright transients  | Tight room reverb               |
| Boron      | B      | Metalloid        | Glassy FM bell; melodic leads, chime arpeggios            | Delay, bitcrush (light)         |
| Carbon     | C      | Nonmetal         | Warm acoustic-ish tone; rhythm parts, strummed pads       | Chorus, tape saturation         |
| Nitrogen   | N      | Nonmetal         | Cold slow-attack pad; cinematic drones, dark atmospheres  | Long hall reverb                |
| Oxygen     | O      | Nonmetal         | Breath & air textures; vocal-like pads, spacious mixes    | Phaser, stereo widener          |
| Fluorine   | F      | Halogen          | Sharp biting lead; aggressive synth leads, EDM hooks      | Distortion, high-pass           |
| Neon       | Ne     | Noble gas        | Glowing sine lead; retro/synthwave melodies               | Delay, chorus                   |
| Sodium     | Na     | Alkali metal     | Salty bright pluck; funky bass lines, plucky chords       | Spring reverb                   |
| Magnesium  | Mg     | Alkaline earth   | Light metallic ping; hi-hat-like accents, bright hits     | Short delay                     |
| Aluminium  | Al     | Post-transition  | Smooth metallic pad; clean modern pads, pop ballads       | Flanger                         |
| Silicon    | Si     | Metalloid        | Digital crystal tone; chiptune, glitch, digital textures  | Bitcrush, delay                 |
| Phosphorus | P      | Nonmetal         | Glowing organic pad; warm evolving pads, ambient          | Tremolo, reverb                 |
| Sulfur     | S      | Nonmetal         | Gritty acidic lead; industrial, aggressive sequences      | Distortion, filter sweep        |
| Chlorine   | Cl     | Halogen          | Sharp pungent stab; stabs, risers, transition FX          | Ring mod, delay                 |
| Argon      | Ar     | Noble gas        | Deep inert drone; bass drones, sub foundations            | Low-pass, sub bass              |
| Potassium  | K      | Alkali metal     | Soft warm keys; piano-like chords, singer-songwriter      | Tape saturation                 |
| Calcium    | Ca     | Alkaline earth   | Bone-dry percussive; dry drums, tight grooves             | Room reverb                     |
| Iron       | Fe     | Transition metal | Heavy metallic clang; rock/metal riffs, impact hits       | Distortion, convolution reverb  |
| Copper     | Cu     | Transition metal | Warm conductive tone; soul/R&B chords, smooth leads       | Chorus, analog delay            |
| Silver     | Ag     | Transition metal | Bright shimmering bell; high melodies, sparkly tops       | Shimmer reverb                  |
| Gold       | Au     | Transition metal | Rich golden pad; lush pads, luxury/cinematic feel         | Plate reverb, chorus            |
| Mercury    | Hg     | Transition metal | Flowing liquid glide; portamento leads, smooth bass       | Phaser, flanger                 |
| Lead       | Pb     | Post-transition  | Heavy dark bass; sub bass, dark trap/hip-hop              | Saturation, low-pass            |
| Uranium    | U      | Actinide         | Rumbling decaying drone; dark cinematic, tension          | Granular, long reverb tail      |
| Plutonium  | Pu     | Actinide         | Dark unstable drone; unstable risers, horror/sci-fi       | Granular, distortion, long tail |


> The full 94-element roster is available in the plugin's preset browser and in `docs/elements.md`.

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