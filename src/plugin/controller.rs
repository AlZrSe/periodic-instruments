//! VST3 Plugin Controller implementation

use vst3_sys::vst::{
    IEditController, IEditController2, IComponent, IPlugView,
    ViewRect, ViewType, kResultTrue, kResultFalse, kNotImplemented,
    ParameterInfo, ParameterFlags,
};
use std::ffi::c_void;

/// Minimal editor view
pub struct PeriodicInstrumentsEditor {
    rect: ViewRect,
}

impl PeriodicInstrumentsEditor {
    pub fn new() -> Self {
        Self {
            rect: ViewRect { left: 0, top: 0, right: 1200, bottom: 800 },
        }
    }
}

impl vst3_sys::vst::IPlugView for PeriodicInstrumentsEditor {
    fn get_rect(&self, _rect: *mut ViewRect) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
    
    fn set_rect(&self, _rect: *const ViewRect) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
    
    fn is_platform_type_supported(&self, _type: ViewType) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn attached(&self, _parent: *mut c_void, _type: ViewType) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn removed(&self) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn on_wheel(&self, _distance: f32) -> i32 {
        vst3_sys::vst::kResultTrue
    }
}

/// Minimal controller implementation
pub struct PeriodicInstrumentsController {
    ref_count: std::sync::atomic::AtomicI32,
}

impl PeriodicInstrumentsController {
    pub fn new() -> Self {
        Self {
            ref_count: std::sync::atomic::AtomicI32::new(1),
        }
    }
}

impl vst3_sys::base::FUnknown for PeriodicInstrumentsController {
    fn query_interface(&self, iid: *const vst3_sys::base::TUID, obj: *mut *mut c_void) -> i32 {
        if iid.is_null() || obj.is_null() {
            return vst3_sys::vst::kInvalidArgument;
        }
        
        unsafe {
            let iid = &*iid;
            // Support IEditController, IEditController2, IComponent, FUnknown
            if *iid == vst3_sys::vst::IEditController::IID
                || *iid == vst3_sys::vst::IEditController2::IID
                || *iid == vst3_sys::vst::IComponent::IID
                || *iid == vst3_sys::base::FUnknown::IID {
                *obj = self as *const _ as *mut c_void;
                self.add_ref();
                return vst3_sys::vst::kResultTrue;
            }
        }
        vst3_sys::vst::kNoInterface
    }
    
    fn add_ref(&self) -> u32 {
        self.ref_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
    }
    
    fn release(&self) -> u32 {
        let count = self.ref_count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed) - 1;
        if count == 0 {
            // In a real implementation, we'd deallocate here
            // For minimal implementation, we leak intentionally
        }
        count as u32
    }
}

impl vst3_sys::vst::IComponent for PeriodicInstrumentsController {
    fn initialize(&self, _context: *mut vst3_sys::vst::IHostApplication) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn terminate(&self) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn get_controller_class_id(&self, class_id: *mut vst3_sys::base::TUID) -> i32 {
        if class_id.is_null() {
            return vst3_sys::vst::kInvalidArgument;
        }
        unsafe {
            *class_id = vst3_sys::base::TUID::from_bytes([
                0x70, 0x65, 0x72, 0x69, 0x6F, 0x64, 0x69, 0x63,
                0x49, 0x6E, 0x73, 0x74, 0x72, 0x43, 0x00, 0x00,
            ]);
        }
        vst3_sys::vst::kResultTrue
    }
    
    fn set_io_mode(&self, _mode: i32) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
    
    fn get_bus_count(&self, _bus_type: vst3_sys::vst::BusType, _is_input: i32) -> i32 {
        0
    }
    
    fn get_bus_info(&self, _bus_type: vst3_sys::vst::BusType, _is_input: i32, _index: i32, _info: *mut vst3_sys::vst::BusInfo) -> i32 {
        vst3_sys::vst::kResultFalse
    }
    
    fn get_routing_info(&self, _in_info: *mut vst3_sys::vst::RoutingInfo, _out_info: *mut vst3_sys::vst::RoutingInfo) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
    
    fn activate_bus(&self, _bus_type: vst3_sys::vst::BusType, _is_input: i32, _index: i32, _state: i32) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn set_active(&self, _state: i32) -> i32 {
        vst3_sys::vst::kResultTrue
    }
}

impl vst3_sys::vst::IEditController for PeriodicInstrumentsController {
    fn set_component_state(&self, _state: *mut vst3_sys::base::IBStream) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn set_state(&self, _state: *mut vst3_sys::base::IBStream) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn get_state(&self, _state: *mut vst3_sys::base::IBStream) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn get_parameter_count(&self) -> i32 {
        5
    }
    
