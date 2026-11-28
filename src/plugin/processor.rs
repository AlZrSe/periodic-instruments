//! Audio processor — allocation-free process callback
//!
//! Implements the VST3 audio processing callback with:
//! - Silence output (stub implementation)
//! - Allocation-free on audio thread
//! - Variable block size and sample rate support
//! - Bypass handling

use nice_plug::prelude::*;
use crate::plugin::parameters::{ParameterValues, ParameterId, PARAMETER_COUNT};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Maximum supported block size
pub const MAX_BLOCK_SIZE: usize = 4096;

/// Audio processor state
pub struct PeriodicInstrumentsProcessor {
    /// Shared parameter values (accessed from both UI and audio threads)
    parameters: Arc<ParameterValues>,
    /// Current sample rate
    sample_rate: f32,
    /// Current maximum block size
    max_block_size: u32,
    /// Bypass state
    bypassed: AtomicBool,
    /// Processing active flag
    active: AtomicBool,
    /// Silence buffer (pre-allocated, reused)
    silence_buffer: [f32; MAX_BLOCK_SIZE * 2], // stereo
}

impl PeriodicInstrumentsProcessor {
    /// Create new processor instance
    pub fn new(sample_rate: f32, max_block_size: u32) -> Self {
        let max_block = max_block_size as usize;
        assert!(max_block <= MAX_BLOCK_SIZE, "Block size exceeds maximum");
        
        Self {
            parameters: ParameterValues::new(),
            sample_rate,
            max_block_size,
            bypassed: AtomicBool::new(false),
            active: AtomicBool::new(false),
            silence_buffer: [0.0; MAX_BLOCK_SIZE * 2],
        }
    }
    
    /// Get parameter values (for editor/automation)
    pub fn parameters(&self) -> Arc<ParameterValues> {
        Arc::clone(&self.parameters)
    }
    
    /// Clear output buffers to silence
    fn clear_output(&self, outputs: &mut [AudioBuffer<f32>], num_samples: usize) {
        for output in outputs.iter_mut() {
            for channel in output.channels_mut() {
                // Clear only the valid samples
                for sample in channel.iter_mut().take(num_samples) {
                    *sample = 0.0;
                }
            }
        }
    }
}

impl PluginProcessor for PeriodicInstrumentsProcessor {
    fn initialize(&mut self, _context: &mut InitializeContext) -> Result<(), PluginError> {
        self.active.store(true, Ordering::Relaxed);
        tracing::info!("Processor initialized at {} Hz, max block {}", self.sample_rate, self.max_block_size);
        Ok(())
    }
    
    fn setup_processing(&mut self, setup: &ProcessingSetup) -> Result<(), PluginError> {
        self.sample_rate = setup.sample_rate;
        self.max_block_size = setup.max_samples_per_block;
        
        if self.max_block_size as usize > MAX_BLOCK_SIZE {
            return Err(PluginError::BufferTooSmall);
        }
        
        tracing::info!("Processing setup: {} Hz, max block {}", self.sample_rate, self.max_block_size);
        Ok(())
    }
    
    fn set_active(&mut self, active: bool) -> Result<(), PluginError> {
        self.active.store(active, Ordering::Relaxed);
        if active {
            tracing::info!("Processor activated");
        } else {
            tracing::info!("Processor deactivated");
        }
        Ok(())
    }
    
    fn process(&mut self, context: &mut ProcessContext) -> Result<(), PluginError> {
        // Check if processing is active
        if !self.active.load(Ordering::Relaxed) {
            return Ok(());
        }
        
        // TEST-ONLY: Crash injection - abort on 3rd process() call
        #[cfg(feature = "crash-injection")]
        crate::plugin::processor::crash_injection::maybe_crash();
        
        // TEST-ONLY: Hang injection - infinite loop when enabled
        #[cfg(feature = "crash-injection")]
        crate::plugin::processor::crash_injection::maybe_hang();
        
        let num_samples = context.num_samples();
        let num_output_channels = context.outputs().len();
        
        // Get bypass state
        let bypassed = self.bypassed.load(Ordering::Relaxed);
        
        if bypassed {
            // Bypass: pass input to output (for instruments, typically silence)
            // Since this is an instrument with no audio input, output silence
            self.clear_output(context.outputs_mut(), num_samples);
        } else {
            // Normal processing: output silence (stub implementation)
            // Future tasks will add actual synthesis here
            self.clear_output(context.outputs_mut(), num_samples);
        }
        
        // Process parameter changes from host (automation)
        // Note: In a real implementation, we'd read parameter changes from context.parameter_changes()
        // For now, the parameter values are accessed directly via the shared Arc<ParameterValues>
        
        Ok(())
    }
    
