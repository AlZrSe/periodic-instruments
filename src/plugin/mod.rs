//! VST3 plugin entry point and implementation using nih-plug

pub mod factory;
pub mod parameters;
pub mod processor;
pub mod controller;
pub mod editor;

use nih_plug::prelude::*;
use crate::plugin::factory::{PeriodicInstrumentsFactory, PeriodicInstrumentsPlugin};
use crate::plugin::controller::PeriodicInstrumentsController;
use crate::plugin::processor::PeriodicInstrumentsProcessor;
use crate::plugin::editor::PeriodicInstrumentsEditor;

/// Plugin metadata constants
pub const PLUGIN_NAME: &str = "Periodic Instruments";
pub const PLUGIN_VENDOR: &str = "AlZrSe";
pub const PLUGIN_VERSION: &str = "0.1.0";
pub const PLUGIN_CATEGORY: &str = "Instrument|Synth";
pub const PLUGIN_UNIQUE_ID: &str = "com.alzrse.periodicinstruments";

/// Unique FUID for the plugin class (generated once, must never change)
/// Generated from: uid::from("Periodic Instruments by AlZrSe")
pub const PLUGIN_CLASS_ID: [u8; 16] = [
    0x70, 0x65, 0x72, 0x69,  // "peri"
    0x6F, 0x64, 0x69, 0x63,  // "odic"
    0x49, 0x6E, 0x73, 0x74,  // "Inst"
    0x72, 0x75, 0x6D, 0x65,  // "rume"
    0x6E, 0x74, 0x73, 0x00,  // "nts\0"
];

// nih-plug plugin entry point
nih_plug::vst3::vst3_plugin_main!(PeriodicInstrumentsFactory);