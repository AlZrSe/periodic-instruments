//! Parameter definitions and management
//!
//! Defines the 5 plugin parameters:
//! 0: Element Selector (1-94, default 1)
//! 1: Mass (0.0-1.0, default 0.5)
//! 2: Reactivity (0.0-1.0, default 0.5)
//! 3: State (0.0-1.0, default 0.5)
//! 4: Radiance (0.0-1.0, default 0.5)

use vst3::prelude::*;
use atomic_float::AtomicF32;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;

/// Total number of parameters
pub const PARAMETER_COUNT: usize = 5;

/// Parameter indices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum ParameterId {
    ElementSelector = 0,
    Mass = 1,
    Reactivity = 2,
    State = 3,
    Radiance = 4,
}

impl ParameterId {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(ParameterId::ElementSelector),
            1 => Some(ParameterId::Mass),
            2 => Some(ParameterId::Reactivity),
            3 => Some(ParameterId::State),
            4 => Some(ParameterId::Radiance),
            _ => None,
        }
    }
    
    pub fn index(&self) -> usize {
        *self as usize
    }
}

/// Parameter information exposed to host
pub fn parameter_info(index: usize) -> Option<ParameterInfo> {
    match ParameterId::from_index(index) {
        Some(ParameterId::ElementSelector) => Some(ParameterInfo {
            id: index as u32,
            name: "Element Selector".to_string(),
            label: "Element".to_string(),
            short_label: "El".to_string(),
            min_value: 1.0,
            max_value: 94.0,
            default_value: 1.0,
            step_count: Some(94), // Discrete integer steps
            flags: ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
            unit: "".to_string(),
        }),
        Some(ParameterId::Mass) => Some(ParameterInfo {
            id: index as u32,
            name: "Mass".to_string(),
            label: "Mass".to_string(),
            short_label: "Mass".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            default_value: 0.5,
            step_count: None, // Continuous
            flags: ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
            unit: "".to_string(),
        }),
        Some(ParameterId::Reactivity) => Some(ParameterInfo {
            id: index as u32,
            name: "Reactivity".to_string(),
            label: "Reactivity".to_string(),
            short_label: "Reac".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            default_value: 0.5,
            step_count: None,
            flags: ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
            unit: "".to_string(),
        }),
        Some(ParameterId::State) => Some(ParameterInfo {
            id: index as u32,
            name: "State".to_string(),
            label: "State".to_string(),
            short_label: "State".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            default_value: 0.5,
            step_count: None,
            flags: ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
            unit: "".to_string(),
        }),
        Some(ParameterId::Radiance) => Some(ParameterInfo {
            id: index as u32,
            name: "Radiance".to_string(),
            label: "Radiance".to_string(),
            short_label: "Rad".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            default_value: 0.5,
            step_count: None,
            flags: ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
            unit: "".to_string(),
        }),
        None => None,
    }
}

/// Thread-safe parameter storage
#[derive(Debug)]
pub struct ParameterValues {
    /// Element selector (1-94) - atomic for thread-safe access
    pub element_selector: AtomicU8,
    /// Macro knobs (0.0-1.0) - atomic floats for lock-free access
    pub mass: AtomicF32,
    pub reactivity: AtomicF32,
    pub state: AtomicF32,
    pub radiance: AtomicF32,
}

impl Default for ParameterValues {
    fn default() -> Self {
        Self {
            element_selector: AtomicU8::new(1), // Hydrogen
            mass: AtomicF32::new(0.5),
            reactivity: AtomicF32::new(0.5),
            state: AtomicF32::new(0.5),
            radiance: AtomicF32::new(0.5),
        }
    }
}

impl ParameterValues {
    /// Create new parameter values with defaults
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    
    /// Get element selector as f32 (normalized 0.0-1.0 for automation)
    pub fn element_selector_normalized(&self) -> f32 {
        (self.element_selector.load(std::sync::atomic::Ordering::Relaxed) as f32 - 1.0) / 93.0
    }
    
    /// Set element selector from normalized value (0.0-1.0)
    pub fn set_element_selector_normalized(&self, normalized: f32) {
        let value = (normalized * 93.0).round().clamp(0.0, 93.0) as u8 + 1;
        self.element_selector.store(value, std::sync::atomic::Ordering::Relaxed);
    }
    
    /// Get all macro values as array
    pub fn macro_values(&self) -> [f32; 4] {
        [
            self.mass.load(std::sync::atomic::Ordering::Relaxed),
            self.reactivity.load(std::sync::atomic::Ordering::Relaxed),
            self.state.load(std::sync::atomic::Ordering::Relaxed),
            self.radiance.load(std::sync::atomic::Ordering::Relaxed),
        ]
    }
    
    /// Set parameter by index (for automation/host)
    pub fn set_parameter(&self, index: usize, value: f32) {
        match ParameterId::from_index(index) {
            Some(ParameterId::ElementSelector) => self.set_element_selector_normalized(value),
            Some(ParameterId::Mass) => self.mass.store(value.clamp(0.0, 1.0), std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::Reactivity) => self.reactivity.store(value.clamp(0.0, 1.0), std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::State) => self.state.store(value.clamp(0.0, 1.0), std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::Radiance) => self.radiance.store(value.clamp(0.0, 1.0), std::sync::atomic::Ordering::Relaxed),
            None => {}
        }
    }
    
    /// Get parameter by index (for automation/host)
    pub fn get_parameter(&self, index: usize) -> f32 {
        match ParameterId::from_index(index) {
            Some(ParameterId::ElementSelector) => self.element_selector_normalized(),
            Some(ParameterId::Mass) => self.mass.load(std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::Reactivity) => self.reactivity.load(std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::State) => self.state.load(std::sync::atomic::Ordering::Relaxed),
            Some(ParameterId::Radiance) => self.radiance.load(std::sync::atomic::Ordering::Relaxed),
            None => 0.0,
        }
    }
}