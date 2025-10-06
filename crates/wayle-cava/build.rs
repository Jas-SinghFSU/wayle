//! Build script for linking libcava.

fn main() {
    if let Err(e) = pkg_config::probe_library("cava") {
        eprintln!("Warning: pkg-config failed to find cava: {e}");
        println!("cargo:rustc-link-lib=cava");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
