//! Parameter definitions and handling for Periodic Instruments VST3 plugin

use std::sync::atomic::{AtomicU32, Ordering};
use atomic_float::AtomicF32;
use vst3::parameter::{Parameter, ParameterFlags, ParameterInfo as Vst3ParameterInfo};

/// Number of parameters: Element Selector (1-94) + 4 Macro knobs
pub const PARAMETER_COUNT: usize = 5;

/// Parameter IDs (matching VST3 parameter indices)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ParameterId {
    ElementSelector = 0,
    Mass = 1,
    Reactivity = 2,
    State = 3,
    Radiance = 4,
}

impl ParameterId {
    pub fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Self::ElementSelector),
            1 => Some(Self::Mass),
            2 => Some(Self::Reactivity),
            3 => Some(Self::State),
            4 => Some(Self::Radiance),
            _ => None,
        }
    }
}

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
            element_selector: AtomicU32::new(1), // Hydrogen
            mass: AtomicF32::new(0.5),
            reactivity: AtomicF32::new(0.5),
            state: AtomicF32::new(0.5),
            radiance: AtomicF32::new(0.5),
        }
    }

    pub fn set_parameter(&self, index: u32, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        match ParameterId::from_index(index) {
            Some(ParameterId::ElementSelector) => {
                // Map 0.0-1.0 to 1-94
                let element = ((clamped * 93.0) + 1.0) as u32;
                self.element_selector.store(element.min(94).max(1), Ordering::Relaxed);
            }
            Some(ParameterId::Mass) => self.mass.store(clamped, Ordering::Relaxed),
            Some(ParameterId::Reactivity) => self.reactivity.store(clamped, Ordering::Relaxed),
            Some(ParameterId::State) => self.state.store(clamped, Ordering::Relaxed),
            Some(ParameterId::Radiance) => self.radiance.store(clamped, Ordering::Relaxed),
            None => {}
        }
    }

    pub fn get_parameter(&self, index: u32) -> f32 {
        match ParameterId::from_index(index) {
            Some(ParameterId::ElementSelector) => {
                let val = self.element_selector.load(Ordering::Relaxed);
                (val as f32 - 1.0) / 93.0
            }
            Some(ParameterId::Mass) => self.mass.load(Ordering::Relaxed),
            Some(ParameterId::Reactivity) => self.reactivity.load(Ordering::Relaxed),
            Some(ParameterId::State) => self.state.load(Ordering::Relaxed),
            Some(ParameterId::Radiance) => self.radiance.load(Ordering::Relaxed),
            None => 0.0,
        }
    }
}

