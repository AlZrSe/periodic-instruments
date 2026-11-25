//! VST3 Plugin Controller implementation

use nih_plug::prelude::*;
use std::sync::Arc;
use crate::plugin::parameters::{create_parameters, ParameterValues};
use crate::plugin::processor::PeriodicInstrumentsProcessor;

/// Editor view for the plugin
pub struct PeriodicInstrumentsEditor {
    rect: Rect,
}

impl PeriodicInstrumentsEditor {
    pub fn new(_controller: Arc<PeriodicInstrumentsController>) -> Self {
        Self {
            rect: Rect::new(0, 0, 1200, 800),
        }
    }
}

impl Editor for PeriodicInstrumentsEditor {
    fn size(&self) -> (u32, u32) {
        (self.rect.width, self.rect.height)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.rect = Rect::new(0, 0, width, height);
    }

    fn is_open(&self) -> bool {
        true
    }

    fn open(&mut self, _parent: *mut std::ffi::c_void) -> Result<(), PluginError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// VST3 Plugin Controller
pub struct PeriodicInstrumentsController {
    processor: Arc<PeriodicInstrumentsProcessor>,
    parameters: Vec<Arc<dyn PluginParameter>>,
    param_values: Arc<ParameterValues>,
}

impl PeriodicInstrumentsController {
    pub fn new(processor: Arc<PeriodicInstrumentsProcessor>) -> Self {
        let param_values = Arc::new(ParameterValues::new());
        let parameters = create_parameters();
        
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

impl Controller for PeriodicInstrumentsController {
    fn get_parameters(&self) -> Vec<Arc<dyn PluginParameter>> {
        self.parameters.clone()
    }

    fn get_parameter(&self, index: usize) -> Option<Arc<dyn PluginParameter>> {
        self.parameters.get(index).cloned()
    }

    fn create_editor(&self) -> Box<dyn Editor> {
        Box::new(PeriodicInstrumentsEditor::new(
            Arc::new(self.clone())
        ))
    }
}

impl Clone for PeriodicInstrumentsController {
    fn clone(&self) -> Self {
        let new_controller = Self::new(self.processor.clone());
        for (i, param) in self.parameters.iter().enumerate() {
            if let Some(new_param) = new_controller.parameters.get(i) {
                new_param.set_normalized_value(param.normalized_value());
            }
        }
        new_controller
    }
}