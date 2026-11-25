//! Plugin factory — creates processor and editor instances

use vst3::prelude::*;
use crate::plugin::parameters::{PARAMETER_COUNT, ParameterInfo, parameter_info};
use crate::plugin::processor::PeriodicInstrumentsProcessor;
use crate::plugin::editor::PeriodicInstrumentsEditor;
use crate::plugin::{PLUGIN_NAME, PLUGIN_VENDOR, PLUGIN_VERSION, PLUGIN_CATEGORY, PLUGIN_UNIQUE_ID, PLUGIN_CLASS_ID};

/// Plugin factory implementation
pub struct PeriodicInstrumentsFactory {
    _private: (),
}

impl PeriodicInstrumentsFactory {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for PeriodicInstrumentsFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl vst3::Factory for PeriodicInstrumentsFactory {
    fn create_processor(&self, sample_rate: f32, max_block_size: u32) -> Box<dyn vst3::Processor> {
        Box::new(PeriodicInstrumentsProcessor::new(sample_rate, max_block_size))
    }
    
    fn create_editor(&self) -> Box<dyn vst3::Editor> {
        Box::new(PeriodicInstrumentsEditor::new())
    }
    
    fn parameter_count(&self) -> usize {
        PARAMETER_COUNT
    }
    
    fn parameter_info(&self, index: usize) -> Option<ParameterInfo> {
        parameter_info(index)
    }
    
    fn plugin_info(&self) -> vst3::PluginInfo {
        vst3::PluginInfo {
            name: PLUGIN_NAME,
            vendor: PLUGIN_VENDOR,
            version: PLUGIN_VERSION,
            category: PLUGIN_CATEGORY,
            unique_id: PLUGIN_UNIQUE_ID,
            class_id: PLUGIN_CLASS_ID,
            input_channels: 0,  // Instrument: no audio input
            output_channels: 2, // Stereo output
            midi_inputs: 1,
            midi_outputs: 0,
        }
    }
}