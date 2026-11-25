//! VST3 Plugin Factory implementation using nih-plug

use nih_plug::prelude::*;
use std::sync::Arc;
use crate::plugin::processor::PeriodicInstrumentsProcessor;
use crate::plugin::controller::PeriodicInstrumentsController;

/// Periodic Instruments VST3 Plugin Factory
#[derive(Default)]
pub struct PeriodicInstrumentsFactory;

impl PluginFactory for PeriodicInstrumentsFactory {
    fn new_plugin(&self, sample_rate: f32) -> Box<dyn Plugin> {
        let processor = PeriodicInstrumentsProcessor::new(sample_rate);
        let controller = PeriodicInstrumentsController::new(Arc::new(processor));
        Box::new(PeriodicInstrumentsPlugin { controller })
    }

    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Periodic Instruments".to_string(),
            vendor: "AlZrSe".to_string(),
            unique_id: vst3_sys::generate_uid!("PeriodicInstrum"),
            version: (0, 1, 0),
            category: PluginCategory::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 5,
            programs: 0,
            preset_chunks: false,
            midi_input: true,
            midi_output: false,
            editor: true,
            ..Default::default()
        }
    }
}

/// Wrapper plugin that holds the controller
pub struct PeriodicInstrumentsPlugin {
    controller: Arc<PeriodicInstrumentsController>,
}

impl Plugin for PeriodicInstrumentsPlugin {
    fn get_controller(&self) -> Arc<dyn Controller> {
        self.controller.clone()
    }

    fn get_processor(&self) -> Arc<dyn Processor> {
        // The controller holds the processor
        self.controller.get_processor()
    }
}