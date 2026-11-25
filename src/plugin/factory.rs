//! VST3 Plugin Factory implementation

use std::sync::Arc;
use vst3::factory::{Factory, ClassInfo, ClassCategory, TUID, FactoryInfo, ClassFlags};
use vst3::error::Error;
use vst3::base::FUnknown;
use crate::plugin::parameters::{create_parameters, ParameterValues};

/// Periodic Instruments VST3 Plugin Factory
pub struct PeriodicInstrumentsFactory {
    plugin_uid: TUID,
}

impl PeriodicInstrumentsFactory {
    pub fn new() -> Self {
        // Plugin class ID: "PeriodicInstrum" (16 bytes)
        let plugin_uid = TUID::from_bytes([
            0x70, 0x65, 0x72, 0x69, 0x6F, 0x64, 0x69, 0x63,
            0x49, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x6D, 0x00,
        ]);
        
        Self { plugin_uid }
    }
}

impl Factory for PeriodicInstrumentsFactory {
    fn get_factory_info(&self) -> FactoryInfo {
        FactoryInfo {
            vendor: "AlZrSe".to_string(),
            url: "https://github.com/AlZrSe/periodic-instruments".to_string(),
            email: "".to_string(),
            flags: vst3::factory::FactoryFlags::empty(),
        }
    }

    fn get_class_count(&self) -> u32 { 1 }

    fn get_class_info(&self, index: u32) -> Option<ClassInfo> {
        if index == 0 {
            Some(ClassInfo {
                class_id: self.plugin_uid,
                category: ClassCategory::AudioEffect,
                name: "Periodic Instruments".to_string(),
                vendor: "AlZrSe".to_string(),
                version: "0.1.0".to_string(),
                sdk_version: "3.7.0".to_string(),
                flags: ClassFlags::empty(),
                sub_categories: "Instrument|Synth".to_string(),
            })
        } else {
            None
        }
    }

    fn create_instance(&self, class_id: TUID, _context: *mut FUnknown) -> Result<*mut FUnknown, Error> {
        if class_id == self.plugin_uid {
            let processor = crate::plugin::processor::PeriodicInstrumentsProcessor::new();
            let controller = crate::plugin::controller::PeriodicInstrumentsController::new(
                Arc::new(processor),
            );
            Ok(Box::into_raw(Box::new(controller)) as *mut FUnknown)
        } else {
            Err(Error::NoInterface)
        }
    }
}

impl Default for PeriodicInstrumentsFactory {
    fn default() -> Self {
        Self::new()
    }
}