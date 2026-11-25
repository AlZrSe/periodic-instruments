//! Audio processor implementation for Periodic Instruments VST3 plugin

use nih_plug::prelude::*;
use std::sync::Arc;
use crate::plugin::parameters::ParameterValues;

/// Maximum block size supported
const MAX_BLOCK_SIZE: usize = 4096;

/// Periodic Instruments audio processor - outputs silence (foundation build)
pub struct PeriodicInstrumentsProcessor {
    param_values: Arc<ParameterValues>,
    sample_rate: f32,
    max_block_size: usize,
    bypass: bool,
    silence_buffer: Vec<f32>,
}

impl PeriodicInstrumentsProcessor {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            param_values: Arc::new(ParameterValues::new()),
            sample_rate,
            max_block_size: MAX_BLOCK_SIZE,
            bypass: false,
            silence_buffer: vec![0.0f32; MAX_BLOCK_SIZE],
        }
    }

    pub fn get_param_values(&self) -> Arc<ParameterValues> {
        self.param_values.clone()
    }
}

impl Processor for PeriodicInstrumentsProcessor {
    fn process(&mut self, buffer: &mut Buffer, _context: &mut ProcessContext) -> Result<(), PluginError> {
        // Handle bypass
        if self.bypass {
            // Pass-through: copy inputs to outputs
            if let (Some(inputs), Some(outputs)) = (buffer.inputs(), buffer.outputs()) {
                for (in_ch, out_ch) in inputs.iter().zip(outputs.iter_mut()) {
                    out_ch.copy_from_slice(in_ch);
                }
            }
            return Ok(());
        }

        // Instrument mode: output silence (foundation build)
        if let Some(outputs) = buffer.outputs() {
            for out_ch in outputs.iter_mut() {
                out_ch.fill(0.0);
            }
        }

        Ok(())
    }

    fn initialize(&mut self, context: &InitializeContext) -> Result<(), PluginError> {
        self.sample_rate = context.sample_rate;
        self.max_block_size = context.max_block_size;
        self.silence_buffer.resize(self.max_block_size, 0.0);
        Ok(())
    }

    fn reset(&mut self) {
        // Reset state
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    fn get_tail_size(&self) -> usize {
        0 // No tail for silence output
    }
}