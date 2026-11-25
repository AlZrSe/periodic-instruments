//! Audio processor implementation for Periodic Instruments VST3 plugin

use vst3_sys::vst::{
    IAudioProcessor, ProcessData, ProcessSetup, AudioBusBuffers, AudioBusType,
    SpeakerArrangement, SymbolicSampleSizes, ProcessMode, kResultTrue, kResultFalse,
    kInvalidArgument,
};
use std::sync::atomic::{AtomicU32, AtomicF32, Ordering};

/// Maximum block size supported
const MAX_BLOCK_SIZE: usize = 4096;

/// Number of parameters
pub const PARAMETER_COUNT: usize = 5;

/// Atomic parameter storage for lock-free audio thread access
pub struct ParameterValues {
    pub element_selector: AtomicU32,
    pub mass: AtomicF32,
    pub reactivity: AtomicF32,
    pub state: AtomicF32,
    pub radiance: AtomicF32,
}

impl ParameterValues {
    pub fn new() -> Self {
        Self {
            element_selector: std::sync::atomic::AtomicU32::new(1), // Hydrogen
            mass: AtomicF32::new(0.5),
            reactivity: AtomicF32::new(0.5),
            state: AtomicF32::new(0.5),
            radiance: AtomicF32::new(0.5),
        }
    }
}

/// Periodic Instruments audio processor - outputs silence (foundation build)
pub struct PeriodicInstrumentsProcessor {
    param_values: std::sync::Arc<ParameterValues>,
    sample_rate: f32,
    max_block_size: usize,
    bypass: bool,
    silence_buffer: Vec<f32>,
}

impl PeriodicInstrumentsProcessor {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            param_values: std::sync::Arc::new(ParameterValues::new()),
            sample_rate,
            max_block_size: 4096,
            bypass: false,
            silence_buffer: vec![0.0f32; 4096],
        }
    }

    pub fn get_param_values(&self) -> std::sync::Arc<ParameterValues> {
        self.param_values.clone()
    }
}

impl vst3_sys::vst::IAudioProcessor for PeriodicInstrumentsProcessor {
    fn set_bus_arrangements(
        &self,
        _inputs: *const vst3_sys::vst::SpeakerArrangement,
        _num_ins: i32,
        _outputs: *const vst3_sys::vst::SpeakerArrangement,
        _num_outs: i32,
    ) -> i32 {
        vst3_sys::vst::kResultTrue
    }

    fn get_bus_arrangement(
        &self,
        _bus_type: vst3_sys::vst::BusType,
        _bus_index: i32,
        _arrangement: *mut vst3_sys::vst::SpeakerArrangement,
    ) -> i32 {
        vst3_sys::vst::kResultFalse
    }

    fn can_process_sample_size(&self, symbolic_sample_size: i32) -> i32 {
        if symbolic_sample_size == vst3_sys::vst::SymbolicSampleSizes::Sample32 as i32 {
            vst3_sys::vst::kResultTrue
        } else {
            vst3_sys::vst::kInvalidArgument
        }
    }

    fn setup_processing(&self, setup: *mut ProcessSetup) -> i32 {
        if setup.is_null() {
            return vst3_sys::vst::kInvalidArgument;
        }
        
        unsafe {
            let setup = &mut *setup;
            if setup.max_samples_per_block > 4096 {
                return vst3_sys::vst::kInvalidArgument;
            }
            if setup.symbolic_sample_size != vst3_sys::vst::SymbolicSampleSizes::Sample32 as i32 {
                return vst3_sys::vst::kInvalidArgument;
            }
            // In a real implementation, we'd store sample rate and block size
        }
        vst3_sys::vst::kResultTrue
    }

    fn set_active(&self, _state: i32) -> i32 {
        vst3_sys::vst::kResultTrue
    }

    fn process(&self, data: *mut ProcessData) -> i32 {
        if data.is_null() {
            return vst3_sys::vst::kInvalidArgument;
        }
        
        unsafe {
            let data = &mut *data;
            
            // Handle bypass - pass through
            if data.process_mode & vst3_sys::vst::ProcessMode::kBypass as i32 != 0 {
                // Pass-through: copy inputs to outputs
                if let (Some(inputs), Some(outputs)) = (data.inputs.as_mut(), data.outputs.as_mut()) {
                    for (in_bus, out_bus) in inputs.iter_mut().zip(outputs.iter_mut()) {
                        if let (Some(in_audio), Some(out_audio)) = (in_bus.as_audio(), out_bus.as_audio_mut()) {
                            let num_samples = data.num_samples as usize;
                            for ch in 0..in_audio.num_channels.min(out_audio.num_channels) {
                                if num_samples <= in_audio.channel_buffers32[ch].len() && 
                                   num_samples <= out_audio.channel_buffers32[ch].len() {
                                    out_audio.channel_buffers32[ch][..num_samples]
                                        .copy_from_slice(&in_audio.channel_buffers32[ch][..num_samples]);
                                }
                            }
                        }
                    }
                }
                return vst3_sys::vst::kResultTrue;
            }
            
            // Instrument mode: output silence (foundation build)
            if let Some(outputs) = data.outputs.as_mut() {
                for out_bus in outputs.iter_mut() {
                    if let Some(out_audio) = out_bus.as_audio_mut() {
                        let num_samples = data.num_samples as usize;
                        for ch in 0..out_audio.num_channels {
                            if num_samples <= out_audio.channel_buffers32[ch].len() {
                                out_audio.channel_buffers32[ch][..num_samples].fill(0.0);
                            }
                        }
                    }
                }
            }
        }
        
        vst3_sys::vst::kResultTrue
    }

    fn get_tail_size(&self) -> i32 {
        0 // No tail for silence output
    }
}

// Re-export from vst3-sys
use vst3_sys::vst::{
    IAudioProcessor, ProcessData, ProcessSetup, AudioBusBuffers, AudioBusType,
    SpeakerArrangement, SymbolicSampleSizes, ProcessMode,
    kResultTrue, kResultFalse, kInvalidArgument,
};