    fn set_bypass(&mut self, bypassed: bool) {
        self.bypassed.store(bypassed, Ordering::Relaxed);
    }
    
    fn get_tail_size(&self) -> usize {
        0 // No tail for silence output
    }
    
    fn can_do_midi(&self) -> bool {
        true
    }
    
    fn can_do_automation(&self) -> bool {
        true
    }
}

// ============================================================================
// TEST-ONLY: Crash Injection for Process Isolation Verification
// ============================================================================
// This module is ONLY compiled when the `crash-injection` feature is enabled.
// It provides controlled fault injection points to verify that the host's
// process isolation correctly contains crashes, panics, and hangs in the
// child plugin process without affecting the test runner.
//
// DO NOT enable this feature in production builds.
// ============================================================================

#[cfg(feature = "crash-injection")]
mod crash_injection {
    use std::sync::atomic::{AtomicU32, Ordering};
    
    /// Counter for process() calls - triggers abort on 3rd call (0-indexed = 2)
    static PROCESS_COUNT: AtomicU32 = AtomicU32::new(0);
    
    /// Flag to enable panic injection on parameter set
    static PANIC_ON_PARAM_SET: AtomicU32 = AtomicU32::new(0);
    
    /// Flag to enable infinite loop in process()
    static HANG_IN_PROCESS: AtomicU32 = AtomicU32::new(0);
    
    /// Called from process() - aborts on 3rd invocation
    pub fn maybe_crash() {
        let count = PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
        if count == 2 { // 3rd call (0-indexed: 0, 1, 2)
            std::process::abort();
        }
    }
    
    /// Called from parameter setter - panics when enabled
    pub fn maybe_panic_on_param_set() {
        if PANIC_ON_PARAM_SET.load(Ordering::SeqCst) == 1 {
            panic!("test panic injection: parameter set fault");
        }
    }
    
    /// Called from process() - infinite loop when enabled
    pub fn maybe_hang() {
        if HANG_IN_PROCESS.load(Ordering::SeqCst) == 1 {
            loop { 
                std::hint::spin_loop(); 
            }
        }
    }
    
    /// Enable panic injection for next parameter set call
    pub fn enable_panic_on_param_set() {
        PANIC_ON_PARAM_SET.store(1, Ordering::SeqCst);
    }
    
    /// Enable hang in process loop
    pub fn enable_hang_in_process() {
        HANG_IN_PROCESS.store(1, Ordering::SeqCst);
    }
    
    /// Reset all injection state (for test isolation)
    pub fn reset() {
        PROCESS_COUNT.store(0, Ordering::SeqCst);
        PANIC_ON_PARAM_SET.store(0, Ordering::SeqCst);
        HANG_IN_PROCESS.store(0, Ordering::SeqCst);
    }
}

/// Audio buffer wrapper for nice-plug
#[derive(Debug)]
pub struct AudioBuffer<T> {
    channels: Vec<Vec<T>>,
}

impl<T> AudioBuffer<T> {
    pub fn new(num_channels: usize, capacity: usize) -> Self 
    where 
        T: Default + Clone 
    {
        Self {
            channels: vec![vec![T::default(); capacity]; num_channels],
        }
    }
    
    pub fn channels(&self) -> &[Vec<T>] {
        &self.channels
    }
    
    pub fn channels_mut(&mut self) -> &mut [Vec<T>] {
        &mut self.channels
    }
    
    pub fn channel(&self, index: usize) -> Option<&[T]> {
        self.channels.get(index).map(|v| v.as_slice())
    }
    
    pub fn channel_mut(&mut self, index: usize) -> Option<&mut [T]> {
        self.channels.get_mut(index).map(|v| v.as_mut_slice())
    }
    
    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }
}