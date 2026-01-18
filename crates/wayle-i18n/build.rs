#![allow(clippy::expect_used, missing_docs)]

use std::{
    fs::{self, DirEntry},
    path::Path,
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
    let partials = collect_partials(locale_dir);
    let combined = merge_partials(&partials);
    let output = locale_dir.join("wayle-i18n.ftl");
    fs::write(&output, combined).expect("failed to write combined ftl");
}

fn collect_partials(locale_dir: &Path) -> Vec<DirEntry> {
    let mut partials: Vec<_> = fs::read_dir(locale_dir)
        .expect("locale directory readable")
        .filter_map(Result::ok)
        .filter(is_partial)
        .collect();

    partials.sort_by_key(DirEntry::file_name);
    partials
}

fn is_partial(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|name| name.starts_with('_') && name.ends_with(".ftl"))
}

fn merge_partials(partials: &[DirEntry]) -> String {
    let mut combined = String::new();

    for partial in partials {
        let content = fs::read_to_string(partial.path()).expect("ftl file readable");
        combined.push_str(&content);
        combined.push('\n');
        println!("cargo::rerun-if-changed={}", partial.path().display());
    }

    combined
}
