//! Parameter definitions and handling for Periodic Instruments VST3 plugin

use vst3_sys::vst::kParamTypeUnknown;
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

/// Parameter information for VST3 host (legacy compatibility)
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

/// VST3 parameter information structure for host communication
#[repr(C)]
pub struct VstParameterInfo {
    pub id: i32,
    pub title: [i8; 64],
    pub short_title: [i8; 64],
    pub units: [i8; 64],
    pub step_count: i32,
    pub default_normalized_value: f64,
    pub unit_id: i32,
    pub flags: i32,
    pub type_: i32,
}

impl VstParameterInfo {
    pub fn new(index: u32) -> Self {
        let (title, short_title, units, min, max, default, step_count, flags) = match index {
            0 => (
                "Element Selector", "Element", "",
                1.0, 94.0, 1.0, 94,
                vst3_sys::vst::ParameterInfo::kCanAutomate | vst3_sys::vst::ParameterInfo::kIsReadOnly,
            ),
            1 => (
                "Mass", "Mass", "",
                0.0, 1.0, 0.5, 0,
                vst3_sys::vst::ParameterInfo::kCanAutomate,
            ),
            2 => (
                "Reactivity", "React", "",
                0.0, 1.0, 0.5, 0,
                vst3_sys::vst::ParameterInfo::kCanAutomate,
            ),
            3 => (
                "State", "State", "",
                0.0, 1.0, 0.5, 0,
                vst3_sys::vst::ParameterInfo::kCanAutomate,
            ),
            4 => (
                "Radiance", "Rad", "",
                0.0, 1.0, 0.5, 0,
                vst3_sys::vst::ParameterInfo::kCanAutomate,
            ),
            _ => return Self::empty(),
        };

        let mut info = Self {
            id: index as i32,
            title: [0; 64],
            short_title: [0; 64],
            units: [0; 64],
            step_count,
            default_normalized_value: (default - min) / (max - min),
            unit_id: 0,
            flags,
            type_: kParamTypeUnknown as i32,
        };

        // Copy strings (truncated to 63 chars + null)
        for (i, c) in title.as_bytes().iter().take(63).enumerate() {
            info.title[i] = *c as i8;
        }
        for (i, c) in short_title.as_bytes().iter().take(63).enumerate() {
            info.short_title[i] = *c as i8;
        }
        for (i, c) in units.as_bytes().iter().take(63).enumerate() {
            info.units[i] = *c as i8;
        }

        info
    }

    fn empty() -> Self {
        Self {
            id: -1,
            title: [0; 64],
            short_title: [0; 64],
            units: [0; 64],
            step_count: 0,
            default_normalized_value: 0.0,
            unit_id: 0,
            flags: 0,
            type_: kParamTypeUnknown as i32,
        }
    }
}