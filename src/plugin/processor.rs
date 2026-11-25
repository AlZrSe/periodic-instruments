//! Audio processor implementation for Periodic Instruments VST3 plugin

use vst3::processor::{Processor, AudioBus, AudioBusType, SpeakerArrangement, ProcessData, ProcessMode, SymbolicSampleSizes};
use vst3::error::Error;
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
    pub fn new() -> Self {
        Self {
            param_values: Arc::new(ParameterValues::new()),
            sample_rate: 44100.0,
            max_block_size: MAX_BLOCK_SIZE,
            bypass: false,
            silence_buffer: vec![0.0f32; MAX_BLOCK_SIZE],
        }
    }

    pub fn get_param_values(&self) -> Arc<ParameterValues> {
        self.param_values.clone()
    }
}

impl Default for PeriodicInstrumentsProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for PeriodicInstrumentsProcessor {
    fn initialize(&mut self, _context: vst3::context::InitializeContext) -> Result<(), Error> {
        Ok(())
    }

    fn terminate(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn setup_processing(&mut self, setup: vst3::context::ProcessingSetup) -> Result<(), Error> {
        if setup.max_samples_per_block > MAX_BLOCK_SIZE as u32 {
            return Err(Error::InvalidArgument);
        }
        if setup.symbolic_sample_size != SymbolicSampleSizes::Sample32 {
            return Err(Error::InvalidArgument);
        }
        
        self.sample_rate = setup.sample_rate;
        self.max_block_size = setup.max_samples_per_block as usize;
        self.silence_buffer.resize(self.max_block_size, 0.0);
        Ok(())
    }

    fn set_active(&mut self, active: bool) -> Result<(), Error> {
        if active {
            // Reset state on activation
        }
        Ok(())
    }

    fn process(&mut self, data: &mut ProcessData) -> Result<(), Error> {
        // Handle bypass
        if self.bypass {
            // Pass-through: copy inputs to outputs
            if let (Some(inputs), Some(outputs)) = (data.inputs.as_mut(), data.outputs.as_mut()) {
                if let (Some(in_bus), Some(out_bus)) = (inputs.first_mut(), outputs.first_mut()) {
                    let num_samples = data.num_samples as usize;
                    if num_samples <= in_bus.channel_buffers32.len() && num_samples <= out_bus.channel_buffers32.len() {
                        for ch in 0..in_bus.channel_buffers32.len().min(out_bus.channel_buffers32.len()) {
                            out_bus.channel_buffers32[ch][..num_samples]
                                .copy_from_slice(&in_bus.channel_buffers32[ch][..num_samples]);
                        }
                    }
                }
            }
            return Ok(());
        }

        // Instrument mode: output silence (foundation build)
        if let Some(outputs) = data.outputs.as_mut() {
            if let Some(out_bus) = outputs.first_mut() {
                let num_samples = data.num_samples as usize;
                for ch in 0..out_bus.channel_buffers32.len() {
                    if num_samples <= out_bus.channel_buffers32[ch].len() {
                        out_bus.channel_buffers32[ch][..num_samples].fill(0.0);
                    }
                }
            }
        }

        Ok(())
    }

    fn can_process_sample_size(&self, symbolic_sample_size: SymbolicSampleSizes) -> Result<(), Error> {
        if symbolic_sample_size == SymbolicSampleSizes::Sample32 {
            Ok(())
        } else {
            Err(Error::InvalidArgument)
        }
    }

    fn get_bus_count(&self, bus_type: AudioBusType, _is_input: bool) -> u32 {
        match bus_type {
            AudioBusType::Audio => 1, // Stereo in/out
            _ => 0,
        }
    }

    fn get_bus_info(&self, bus_type: AudioBusType, is_input: bool, index: u32) -> Option<AudioBus> {
        if index == 0 && bus_type == AudioBusType::Audio {
            Some(AudioBus {
                name: if is_input { "Audio Input".to_string() } else { "Audio Output".to_string() },
                bus_type,
                flags: vst3::processor::BusFlags::DEFAULT_ACTIVE,
                channel_count: 2, // Stereo
                arrangement: SpeakerArrangement::Stereo,
            })
        } else {
            None
        }
    }

    fn set_process_mode(&mut self, _mode: ProcessMode) -> Result<(), Error> {
        Ok(())
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    fn get_tail_size(&self) -> u32 {
        0 // No tail for silence output
    }
}