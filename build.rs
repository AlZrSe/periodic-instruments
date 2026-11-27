// Build script for Periodic Instruments VST3 plugin
// Compiles the C++ VST3 plugin using the `cc` crate
// VST3 SDK is included in the repository at vst3sdk/

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/vst3/periodic_instruments.cpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/vst3/entry_windows.cpp");
    println!("cargo:rerun-if-changed=src/vst3/moduleinfo.json");
    println!("cargo:rerun-if-changed=vst3sdk");

    let _target = env::var("TARGET").unwrap();
    let _profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // VST3 SDK is in the repository
    let vst3_sdk = manifest_dir.join("vst3sdk");
    if !vst3_sdk.exists() {
        panic!("VST3 SDK not found at {}. The VST3 SDK must be in the vst3sdk/ directory.", vst3_sdk.display());
    }

    // Build the C++ VST3 plugin using the `cc` crate
    // We ONLY compile our plugin source + minimal platform entry point.
    // We DO NOT compile any SDK .cpp files - the SDK is header-only for us.
    // This avoids redefinition errors from headers with const/inline function definitions.
    let mut build = cc::Build::new();
    build.cpp(true)
         .define("VST_VERSION_STR", Some("\"0.1.0\""))
         .define("NDEBUG", Some("1"))
         .opt_level(3);

    // Platform-specific settings
    #[cfg(target_os = "windows")]
    {
        // On Windows with MSVC
        build.flag("/std:c++17")
             .flag("/EHsc")  // Enable C++ exceptions
             .flag("/W3")    // Warning level 3
             .flag("/Zc:inline") // C++17 inline variables support
             .define("_CRT_SECURE_NO_WARNINGS", None); // Suppress CRT warnings
    }
    #[cfg(not(target_os = "windows"))]
    {
        build.flag("-std=c++17")
             .pic(true)
             .flag("-fvisibility=default");
    }

    // Add VST3 SDK include paths
    build.include(&vst3_sdk);  // For "pluginterfaces/base/fplatform.h" style includes
    build.include(vst3_sdk.join("pluginterfaces"));
    build.include(vst3_sdk.join("public.sdk").join("source"));
    build.include(vst3_sdk.join("base"));

    // Platform entry point
    if cfg!(target_os = "windows") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("dllmain.cpp"));
    } else if cfg!(target_os = "linux") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("linuxmain.cpp"));
        // On Linux, compile essential SDK files for VST3 factory
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("moduleinit.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("pluginfactory.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("hosting").join("processdata.cpp"));
        // Core VST3 implementation files needed for the plugin to work
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstcomponent.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstcomponentbase.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vsteditcontroller.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstaudioeffect.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstbus.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstparameters.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstpresetfile.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstrepresentation.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstinitiids.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("common").join("pluginview.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("common").join("commoniids.cpp"));
        // Base SDK files
        build.file(vst3_sdk.join("base").join("funknown.cpp"));
        build.file(vst3_sdk.join("base").join("coreiids.cpp"));
        build.file(vst3_sdk.join("base").join("ustring.cpp"));
        build.file(vst3_sdk.join("base").join("conststringtable.cpp"));
        build.file(vst3_sdk.join("base").join("source").join("fobject.cpp"));
        build.file(vst3_sdk.join("base").join("source").join("fstring.cpp"));
        build.file(vst3_sdk.join("base").join("source").join("fbuffer.cpp"));
        build.file(vst3_sdk.join("base").join("source").join("fstreamer.cpp"));
        build.file(vst3_sdk.join("base").join("thread").join("source").join("flock.cpp"));
        build.file(vst3_sdk.join("base").join("thread").join("source").join("fcondition.cpp"));
    } else if cfg!(target_os = "macos") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("macmain.cpp"));
        // On macOS, compile essential SDK files
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("moduleinit.cpp"));
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("pluginfactory.cpp"));
    }

    // Our plugin source
    build.file("src/vst3/periodic_instruments.cpp");

    if cfg!(target_os = "windows") {
        build.shared_flag(true);
        build.compile("periodic_instruments");
    } else if cfg!(target_os = "linux") {
        build.compile("periodic_instruments");
    } else if cfg!(target_os = "macos") {
        build.compile("periodic_instruments");
    }

    // Link the static library into a shared library
    if cfg!(target_os = "linux") {
        use std::process::Command;
        let so_path = out_dir.join("libperiodic_instruments.so");
        // Collect all .o files
        let mut cmd = Command::new("g++");
        cmd.arg("-shared")
           .arg("-fPIC")
           .arg("-Wl,--export-dynamic")
           .arg("-o")
           .arg(&so_path);
        if let Ok(entries) = std::fs::read_dir(&out_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "o") {
                    cmd.arg(&path);
                }
            }
        }
        cmd.arg("-lstdc++");
        let status = cmd.status().expect("Failed to link .so");
        if !status.success() {
            panic!("Failed to link shared library");
        }
        // Debug: show exported symbols
        eprintln!("DEBUG: Exported symbols from .so:");
        let _ = Command::new("nm")
            .arg("-D")
            .arg("--defined-only")
            .arg(&so_path)
            .status();
        // Also check for VST3 symbols specifically
        eprintln!("DEBUG: VST3 symbols:");
        let _ = Command::new("bash")
            .arg("-c")
            .arg(format!("nm -D --defined-only {} | grep -iE 'GetPluginFactory|ModuleEntry|ModuleExit|gPluginFactory'", so_path.display()))
            .status();
    }

    // Create VST3 bundle
    let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target"
    }));
    // When cross-compiling, the target directory includes the target triple
    let target = env::var("TARGET").unwrap_or_else(|_| env::var("HOST").unwrap_or_default());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    // The build output goes to target/<target>/<profile>/build/.../out
    // The final binary goes to target/<target>/<profile>/
    let profile_dir = target_dir.join(&target).join(&env::var("PROFILE").unwrap_or_else(|_| "release".to_string()));
    let bundle_name = "Periodic Instruments.vst3";
    let bundle_dir = profile_dir.join(bundle_name);

    create_vst3_bundle(&bundle_dir, &out_dir);

    // The C++ VST3 plugin is built as a shared library in the bundle
    // No need to link into the Rust library
}

