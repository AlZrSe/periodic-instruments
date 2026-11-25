//! Parameter definitions and handling for Periodic Instruments VST3 plugin

use nih_plug::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicF32, Ordering};

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

/// Create nih-plug parameter objects for each parameter
pub fn create_parameters() -> Vec<Arc<dyn PluginParameter>> {
    use nih_plug::params::{FloatParam, IntParam, ParamFlags};
    use std::sync::Arc;

    let element_selector = Arc::new(IntParam::new(
        "Element Selector",
        1,
        IntRange::Linear { min: 1, max: 94 },
    ));

    let mass = Arc::new(FloatParam::new(
        "Mass",
        0.5,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    ).with_flags(ParamFlags::AUTOMATABLE | ParamFlags::CAN_RAMP));

    let reactivity = Arc::new(FloatParam::new(
        "Reactivity",
        0.5,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    ).with_flags(ParamFlags::AUTOMATABLE | ParamFlags::CAN_RAMP));

    let state = Arc::new(FloatParam::new(
        "State",
        0.5,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    ).with_flags(ParamFlags::AUTOMATABLE | ParamFlags::CAN_RAMP));

    let radiance = Arc::new(FloatParam::new(
        "Radiance",
        0.5,
        FloatRange::Linear { min: 0.0, max: 1.0 },
    ).with_flags(ParamFlags::AUTOMATABLE | ParamFlags::CAN_RAMP));

    vec![element_selector, mass, reactivity, state, radiance]
}

/// Legacy parameter info for compatibility
pub fn parameter_info(index: u32) -> Option<ParameterInfoLegacy> {
    let (id, name, min, max, default, step_count) = match index {
        0 => (0, "Element Selector", 1.0, 94.0, 1.0, Some(94)),
        1 => (1, "Mass", 0.0, 1.0, 0.5, None),
        2 => (2, "Reactivity", 0.0, 1.0, 0.5, None),
        3 => (3, "State", 0.0, 1.0, 0.5, None),
        4 => (4, "Radiance", 0.0, 1.0, 0.5, None),
        _ => return None,
    };

    Some(ParameterInfoLegacy { id, name: name.to_string(), min, max, default, step_count })
}

/// Legacy parameter info struct for compatibility with existing code
#[derive(Debug, Clone)]
pub struct ParameterInfoLegacy {
    pub id: u32,
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub step_count: Option<u32>,
}