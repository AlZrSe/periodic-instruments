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
    let mut build = cc::Build::new();
    build.cpp(true)
         .flag_if_supported("-std=c++17")
         .flag_if_supported("/std:c++17")
         .define("VST_VERSION_STR", Some("\"0.1.0\""))
         .define("NDEBUG", Some("1"))
         .opt_level(3)
         .pic(true)
         .flag("-fvisibility=default");

    // Add VST3 SDK include paths
    build.include(&vst3_sdk);  // For "pluginterfaces/base/fplatform.h" style includes
    build.include(vst3_sdk.join("pluginterfaces"));
    build.include(vst3_sdk.join("public.sdk").join("source"));
    build.include(vst3_sdk.join("base"));

    // Collect ALL .cpp files under base/ and public.sdk/source/ (excluding
    // platform-specific wrappers we don't need) so every IID and helper impl
    // is compiled in. This avoids undefined-symbol errors at dlopen time.
    fn collect_cpp(dir: &Path, out: &mut Vec<PathBuf>, exclude: &[&str]) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if exclude.contains(&name) { continue; }
                    collect_cpp(&path, out, exclude);
                } else if path.extension().map_or(false, |e| e == "cpp") {
                    out.push(path);
                }
            }
        }
    }
    // Exclude the "main" directory from auto-collection; we add the correct
    // platform entry point manually below.
    let exclude = ["aaxwrapper", "auwrapper", "auv3wrapper", "interappaudio", "main"];
    let mut sdk_sources: Vec<PathBuf> = Vec::new();
    collect_cpp(&vst3_sdk.join("base"), &mut sdk_sources, &exclude);
    collect_cpp(&vst3_sdk.join("public.sdk").join("source"), &mut sdk_sources, &exclude);
    for f in &sdk_sources {
        build.file(f);
    }

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

    // Link the static library into a shared library (.so)
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let so_path = out_dir.join("libperiodic_instruments.so");
        // Collect all .o files
        let mut cmd = Command::new("g++");
        cmd.arg("-shared").arg("-fPIC").arg("-Wl,--export-dynamic").arg("-o").arg(&so_path);
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
    }

    // Create VST3 bundle
    let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target"
    }));
    let profile_dir = target_dir.join(&env::var("PROFILE").unwrap());
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
    #[cfg(target_os = "linux")]
    let platform_dir = contents_dir.join("x86_64-linux");
    #[cfg(target_os = "windows")]
    let platform_dir = contents_dir.join("x86_64-win");
    #[cfg(target_os = "macos")]
    let platform_dir = contents_dir.join("MacOS");

    fs::create_dir_all(&platform_dir).expect("Failed to create platform directory");

    // Find and copy the compiled library to the bundle
    let possible_names = if cfg!(target_os = "windows") {
        vec!["periodic_instruments.dll", "periodic_instruments.lib"]
    } else if cfg!(target_os = "macos") {
        vec!["libperiodic_instruments.dylib", "periodic_instruments.dylib"]
    } else {
        vec!["libperiodic_instruments.so", "periodic_instruments.so"]
    };

    let mut found = false;
    for name in &possible_names {
        let src_lib = out_dir.join(name);
        if src_lib.exists() {
            let dst_name = if cfg!(target_os = "windows") {
                "Periodic Instruments.dll"
            } else if cfg!(target_os = "macos") {
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
    }
}
