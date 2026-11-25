// Build script for Periodic Instruments VST3 plugin
// Compiles the C++ VST3 plugin using the `cc` crate
// VST3 SDK is included in the repository at vst3sdk/

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/vst3/periodic_instruments.cpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/vst3/moduleinfo.json");
    println!("cargo:rerun-if-changed=vst3sdk");

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // VST3 SDK is in the repository
    let vst3_sdk = manifest_dir.join("vst3sdk");
    if !vst3_sdk.exists() {
        panic!("VST3 SDK not found at {}. The VST3 SDK must be in the vst3sdk/ directory.", vst3_sdk.display());
    }

    // Build the C++ VST3 plugin using the `cc` crate
    let mut build = cc::Build::new();
    build.cpp(true)
         .flag_if_supported("-std=c++17")
         .flag_if_supported("/std:c++17")
         .define("VST_VERSION_STR", Some("\"0.1.0\""))
         .define("NDEBUG", Some("1"))
         .opt_level(3)
         .pic(true);

    // Add VST3 SDK include paths
    build.include(vst3_sdk.join("pluginterfaces"));
    build.include(vst3_sdk.join("public.sdk").join("source"));
    build.include(vst3_sdk.join("base"));

    // Also need base source for .cpp files
    build.file(vst3_sdk.join("base").join("funknown.cpp"));
    build.file(vst3_sdk.join("base").join("ustring.cpp"));
    build.file(vst3_sdk.join("base").join("conststringtable.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstcomponent.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstcomponentbase.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vsteditcontroller.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstaudioeffect.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstbus.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstparameters.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstpresetfile.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstrepresentation.cpp"));
    build.file(vst3_sdk.join("public.sdk").join("source").join("vst").join("vstinitiids.cpp"));

    // Compile SDK main files
    if cfg!(target_os = "windows") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("dllmain.cpp"));
    } else if cfg!(target_os = "linux") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("linuxmain.cpp"));
    } else if cfg!(target_os = "macos") {
        build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("macmain.cpp"));
    }
    build.file(vst3_sdk.join("public.sdk").join("source").join("main").join("moduleinit.cpp"));

    // Compile the plugin source
    build.file("src/vst3/periodic_instruments.cpp");

    if cfg!(target_os = "windows") {
        build.compile("periodic_instruments");
    } else if cfg!(target_os = "linux") {
        build.compile("periodic_instruments");
    } else if cfg!(target_os = "macos") {
        build.compile("periodic_instruments");
    }

    // Create VST3 bundle
    let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target"
    }));
    let profile_dir = target_dir.join(&env::var("PROFILE").unwrap());
    let bundle_name = "Periodic Instruments.vst3";
    let bundle_dir = profile_dir.join(bundle_name);

    create_vst3_bundle(&bundle_dir);

    // Link the library
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=periodic_instruments");
}

fn create_vst3_bundle(bundle_dir: &Path) {
    use std::fs;

    let contents_dir = bundle_dir.join("Contents");
    fs::create_dir_all(&contents_dir).expect("Failed to create bundle directory");

    // Copy moduleinfo.json
    let module_info_src = Path::new("src/vst3/moduleinfo.json");
    if module_info_src.exists() {
        fs::copy(module_info_src, contents_dir.join("moduleinfo.json"))
            .expect("Failed to copy moduleinfo.json");
    }
}