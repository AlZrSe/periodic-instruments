//! Periodic Instruments — VST3 Synthesizer Plugin
//!
//! A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)
//! with unique synthesis engines per element, macro knob modulation, and a periodic
//! table-based UI.

#![cfg_attr(not(test), deny(clippy::all))]
#![cfg_attr(not(test), deny(clippy::pedantic))]
#![cfg_attr(not(test), deny(clippy::nursery))]
#![cfg_attr(not(test), deny(clippy::cargo))]
#![cfg_attr(not(test), deny(missing_docs))]
#![cfg_attr(not(test), deny(rustdoc::missing_crate_level_docs))]

// Core modules
pub mod core;
pub mod elements;
pub mod fx;
pub mod ui;
pub mod plugin;

// Re-export main plugin types for VST3 entry point
pub use plugin::{
    PeriodicInstrumentsFactory,
    PeriodicInstrumentsProcessor,
    PeriodicInstrumentsController,
    PeriodicInstrumentsEditor,
    PLUGIN_NAME,
    PLUGIN_VENDOR,
    PLUGIN_VERSION,
    PLUGIN_CATEGORY,
    PLUGIN_UNIQUE_ID,
    PLUGIN_CLASS_ID,
};

// Re-export parameter types
pub use plugin::parameters::{
    ParameterId,
    ParameterValues,
    PARAMETER_COUNT,
    parameter_info,
    create_parameters,
};
pub use vst3::parameter::ParameterInfo as Vst3ParameterInfo;

/// Plugin version as semver string
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the plugin
pub fn init_logging() {
    use tracing_subscriber::{EnvFilter, fmt};
    
    fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("periodic_instruments=debug".parse().unwrap()))
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact()
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parameter_count() {
        assert_eq!(PARAMETER_COUNT, 5);
    }
    
    #[test]
    fn test_parameter_info_element_selector() {
        let info = parameter_info(0).expect("Parameter 0 should exist");
        assert_eq!(info.title, "Element Selector");
        assert_eq!(info.step_count, Some(94));
    }
    
    #[test]
    fn test_parameter_info_macro_knobs() {
        for i in 1..5 {
            let info = parameter_info(i).expect(&format!("Parameter {i} should exist"));
            // Vst3ParameterInfo uses default_normalized_value, not default_value
            assert_eq!(info.default_normalized_value, 0.5);
        }
    }
    
    #[test]
    fn test_parameter_values_default() {
        let params = ParameterValues::new();
        assert_eq!(params.element_selector.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(params.mass.load(std::sync::atomic::Ordering::Relaxed), 0.5);
        assert_eq!(params.reactivity.load(std::sync::atomic::Ordering::Relaxed), 0.5);
        assert_eq!(params.state.load(std::sync::atomic::Ordering::Relaxed), 0.5);
        assert_eq!(params.radiance.load(std::sync::atomic::Ordering::Relaxed), 0.5);
    }
    
    #[test]
    fn test_parameter_values_set_get() {
        let params = ParameterValues::new();
        
        // Test element selector
        params.set_parameter(0, 0.5); // Middle = element ~48
        let val = params.get_parameter(0);
        assert!((val - 0.5).abs() < 0.02);
        
        // Test macro knobs
        for i in 1..5 {
            params.set_parameter(i, 0.75);
            assert!((params.get_parameter(i) - 0.75).abs() < f32::EPSILON);
        }
    }
    
    #[test]
    fn test_parameter_clamping() {
        let params = ParameterValues::new();
        
        // Test out-of-range clamping
        params.set_parameter(1, 1.5); // Should clamp to 1.0
        assert_eq!(params.get_parameter(1), 1.0);
        
        params.set_parameter(1, -0.5); // Should clamp to 0.0
        assert_eq!(params.get_parameter(1), 0.0);
    }
}