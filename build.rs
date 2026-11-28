//! Build script for VST3 plugin bundle creation
//!
//! This build script:
//! 1. Locates the VST3 SDK (via VST3_SDK env var or default locations)
//! 2. Configures vst3-sys crate
//! 3. Creates the .vst3 bundle structure after build

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to rerun if these change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=VST3_SDK");

    // Find VST3 SDK
    let vst3_sdk = find_vst3_sdk();

    if let Some(sdk_path) = vst3_sdk {
        println!("cargo:rustc-env=VST3_SDK={}", sdk_path.display());
        println!("cargo:info=VST3 SDK found at: {}", sdk_path.display());
    } else {
        println!("cargo:warning=VST3 SDK not found. Set VST3_SDK environment variable or install SDK to default location.");
        println!("cargo:warning=On Windows: C:\\Program Files\\Steinberg\\VST3 SDK");
        println!("cargo:warning=On Linux: /usr/local/vst3sdk or /opt/vst3sdk");
    }

    // Set up vst3-sys
    println!("cargo:rustc-cfg=vst3_sdk_found");

    // Platform-specific linker args
    if cfg!(target_os = "windows") {
        // Windows: link against VST3 SDK libraries if needed
        println!("cargo:rustc-link-search=native={}/VST3_SDK", env::var("VST3_SDK").unwrap_or_default());
    } else if cfg!(target_os = "linux") {
        // Linux: ensure we link correctly
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=rt");
    }

    // Post-build: create VST3 bundle
    if env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() != "wasm" {
        create_vst3_bundle();
    }
}