    fn get_parameter_info(&self, index: i32, info: *mut vst3_sys::vst::ParameterInfo) -> i32 {
        if info.is_null() || index < 0 || index >= 5 {
            return vst3_sys::vst::kInvalidArgument;
        }
        
        unsafe {
            let (title, short_title, units, step_count, flags) = match index {
                0 => ("Element Selector", "Element", "", 94, ParameterFlags::kCanAutomate as i32 | ParameterFlags::kIsReadOnly as i32),
                1 => ("Mass", "Mass", "", 0, ParameterFlags::kCanAutomate as i32),
                2 => ("Reactivity", "React", "", 0, ParameterFlags::kCanAutomate as i32),
                3 => ("State", "State", "", 0, ParameterFlags::kCanAutomate as i32),
                4 => ("Radiance", "Rad", "", 0, ParameterFlags::kCanAutomate as i32),
                _ => return vst3_sys::vst::kInvalidArgument,
            };
            
            (*info).id = index as i32;
            (*info).step_count = step_count;
            (*info).default_normalized_value = if index == 0 { 0.0 } else { 0.5 };
            (*info).unit_id = 0;
            (*info).flags = flags;
            (*info).type_ = vst3_sys::vst::kParamTypeUnknown as i32;
            
            // Copy strings
            let title_bytes = title.as_bytes();
            for (i, &c) in title_bytes.iter().take(63).enumerate() {
                (*info).title[i] = c as i8;
            }
            (*info).title[title_bytes.len().min(63)] = 0;
            
            let short_bytes = short_title.as_bytes();
            for (i, &c) in short_bytes.iter().take(63).enumerate() {
                (*info).short_title[i] = c as i8;
            }
            (*info).short_title[short_bytes.len().min(63)] = 0;
            
            let units_bytes = units.as_bytes();
            for (i, &c) in units_bytes.iter().take(63).enumerate() {
                (*info).units[i] = c as i8;
            }
            (*info).units[units_bytes.len().min(63)] = 0;
        }
        
        vst3_sys::vst::kResultTrue
    }
    
    fn get_parameter_normalized(&self, index: i32) -> f64 {
        match index {
            0 => 0.0, // Element Selector default (Hydrogen = 1)
            1..=4 => 0.5, // Macro knobs default
            _ => 0.0,
        }
    }
    
    fn set_parameter_normalized(&self, _index: i32, _normalized: f64) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn set_component_handler(&self, _handler: *mut vst3_sys::vst::IComponentHandler) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn create_view(&self, _name: *const i8) -> *mut vst3_sys::vst::IPlugView {
        let editor = Box::new(PeriodicInstrumentsEditor::new());
        Box::into_raw(editor) as *mut vst3_sys::vst::IPlugView
    }
}

impl vst3_sys::vst::IEditController2 for PeriodicInstrumentsController {
    fn get_midi_controller_assignment(
        &self,
        _bus_index: i32,
        _channel: i16,
        _midi_cc: i16,
        _param_id: *mut vst3_sys::vst::ParamID,
    ) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
}

/// Minimal editor view
pub struct PeriodicInstrumentsEditor;

impl PeriodicInstrumentsEditor {
    pub fn new() -> Self {
        Self
    }
}

impl vst3_sys::vst::IPlugView for PeriodicInstrumentsEditor {
    fn get_rect(&self, rect: *mut vst3_sys::vst::ViewRect) -> i32 {
        if rect.is_null() {
            return vst3_sys::vst::kInvalidArgument;
        }
        unsafe {
            *rect = vst3_sys::vst::ViewRect { left: 0, top: 0, right: 1200, bottom: 800 };
        }
        vst3_sys::vst::kResultTrue
    }
    
    fn set_rect(&self, _rect: *const vst3_sys::vst::ViewRect) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn is_platform_type_supported(&self, _type: vst3_sys::vst::ViewType) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn attached(&self, _parent: *mut c_void, _type: vst3_sys::vst::ViewType) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn removed(&self) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn on_wheel(&self, _distance: f32) -> i32 {
        vst3_sys::vst::kResultTrue
    }
}

// Re-export from vst3-sys
use vst3_sys::vst::{
    IEditController, IEditController2, IComponent, IPlugView,
    ViewRect, ViewType, ParameterInfo, ParameterFlags,
    kResultTrue, kResultFalse, kNotImplemented, kInvalidArgument,
};
use std::ffi::c_void;