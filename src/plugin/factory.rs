//! VST3 Plugin Factory implementation using vst3-sys

use vst3_sys::{
    vst::{
        IPluginFactory, PluginFactoryInfo, ClassInfo, ClassCategory, TUID,
        kResultTrue, kResultFalse,
    },
    base::FUnknown,
};
use std::ffi::c_void;

// Plugin class ID: "PeriodicInstrum" (16 bytes)
const PLUGIN_UID: TUID = TUID::from_bytes([
    0x70, 0x65, 0x72, 0x69, 0x6F, 0x64, 0x69, 0x63,
    0x49, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x6D, 0x00,
]);

/// Factory entry point - required by VST3 hosts
#[no_mangle]
pub extern "C" fn GetPluginFactory() -> *mut dyn vst3_sys::vst::IPluginFactory {
    let factory = Box::new(PluginFactoryImpl);
    Box::into_raw(factory) as *mut dyn vst3_sys::vst::IPluginFactory
}

/// Internal factory implementation
struct PluginFactoryImpl;

impl vst3_sys::vst::IPluginFactory for PluginFactoryImpl {
    fn get_factory_info(&self, info: *mut vst3_sys::vst::PluginFactoryInfo) -> i32 {
        if info.is_null() {
            return vst3_sys::vst::kResultFalse;
        }
        
        unsafe {
            (*info).vendor = b"AlZrSe\0".as_ptr() as *const _;
            (*info).url = b"https://github.com/AlZrSe/periodic-instruments\0".as_ptr() as *const _;
            (*info).email = b"\0".as_ptr() as *const _;
            (*info).flags = 0;
        }
        vst3_sys::vst::kResultTrue
    }

    fn count_classes(&self, _category: *const i8) -> i32 {
        2 // Plugin + Controller
    }

    fn get_class_info(&self, index: i32, info: *mut vst3_sys::vst::ClassInfo) -> i32 {
        if info.is_null() {
            return vst3_sys::vst::kResultFalse;
        }
        
        unsafe {
            if index == 0 {
                // Plugin class
                (*info).class_id = PLUGIN_UID;
                (*info).category = vst3_sys::vst::ClassCategory::AudioEffect as *const _;
                (*info).name = b"Periodic Instruments\0".as_ptr() as *const _;
                (*info).vendor = b"AlZrSe\0".as_ptr() as *const _;
                (*info).version = b"0.1.0\0".as_ptr() as *const _;
                (*info).sdk_version = b"VST 3.7.0\0".as_ptr() as *const _;
                (*info).flags = 0;
                (*info).sub_categories = b"Instrument|Synth\0".as_ptr() as *const _;
            } else if index == 1 {
                // Controller class
                let controller_uid = TUID::from_bytes([
                    0x70, 0x65, 0x72, 0x69, 0x6F, 0x64, 0x69, 0x63,
                    0x49, 0x6E, 0x73, 0x74, 0x72, 0x43, 0x00, 0x00,
                ]);
                (*info).class_id = controller_uid;
                (*info).category = vst3_sys::vst::ClassCategory::ComponentController as *const _;
                (*info).name = b"Periodic Instruments Controller\0".as_ptr() as *const _;
                (*info).vendor = b"AlZrSe\0".as_ptr() as *const _;
                (*info).version = b"0.1.0\0".as_ptr() as *const _;
                (*info).sdk_version = b"VST 3.7.0\0".as_ptr() as *const _;
                (*info).flags = 0;
                (*info).sub_categories = std::ptr::null();
            } else {
                return vst3_sys::vst::kResultFalse;
            }
        }
        vst3_sys::vst::kResultTrue
    }

    fn create_instance(&self, class_id: TUID, _context: *mut c_void) -> *mut c_void {
        // Return a minimal controller instance
        if class_id == PLUGIN_UID {
            let controller = Box::new(MinimalController::new());
            Box::into_raw(controller) as *mut c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

/// Minimal controller implementation
struct MinimalController {
    ref_count: std::sync::atomic::AtomicI32,
}

impl MinimalController {
    fn new() -> Self {
        Self {
            ref_count: std::sync::atomic::AtomicI32::new(1),
        }
    }
}

impl vst3_sys::base::FUnknown for MinimalController {
    fn query_interface(&self, _iid: *const vst3_sys::base::TUID, _obj: *mut *mut c_void) -> i32 {
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

impl vst3_sys::vst::IComponent for MinimalController {
    fn initialize(&self, _context: *mut vst3_sys::vst::IHostApplication) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn terminate(&self) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn get_controller_class_id(&self, _class_id: *mut vst3_sys::base::TUID) -> i32 {
        vst3_sys::vst::kResultTrue
    }
    
    fn set_io_mode(&self, _mode: i32) -> i32 {
        vst3_sys::vst::kNotImplemented
    }
    
    fn get_bus_count(&self, _bus_type: vst3_sys::vst::BusType, _is_input: i32) -> i32 {
        if _bus_type == vst3_sys::vst::BusType::kAudio as i32 {
            1
        } else {
            0
        }
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

// Re-export from vst3-sys
use vst3_sys::vst::{
    IPluginFactory, PluginFactoryInfo, ClassInfo, ClassCategory,
    kResultTrue, kResultFalse, kNoInterface, kNotImplemented,
    IComponent, BusType, BusInfo, RoutingInfo,
};
use std::ffi::c_void;