impl Default for ParameterValues {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameter information for VST3 host
pub fn parameter_info(index: u32) -> Option<Vst3ParameterInfo> {
    let (name, min, max, default, step_count, flags) = match index {
        0 => (
            "Element Selector",
            1.0,
            94.0,
            1.0,
            Some(94),
            ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
        ),
        1 => (
            "Mass",
            0.0,
            1.0,
            0.5,
            None,
            ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
        ),
        2 => (
            "Reactivity",
            0.0,
            1.0,
            0.5,
            None,
            ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
        ),
        3 => (
            "State",
            0.0,
            1.0,
            0.5,
            None,
            ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
        ),
        4 => (
            "Radiance",
            0.0,
            1.0,
            0.5,
            None,
            ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP,
        ),
        _ => return None,
    };

    Some(Vst3ParameterInfo {
        id: index,
        title: name.to_string(),
        short_title: name.to_string(),
        units: "".to_string(),
        step_count,
        default_normalized_value: (default - min) / (max - min),
        unit_id: 0,
        flags,
    })
}

/// VST3 Parameter implementations for each parameter
pub struct ElementSelectorParameter {
    values: std::sync::Arc<ParameterValues>,
}

impl Parameter for ElementSelectorParameter {
    fn get_id(&self) -> u32 { 0 }
    fn get_title(&self) -> String { "Element Selector".to_string() }
    fn get_short_title(&self) -> String { "Element".to_string() }
    fn get_units(&self) -> String { "".to_string() }
    fn get_step_count(&self) -> u32 { 94 }
    fn get_default_normalized_value(&self) -> f64 { 0.0 }
    fn get_unit_id(&self) -> u32 { 0 }
    fn get_flags(&self) -> ParameterFlags { ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP }
    fn to_normalized(&self, plain_value: f64) -> f64 { (plain_value - 1.0) / 93.0 }
    fn to_plain(&self, normalized_value: f64) -> f64 { normalized_value * 93.0 + 1.0 }
    fn get_normalized(&self) -> f64 { 
        let val = self.values.element_selector.load(Ordering::Relaxed);
        ((val as f64) - 1.0) / 93.0 
    }
    fn set_normalized(&self, normalized: f64) {
        let element = ((normalized * 93.0) + 1.0) as u32;
        self.values.element_selector.store(element.min(94).max(1), Ordering::Relaxed);
    }
}

pub struct MassParameter {
    values: std::sync::Arc<ParameterValues>,
}

impl Parameter for MassParameter {
    fn get_id(&self) -> u32 { 1 }
    fn get_title(&self) -> String { "Mass".to_string() }
    fn get_short_title(&self) -> String { "Mass".to_string() }
    fn get_units(&self) -> String { "".to_string() }
    fn get_step_count(&self) -> u32 { 0 }
    fn get_default_normalized_value(&self) -> f64 { 0.5 }
    fn get_unit_id(&self) -> u32 { 0 }
    fn get_flags(&self) -> ParameterFlags { ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP }
    fn to_normalized(&self, plain_value: f64) -> f64 { plain_value }
    fn to_plain(&self, normalized_value: f64) -> f64 { normalized_value }
    fn get_normalized(&self) -> f64 { self.values.mass.load(Ordering::Relaxed) as f64 }
    fn set_normalized(&self, normalized: f64) {
        self.values.mass.store(normalized as f32, Ordering::Relaxed);
    }
}

pub struct ReactivityParameter {
    values: std::sync::Arc<ParameterValues>,
}

impl Parameter for ReactivityParameter {
    fn get_id(&self) -> u32 { 2 }
    fn get_title(&self) -> String { "Reactivity".to_string() }
    fn get_short_title(&self) -> String { "React".to_string() }
    fn get_units(&self) -> String { "".to_string() }
    fn get_step_count(&self) -> u32 { 0 }
    fn get_default_normalized_value(&self) -> f64 { 0.5 }
    fn get_unit_id(&self) -> u32 { 0 }
    fn get_flags(&self) -> ParameterFlags { ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP }
    fn to_normalized(&self, plain_value: f64) -> f64 { plain_value }
    fn to_plain(&self, normalized_value: f64) -> f64 { normalized_value }
    fn get_normalized(&self) -> f64 { self.values.reactivity.load(Ordering::Relaxed) as f64 }
    fn set_normalized(&self, normalized: f64) {
        self.values.reactivity.store(normalized as f32, Ordering::Relaxed);
    }
}

pub struct StateParameter {
    values: std::sync::Arc<ParameterValues>,
}

impl Parameter for StateParameter {
    fn get_id(&self) -> u32 { 3 }
    fn get_title(&self) -> String { "State".to_string() }
    fn get_short_title(&self) -> String { "State".to_string() }
    fn get_units(&self) -> String { "".to_string() }
    fn get_step_count(&self) -> u32 { 0 }
    fn get_default_normalized_value(&self) -> f64 { 0.5 }
    fn get_unit_id(&self) -> u32 { 0 }
    fn get_flags(&self) -> ParameterFlags { ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP }
    fn to_normalized(&self, plain_value: f64) -> f64 { plain_value }
    fn to_plain(&self, normalized_value: f64) -> f64 { normalized_value }
    fn get_normalized(&self) -> f64 { self.values.state.load(Ordering::Relaxed) as f64 }
    fn set_normalized(&self, normalized: f64) {
        self.values.state.store(normalized as f32, Ordering::Relaxed);
    }
}

pub struct RadianceParameter {
    values: std::sync::Arc<ParameterValues>,
}

impl Parameter for RadianceParameter {
    fn get_id(&self) -> u32 { 4 }
    fn get_title(&self) -> String { "Radiance".to_string() }
    fn get_short_title(&self) -> String { "Rad".to_string() }
    fn get_units(&self) -> String { "".to_string() }
    fn get_step_count(&self) -> u32 { 0 }
    fn get_default_normalized_value(&self) -> f64 { 0.5 }
    fn get_unit_id(&self) -> u32 { 0 }
    fn get_flags(&self) -> ParameterFlags { ParameterFlags::AUTOMATABLE | ParameterFlags::CAN_RAMP }
    fn to_normalized(&self, plain_value: f64) -> f64 { plain_value }
    fn to_plain(&self, normalized_value: f64) -> f64 { normalized_value }
    fn get_normalized(&self) -> f64 { self.values.radiance.load(Ordering::Relaxed) as f64 }
    fn set_normalized(&self, normalized: f64) {
        self.values.radiance.store(normalized as f32, Ordering::Relaxed);
    }
}

/// Create all parameter implementations
pub fn create_parameters(values: std::sync::Arc<ParameterValues>) -> Vec<Box<dyn Parameter>> {
    vec![
        Box::new(ElementSelectorParameter { values: values.clone() }),
        Box::new(MassParameter { values: values.clone() }),
        Box::new(ReactivityParameter { values: values.clone() }),
        Box::new(StateParameter { values: values.clone() }),
        Box::new(RadianceParameter { values }),
    ]
}