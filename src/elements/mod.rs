//! Element definitions and factory module
//!
//! This module contains per-element definitions aligned with `docs/elements.md`.
//! Currently empty — implementations will be added in subsequent tasks.

/// Element factory for creating element-specific synthesis engines
pub mod factory {
    use crate::core::voice_manager::MAX_VOICES;
    
    /// Atomic number range for elements (H=1 through Pu=94)
    pub const MIN_ATOMIC_NUMBER: u8 = 1;
    pub const MAX_ATOMIC_NUMBER: u8 = 94;
    pub const DEFAULT_ATOMIC_NUMBER: u8 = 1; // Hydrogen
    
    /// Element synthesis type enumeration
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ElementSynthesisType {
        /// Subtractive synthesis (most elements)
        Subtractive,
        /// FM synthesis
        FM,
        /// Wavetable synthesis
        Wavetable,
        /// Granular synthesis
        Granular,
        /// Physical modeling
        PhysicalModeling,
        /// Hybrid (multiple engines)
        Hybrid,
    }
    
    /// Element definition containing synthesis parameters
    #[derive(Debug, Clone)]
    pub struct ElementDefinition {
        pub atomic_number: u8,
        pub symbol: &'static str,
        pub name: &'static str,
        pub synthesis_type: ElementSynthesisType,
        pub category: ElementCategory,
    }
    
    /// Periodic table category
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ElementCategory {
        AlkaliMetal,
        AlkalineEarthMetal,
        TransitionMetal,
        PostTransitionMetal,
        Metalloid,
        Nonmetal,
        Halogen,
        NobleGas,
        Lanthanide,
        Actinide,
    }
    
    /// Get element definition by atomic number
    pub fn get_element(atomic_number: u8) -> Option<&'static ElementDefinition> {
        // Placeholder — will be populated from docs/elements.md in later tasks
        if atomic_number >= MIN_ATOMIC_NUMBER && atomic_number <= MAX_ATOMIC_NUMBER {
            Some(&HYDROGEN)
        } else {
            None
        }
    }
    
    /// Hydrogen (element 1) — placeholder definition
    static HYDROGEN: ElementDefinition = ElementDefinition {
        atomic_number: 1,
        symbol: "H",
        name: "Hydrogen",
        synthesis_type: ElementSynthesisType::Subtractive,
        category: ElementCategory::Nonmetal,
    };
}

/// Element parameter mappings — maps macro knobs to DSP parameters per element
pub mod parameter_mapping {
    /// Macro knob indices
    pub const MACRO_MASS: usize = 0;
    pub const MACRO_REACTIVITY: usize = 1;
    pub const MACRO_STATE: usize = 2;
    pub const MACRO_RADIANCE: usize = 3;
    
    /// Maps element macro values to synthesis engine parameters
    pub fn map_macros_to_engine(
        element_atomic_number: u8,
        mass: f32,
        reactivity: f32,
        state: f32,
        radiance: f32,
    ) -> EngineParameters {
        // Placeholder — actual mappings per element will be implemented later
        EngineParameters {
            oscillator_pitch: mass,
            filter_cutoff: reactivity,
            envelope_attack: state,
            fx_mix: radiance,
        }
    }
    
    /// Engine parameters derived from macro knobs
    #[derive(Debug, Clone, Copy)]
    pub struct EngineParameters {
        pub oscillator_pitch: f32,
        pub filter_cutoff: f32,
        pub envelope_attack: f32,
        pub fx_mix: f32,
    }
}