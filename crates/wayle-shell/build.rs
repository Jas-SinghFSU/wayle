#![allow(clippy::expect_used)]

use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let locales_dir = Path::new("locales");

    for entry in fs::read_dir(locales_dir).expect("locales/ directory must exist") {
        let locale_dir = entry.expect("readable directory entry").path();
        if locale_dir.is_dir() {
            concatenate_partials(&locale_dir);
        }
    }
}

fn concatenate_partials(locale_dir: &Path) {
    let partials = collect_partials_recursive(locale_dir);
    let combined = merge_partials(&partials);
    let output = locale_dir.join("wayle-shell.ftl");
    fs::write(&output, combined).expect("failed to write combined ftl");
}

fn collect_partials_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut partials = Vec::new();
    collect_partials_inner(dir, &mut partials);
    partials.sort();
    partials
}

fn collect_partials_inner(dir: &Path, partials: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_dir() {
            collect_partials_inner(&path, partials);
        } else if is_partial(&path) {
            partials.push(path);
        }
    }
}

fn is_partial(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| name.starts_with('_') && name.ends_with(".ftl"))
}

fn merge_partials(partials: &[PathBuf]) -> String {
    let mut combined = String::new();

    for partial in partials {
        let content = fs::read_to_string(partial).expect("ftl file readable");
        combined.push_str(&content);
        combined.push('\n');
        println!("cargo::rerun-if-changed={}", partial.display());
    }

    combined
}
