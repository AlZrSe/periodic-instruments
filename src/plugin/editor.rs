//! Editor registration via nice-plug/iced
//!
//! Provides the VST3 editor view using nice-plug's iced integration.

use nice_plug::prelude::*;
use nice_plug::IcedEditor;
use crate::ui::{EditorState, EditorMessage, PluginEditor, DEFAULT_WIDTH, DEFAULT_HEIGHT, MIN_WIDTH, MIN_HEIGHT};

/// Plugin editor wrapper for nice-plug
pub struct PeriodicInstrumentsEditor {
    editor: PluginEditor,
}

impl PeriodicInstrumentsEditor {
    pub fn new() -> Self {
        let settings = Settings {
            window: iced::window::Settings {
                size: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
                min_size: Some((MIN_WIDTH, MIN_HEIGHT)),
                max_size: None,
                visible: false,
                resizable: true,
                decorations: true,
                transparent: false,
                always_on_top: false,
                icon: None,
            },
            ..Settings::default()
        };
        
        let editor = IcedEditor::<EditorState, EditorMessage>::with_settings(settings);
        
        Self { editor }
    }
}

impl Default for PeriodicInstrumentsEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginEditor for PeriodicInstrumentsEditor {
    fn is_open(&self) -> bool {
        self.editor.is_open()
    }
    
    fn open(&mut self, parent: *mut std::ffi::c_void) -> Result<(), PluginError> {
        self.editor.open(parent)
    }
    
    fn close(&mut self) -> Result<(), PluginError> {
        self.editor.close()
    }
    
    fn set_bounds(&mut self, bounds: EditorBounds) -> Result<(), PluginError> {
        self.editor.set_bounds(bounds)
    }
    
    fn get_bounds(&self) -> EditorBounds {
        self.editor.get_bounds()
    }
    
    fn idle(&mut self) {
        self.editor.idle()
    }
    
    fn preferred_size(&self) -> (u32, u32) {
        (DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
    
    fn minimum_size(&self) -> (u32, u32) {
        (MIN_WIDTH, MIN_HEIGHT)
    }
    
    fn maximum_size(&self) -> (u32, u32) {
        (1920, 1200)
    }
}