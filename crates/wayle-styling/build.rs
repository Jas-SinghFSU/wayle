#![allow(clippy::expect_used, missing_docs)]

use std::{env, fs, path::Path};

fn main() {
    let scss_dir = Path::new("scss");
    let main_path = scss_dir.join("main.scss");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set");

    println!("cargo::rerun-if-changed=scss");

    let options = grass::Options::default().load_path(scss_dir);
    let css = grass::from_path(&main_path, &options).expect("SCSS compilation failed");

    fs::write(Path::new(&out_dir).join("style.css"), css).expect("failed to write CSS");
}
