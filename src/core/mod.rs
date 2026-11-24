//! DSP core module — voice manager, modulation, parameter mapping
//!
//! This module contains the core DSP infrastructure for the plugin.
//! Currently empty — implementations will be added in subsequent tasks.

/// Voice management for polyphonic synthesis (up to 64 voices)
pub mod voice_manager {
    /// Maximum number of simultaneous voices
    pub const MAX_VOICES: usize = 64;
    
    /// Placeholder for voice manager implementation
    pub struct VoiceManager;
    
    impl VoiceManager {
        pub fn new() -> Self {
            Self
        }
    }
}

/// Modulation system — sources, destinations, routing
pub mod modulation {
    /// Placeholder for modulation system implementation
    pub struct ModulationSystem;
    
    impl ModulationSystem {
        pub fn new() -> Self {
            Self
        }
    }
}

/// Parameter mapping utilities
pub mod parameter_mapping {
    /// Convert normalized parameter value (0.0-1.0) to frequency in Hz
    pub fn normalized_to_frequency(normalized: f32, min_freq: f32, max_freq: f32) -> f32 {
        min_freq * (max_freq / min_freq).powf(normalized)
    }
    
    /// Convert normalized parameter value to decibels
    pub fn normalized_to_db(normalized: f32, min_db: f32, max_db: f32) -> f32 {
        min_db + normalized * (max_db - min_db)
    }
    
    /// Convert normalized parameter value to linear range
    pub fn normalized_to_linear(normalized: f32, min: f32, max: f32) -> f32 {
        min + normalized * (max - min)
    }
}