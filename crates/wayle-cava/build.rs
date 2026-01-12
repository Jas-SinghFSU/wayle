//! Build script for generating libcava FFI bindings via bindgen.

use std::{env, path::PathBuf};

const REQUIRED_VERSION: &str = "0.10.6";

#[allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let lib =
        pkg_config::probe_library("cava").unwrap_or_else(|e| panic!("libcava not found: {e}"));

    let version = &lib.version;
    if version != REQUIRED_VERSION {
        panic!("libcava version mismatch: found {version}, required {REQUIRED_VERSION}");
    }

    println!("cargo:rustc-env=LIBCAVA_VERSION={version}");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(
            lib.include_paths
                .iter()
                .map(|p| format!("-I{}", p.display())),
        )
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate libcava bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");

    println!("cargo:rustc-link-lib=cava");
}