fn create_vst3_bundle(bundle_dir: &Path, out_dir: &Path) {
    use std::fs;

    let contents_dir = bundle_dir.join("Contents");
    fs::create_dir_all(&contents_dir).expect("Failed to create bundle directory");

    // Platform-specific binary directory
    let target_os = env::var("TARGET").unwrap_or_else(|_| env::var("HOST").unwrap_or_default());
    let platform_dir = if target_os.contains("linux") {
        contents_dir.join("x86_64-linux")
    } else if target_os.contains("windows") {
        contents_dir.join("x86_64-win")
    } else if target_os.contains("macos") || target_os.contains("darwin") {
        contents_dir.join("MacOS")
    } else {
        // Default fallback
        contents_dir.join("x86_64-linux")
    };

    fs::create_dir_all(&platform_dir).expect("Failed to create platform directory");

    // Find and copy the compiled library to the bundle
    // For cross-compilation, the target OS might not match the build OS
    let target_os = env::var("TARGET").unwrap_or_else(|_| env::var("HOST").unwrap_or_default());
    let possible_names = if target_os.contains("windows") {
        vec!["periodic_instruments.dll", "periodic_instruments.lib"]
    } else if target_os.contains("macos") || target_os.contains("darwin") {
        vec!["libperiodic_instruments.dylib", "periodic_instruments.dylib"]
    } else {
        vec!["libperiodic_instruments.so", "periodic_instruments.so"]
    };

    let mut found = false;
    for name in &possible_names {
        let src_lib = out_dir.join(name);
        if src_lib.exists() {
            // Use target OS from TARGET env var, not cfg!
            let dst_name = if target_os.contains("windows") {
                "Periodic Instruments.dll"
            } else if target_os.contains("macos") || target_os.contains("darwin") {
                "Periodic Instruments.dylib"
            } else {
                "Periodic Instruments.so"
            };
            fs::copy(&src_lib, platform_dir.join(dst_name))
                .expect("Failed to copy library to bundle");
            found = true;
            break;
        }
    }

    if !found {
        // List OUT_DIR contents for debugging
        eprintln!("OUT_DIR contents:");
        if let Ok(entries) = fs::read_dir(out_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    eprintln!("  {}", entry.path().display());
                }
            }
        }
        panic!("Compiled library not found in OUT_DIR");
    }

    // Copy moduleinfo.json
    let module_info_src = Path::new("src/vst3/moduleinfo.json");
    if module_info_src.exists() {
        fs::copy(module_info_src, contents_dir.join("moduleinfo.json"))
            .expect("Failed to copy moduleinfo.json");
        eprintln!("DEBUG: Copied moduleinfo.json to bundle");
    } else {
        eprintln!("DEBUG: moduleinfo.json NOT FOUND at {:?}", module_info_src);
    }

    // Copy manifest.ttl
    let manifest_src = Path::new("src/vst3/manifest.ttl");
    if manifest_src.exists() {
        fs::copy(manifest_src, contents_dir.join("manifest.ttl"))
            .expect("Failed to copy manifest.ttl");
        eprintln!("DEBUG: Copied manifest.ttl to bundle");
    } else {
        eprintln!("DEBUG: manifest.ttl NOT FOUND at {:?}", manifest_src);
    }

    // Debug: list bundle contents
    eprintln!("DEBUG: Bundle contents:");
    if let Ok(entries) = fs::read_dir(&contents_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                eprintln!("  {}", entry.path().display());
            }
        }
    }
}