//! VST3 plugin entry point and implementation
//!
//! This module contains the complete VST3 plugin implementation including:
//! - Plugin factory and FUID registration
//! - Parameter definitions (5 parameters)
//! - Audio processor (allocation-free)
//! - GUI editor registration via nice-plug/iced

pub mod factory;
pub mod parameters;
pub mod processor;
pub mod editor;

use nice_plug::prelude::*;
use std::sync::Arc;

// Re-export main types
pub use factory::PeriodicInstrumentsFactory;
pub use parameters::{ParameterId, PARAMETER_COUNT, ParameterInfo};
pub use processor::PeriodicInstrumentsProcessor;
pub use editor::PeriodicInstrumentsEditor;

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

/// Plugin entry point — called by VST3 host
#[no_mangle]
pub extern "C" fn GetPluginFactory() -> *mut dyn PluginFactory {
    let factory = Box::new(PeriodicInstrumentsFactory::new());
    Box::into_raw(factory) as *mut dyn PluginFactory
}

/// Plugin descriptor for nice-plug
pub struct PeriodicInstrumentsPlugin;

impl Plugin for PeriodicInstrumentsPlugin {
    const NAME: &'static str = PLUGIN_NAME;
    const VENDOR: &'static str = PLUGIN_VENDOR;
    const VERSION: &'static str = PLUGIN_VERSION;
    const CATEGORY: &'static str = PLUGIN_CATEGORY;
    const UNIQUE_ID: &'static str = PLUGIN_UNIQUE_ID;
    const CLASS_ID: [u8; 16] = PLUGIN_CLASS_ID;
    
    type Factory = PeriodicInstrumentsFactory;
    type Processor = PeriodicInstrumentsProcessor;
    type Editor = PeriodicInstrumentsEditor;
}