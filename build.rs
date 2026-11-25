// Build script for Periodic Instruments VST3 plugin
// Compiles the C++ VST3 plugin using the `cc` crate

use std::env;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=src/vst3/periodic_instruments.cpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/vst3/moduleinfo.json");

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

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
    if let Ok(vst3_sdk) = env::var("VST3_SDK") {
        let sdk = PathBuf::from(vst3_sdk);
        build.include(sdk.join("pluginterfaces"));
        build.include(sdk.join("public.sdk").join("source"));
        build.include(sdk.join("base").join("source"));
    } else {
        // Try common locations
        if cfg!(target_os = "windows") {
            for path in &[
                r"C:\Program Files\Steinberg\VST3 SDK",
                r"C:\Program Files (x86)\Steinberg\VST3 SDK",
            ] {
                if Path::new(path).exists() {
                    let p = PathBuf::from(path);
                    build.include(p.join("pluginterfaces"));
                    build.include(p.join("public.sdk").join("source"));
                    build.include(p.join("base").join("source"));
                    break;
                }
            }
        } else if cfg!(target_os = "linux") {
            for path in &[
                "/usr/local/vst3sdk",
                "/opt/vst3sdk",
                "/usr/include/vst3sdk",
            ] {
                if Path::new(path).exists() {
                    let p = PathBuf::from(path);
                    build.include(p.join("pluginterfaces"));
                    build.include(p.join("public.sdk").join("source"));
                    build.include(p.join("base").join("source"));
                    break;
                }
            }
        } else if cfg!(target_os = "macos") {
            for path in &[
                "/Applications/VST3 SDK",
                "/Library/Audio/Plug-Ins/VST3 SDK",
            ] {
                if Path::new(path).exists() {
                    let p = PathBuf::from(path);
                    build.include(p.join("pluginterfaces"));
                    build.include(p.join("public.sdk").join("source"));
                    build.include(p.join("base").join("source"));
                    break;
                }
            }
        }
    }

    // Compile the C++ plugin
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

    // Platform-specific bundle structure
    #[cfg(target_os = "windows")]
    let contents_dir = bundle_dir.join("Contents");
    #[cfg(target_os = "linux")]
    let contents_dir = bundle_dir.join("Contents");
    #[cfg(target_os = "macos")]
    let contents_dir = bundle_dir.join("Contents");

    fs::create_dir_all(&contents_dir).expect("Failed to create bundle directory");

    // Copy moduleinfo.json
    let module_info_src = Path::new("src/vst3/moduleinfo.json");
    if module_info_src.exists() {
        fs::copy(module_info_src, contents_dir.join("moduleinfo.json"))
            .expect("Failed to copy moduleinfo.json");
    }

    // The actual binary will be placed by the build system
    // This is a placeholder - the actual binary placement depends on the platform
}