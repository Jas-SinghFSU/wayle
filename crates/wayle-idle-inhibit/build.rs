//! Build script for wayle-idle-inhibit.
//!
//! this crate intentionally does not force link order for
//! libwayland-client. Binaries that also use `gtk4-layer-shell` must ensure
//! `gtk4-layer-shell` is linked before `wayland-client` so its interposition
//! works. Handle that in the final binary (via its build script).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
}