/// Find VST3 SDK installation
fn find_vst3_sdk() -> Option<PathBuf> {
    // 1. Check environment variable
    if let Ok(sdk) = env::var("VST3_SDK") {
        let path = PathBuf::from(&sdk);
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Check default locations by platform
    let default_paths = if cfg!(target_os = "windows") {
        vec![
            PathBuf::from(r"C:\Program Files\Steinberg\VST3 SDK"),
            PathBuf::from(r"C:\Program Files (x86)\Steinberg\VST3 SDK"),
            PathBuf::from(r"C:\VST3 SDK"),
        ]
    } else if cfg!(target_os = "linux") {
        vec![
            PathBuf::from("/usr/local/vst3sdk"),
            PathBuf::from("/opt/vst3sdk"),
            PathBuf::from("/usr/include/vst3sdk"),
            PathBuf::from("/home/linuxbrew/.linuxbrew/include/vst3sdk"),
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            PathBuf::from("/Library/Audio/Plug-Ins/VST3/"),
            PathBuf::from("/usr/local/vst3sdk"),
        ]
    } else {
        vec![]
    };

    for path in default_paths {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Create VST3 bundle after compilation
fn create_vst3_bundle() {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    let out_dir = PathBuf::from(&target_dir).join(&profile);

    let plugin_name = "Periodic Instruments";
    let bundle_name = format!("{}.vst3", plugin_name);

    #[cfg(target_os = "windows")]
    {
        create_windows_bundle(&out_dir, plugin_name, &bundle_name);
    }

    #[cfg(target_os = "linux")]
    {
        create_linux_bundle(&out_dir, plugin_name, &bundle_name);
    }

    #[cfg(target_os = "macos")]
    {
        create_macos_bundle(&out_dir, plugin_name, &bundle_name);
    }
}

#[cfg(target_os = "windows")]
fn create_windows_bundle(out_dir: &Path, plugin_name: &str, bundle_name: &str) {
    use std::fs;

    let dll_name = format!("{}.dll", plugin_name);
    let dll_path = out_dir.join(&dll_name);

    if !dll_path.exists() {
        // Try alternative naming
        let alt_dll = out_dir.join("periodic_instruments.dll");
        if alt_dll.exists() {
            fs::copy(&alt_dll, &dll_path).ok();
        } else {
            eprintln!("Warning: DLL not found at {:?}", dll_path);
            return;
        }
    }

    let bundle_dir = out_dir.join(bundle_name);
    let contents_dir = bundle_dir.join("Contents");
    let x64_dir = contents_dir.join("x86_64-win");
    let resources_dir = contents_dir.join("Resources");

    fs::create_dir_all(&x64_dir).ok();
    fs::create_dir_all(&resources_dir).ok();

    // Copy DLL
    let dest_dll = x64_dir.join(&dll_name);
    fs::copy(&dll_path, &dest_dll).ok();

    // Create moduleinfo.json
    let module_info = format!(r#"{{
  "factory": "{}",
  "classes": [
    {{
      "name": "{}",
      "cid": "70657269-6f64-6963-496e-737472756d656e747300",
      "category": "Instrument|Synth",
      "version": "{}",
      "vendor": "{}",
      "sdkVersion": "3.7.0"
    }}
  ]
}}"#, plugin_name, plugin_name, env!("CARGO_PKG_VERSION"), "AlZrSe");

    fs::write(resources_dir.join("moduleinfo.json"), module_info).ok();

    println!("cargo:info=Created VST3 bundle at {:?}", bundle_dir);
}

#[cfg(target_os = "linux")]
fn create_linux_bundle(out_dir: &Path, plugin_name: &str, bundle_name: &str) {
    use std::fs;

    let so_name = format!("lib{}.so", plugin_name.replace(' ', ""));
    let so_path = out_dir.join(&so_name);

    if !so_path.exists() {
        // Try alternative naming
        let alt_so = out_dir.join("libperiodic_instruments.so");
        if alt_so.exists() {
            fs::copy(&alt_so, &so_path).ok();
        } else {
            eprintln!("Warning: Shared library not found at {:?}", so_path);
            return;
        }
    }

    let bundle_dir = out_dir.join(bundle_name);
    let contents_dir = bundle_dir.join("Contents");
    let x64_dir = contents_dir.join("x86_64-linux");
    let resources_dir = contents_dir.join("Resources");

    fs::create_dir_all(&x64_dir).ok();
    fs::create_dir_all(&resources_dir).ok();

    // Copy .so
    let dest_so = x64_dir.join(&so_name);
    fs::copy(&so_path, &dest_so).ok();

    // Create moduleinfo.json
    let module_info = format!(r#"{{
  "factory": "{}",
  "classes": [
    {{
      "name": "{}",
      "cid": "70657269-6f64-6963-496e-737472756d656e747300",
      "category": "Instrument|Synth",
      "version": "{}",
      "vendor": "{}",
      "sdkVersion": "3.7.0"
    }}
  ]
}}"#, plugin_name, plugin_name, env!("CARGO_PKG_VERSION"), "AlZrSe");

    fs::write(resources_dir.join("moduleinfo.json"), module_info).ok();

    println!("cargo:info=Created VST3 bundle at {:?}", bundle_dir);
}

#[cfg(target_os = "macos")]
fn create_macos_bundle(out_dir: &Path, plugin_name: &str, bundle_name: &str) {
    use std::fs;

    let dylib_name = format!("lib{}.dylib", plugin_name.replace(' ', ""));
    let dylib_path = out_dir.join(&dylib_name);

    if !dylib_path.exists() {
        let alt_dylib = out_dir.join("libperiodic_instruments.dylib");
        if alt_dylib.exists() {
            fs::copy(&alt_dylib, &dylib_path).ok();
        } else {
            eprintln!("Warning: dylib not found at {:?}", dylib_path);
            return;
        }
    }

    let bundle_dir = out_dir.join(bundle_name);
    let contents_dir = bundle_dir.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let resources_dir = contents_dir.join("Resources");

    fs::create_dir_all(&macos_dir).ok();
    fs::create_dir_all(&resources_dir).ok();

    // Copy dylib
    let dest_dylib = macos_dir.join(plugin_name);
    fs::copy(&dylib_path, &dest_dylib).ok();

    // Create Info.plist
    let info_plist = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>{}</string>
    <key>CFBundleIdentifier</key>
    <string>com.alzrse.periodicinstruments</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>{}</string>
    <key>CSResourcesFileMapped</key>
    <true/>
</dict>
</plist>"#, plugin_name, plugin_name, env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_VERSION"));

    fs::write(contents_dir.join("Info.plist"), info_plist).ok();

    // Create moduleinfo.json
    let module_info = format!(r#"{{
  "factory": "{}",
  "classes": [
    {{
      "name": "{}",
      "cid": "70657269-6f64-6963-496e-737472756d656e747300",
      "category": "Instrument|Synth",
      "version": "{}",
      "vendor": "{}",
      "sdkVersion": "3.7.0"
    }}
  ]
}}"#, plugin_name, plugin_name, env!("CARGO_PKG_VERSION"), "AlZrSe");

    fs::write(resources_dir.join("moduleinfo.json"), module_info).ok();

    println!("cargo:info=Created VST3 bundle at {:?}", bundle_dir);
}