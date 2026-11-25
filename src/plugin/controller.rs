//! VST3 Plugin Controller implementation

use vst3::{Editor, ViewRect, Error, ViewType};
use std::sync::Arc;
use crate::plugin::parameters::ParameterValues;
use crate::plugin::processor::PeriodicInstrumentsProcessor;

/// Editor view for the plugin
pub struct PeriodicInstrumentsEditor {
    rect: ViewRect,
}

impl PeriodicInstrumentsEditor {
    pub fn new(_controller: Arc<PeriodicInstrumentsController>) -> Self {
        Self {
            rect: ViewRect { left: 0, top: 0, right: 1200, bottom: 800 },
        }
    }
}

impl Editor for PeriodicInstrumentsEditor {
    fn get_rect(&self) -> ViewRect {
        self.rect
    }

    fn set_rect(&mut self, rect: ViewRect) -> Result<(), Error> {
        self.rect = rect;
        Ok(())
    }

    fn is_platform_type_supported(&self, _view_type: ViewType) -> bool {
        true
    }

    fn attached(&mut self, _parent: *mut std::ffi::c_void, _view_type: ViewType) -> Result<(), Error> {
        Ok(())
    }

    fn removed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_wheel(&mut self, _distance: f32) -> Result<(), Error> {
        Ok(())
    }
}

/// VST3 Plugin Controller
pub struct PeriodicInstrumentsController {
    processor: Arc<PeriodicInstrumentsProcessor>,
    parameters: Vec<Box<dyn vst3::Parameter>>,
    param_values: Arc<ParameterValues>,
}

impl PeriodicInstrumentsController {
    pub fn new(processor: Arc<PeriodicInstrumentsProcessor>) -> Self {
        let param_values = Arc::new(ParameterValues::new());
        let parameters = crate::plugin::parameters::create_parameters(param_values.clone());
        
        Self {
            processor,
            parameters,
            param_values,
        }
    }

    pub fn get_processor(&self) -> Arc<PeriodicInstrumentsProcessor> {
        self.processor.clone()
    }

    pub fn get_param_values(&self) -> Arc<ParameterValues> {
        self.param_values.clone()
    }
}

impl vst3::PluginController for PeriodicInstrumentsController {
    fn initialize(&mut self, _context: vst3::InitializeContext) -> Result<(), Error> {
        Ok(())
    }

    fn terminate(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn get_parameter_count(&self) -> u32 {
        self.parameters.len() as u32
    }

    fn get_parameter_info(&self, index: u32) -> Option<vst3::ParameterInfo> {
        self.parameters.get(index as usize).map(|p| vst3::ParameterInfo {
            id: p.get_id(),
            title: p.get_title(),
            short_title: p.get_short_title(),
            units: p.get_units(),
            step_count: p.get_step_count(),
            default_normalized_value: p.get_default_normalized_value(),
            unit_id: p.get_unit_id(),
            flags: p.get_flags(),
        })
    }

    fn get_parameter_normalized(&self, index: u32) -> f64 {
        self.parameters.get(index as usize).map(|p| p.get_normalized()).unwrap_or(0.0)
    }

    fn set_parameter_normalized(&mut self, index: u32, normalized: f64) {
        if let Some(param) = self.parameters.get(index as usize) {
            param.set_normalized(normalized);
        }
    }

    fn get_controller_class_id(&self) -> vst3::TUID {
        vst3::TUID::from_bytes([
            0x70, 0x65, 0x72, 0x69, 0x6F, 0x64, 0x69, 0x63,
            0x49, 0x6E, 0x73, 0x74, 0x72, 0x43, 0x00, 0x00,
        ])
    }

    fn create_editor(&mut self) -> Result<Box<dyn Editor>, Error> {
        Ok(Box::new(PeriodicInstrumentsEditor::new(
            Arc::new(self.clone())
        )))
    }
}

// Need Clone for the controller
impl Clone for PeriodicInstrumentsController {
    fn clone(&self) -> Self {
        // Create new controller with same processor reference
        let new_controller = Self::new(self.processor.clone());
        // Copy current parameter values
        for (i, param) in self.parameters.iter().enumerate() {
            if let Some(new_param) = new_controller.parameters.get(i) {
                new_param.set_normalized(param.get_normalized());
            }
        }
        new_controller
    }
}