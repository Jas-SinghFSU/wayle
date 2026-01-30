//! Build script for libcava FFI bindings.
//!
//! Supports two modes:
//! - System libcava via pkg-config (default)
//! - Vendored build from submodule (`vendored` feature)
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use std::{env, path::PathBuf};

const REQUIRED_VERSION: &str = "0.10.6";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "vendored")]
    let include_paths = build_vendored();

    #[cfg(not(feature = "vendored"))]
    let include_paths = build_system();

    generate_bindings(&include_paths);
}

#[cfg(not(feature = "vendored"))]
fn build_system() -> Vec<PathBuf> {
    let lib =
        pkg_config::probe_library("libcava").unwrap_or_else(|e| panic!("libcava not found: {e}"));

    println!("cargo:rustc-env=LIBCAVA_VERSION={}", lib.version);
    println!("cargo:rustc-link-lib=cava");

    lib.include_paths
}

#[cfg(feature = "vendored")]
fn build_vendored() -> Vec<PathBuf> {
    use std::path::Path;

    let cava_dir = Path::new("cava");
    let src_dir = cava_dir.join("src");
    let include_dir = cava_dir.join("include");

    let fftw = pkg_config::probe_library("fftw3").expect("fftw3 required");
    let pipewire = pkg_config::probe_library("libpipewire-0.3").expect("libpipewire-0.3 required");
    let pulse = pkg_config::probe_library("libpulse").ok();

    if pulse.is_none() {
        println!("cargo:warning=libpulse not found, building without PulseAudio support");
    }

    let mut build = cc::Build::new();
    build
        .include(&include_dir)
        .define("PACKAGE", "\"cava\"")
        .define("VERSION", format!("\"{REQUIRED_VERSION}\"").as_str())
        .define("NDEBUG", None)
        .define("PIPEWIRE", None)
        .warnings(false)
        .extra_warnings(false);

    for path in &fftw.include_paths {
        build.include(path);
    }
    for path in &pipewire.include_paths {
        build.include(path);
    }

    if let Some(ref pulse_lib) = pulse {
        build.define("PULSE", None);
        for path in &pulse_lib.include_paths {
            build.include(path);
        }
    }

    let sources = [
        "cavacore.c",
        "common.c",
        "input/common.c",
        "input/fifo.c",
        "input/shmem.c",
        "input/pipewire.c",
        "output/common.c",
        "output/raw.c",
        "output/noritake.c",
        "output/terminal_noncurses.c",
    ];

    for source in &sources {
        build.file(src_dir.join(source));
    }

    if pulse.is_some() {
        build.file(src_dir.join("input/pulse.c"));
    }

    build.compile("cava");

    println!("cargo:rustc-link-lib=static=cava");
    println!("cargo:rustc-link-lib=fftw3");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");

    for lib_name in &pipewire.libs {
        println!("cargo:rustc-link-lib={lib_name}");
    }

    if let Some(ref pulse_lib) = pulse {
        for lib_name in &pulse_lib.libs {
            println!("cargo:rustc-link-lib={lib_name}");
        }
        println!("cargo:rustc-link-lib=pulse-simple");
    }

    println!("cargo:rustc-env=LIBCAVA_VERSION={REQUIRED_VERSION}");

    vec![include_dir]
}

fn generate_bindings(include_paths: &[PathBuf]) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .rust_target(bindgen::RustTarget::stable(82, 0).expect("valid Rust version"))
        .raw_line("pub type fftw_plan = *mut fftw_plan_s;")
        .raw_line("pub type fftw_complex = [f64; 2];")
        .allowlist_type("cava_plan")
        .allowlist_type("config_params")
        .allowlist_type("audio_data")
        .allowlist_type("audio_raw")
        .allowlist_type("input_method")
        .allowlist_type("output_method")
        .allowlist_type("mono_option")
        .allowlist_type("xaxis_scale")
        .allowlist_type("orientation")
        .allowlist_type("data_format")
        .allowlist_function("cava_init")
        .allowlist_function("cava_execute")
        .allowlist_function("cava_destroy")
        .allowlist_function("get_input")
        .allowlist_function("audio_raw_init")
        .allowlist_function("audio_raw_clean")
        .allowlist_function("audio_raw_destroy")
        .blocklist_type("fftw_plan")
        .blocklist_type("fftw_complex")
        .opaque_type("fftw_plan_s")
        .opaque_type("fftw_complex")
        .derive_debug(true)
        .derive_default(false)
        .generate_comments(false)
        .layout_tests(true)
        .wrap_unsafe_ops(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for path in include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .generate()
        .expect("Failed to generate libcava bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
