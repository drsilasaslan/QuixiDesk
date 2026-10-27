//! Resolves the (optional) reverse-engineered EIPC schema catalogue.
//!
//! The full catalogue lives outside the repository (`reference/`) because it is a
//! reverse-engineering artifact. When it is absent we fall back to the checked-in
//! empty stub in `assets/`, so the crate always builds.

use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("ipc-exact-schemas.json");

    let external = manifest_dir.join("../../reference/ipc-exact-schemas.json");
    let stub = manifest_dir.join("assets/ipc-exact-schemas.json");

    println!("cargo:rerun-if-changed={}", external.display());
    println!("cargo:rerun-if-changed={}", stub.display());

    let source = if is_usable(&external) { external } else { stub };
    std::fs::copy(&source, &out_path)
        .unwrap_or_else(|e| panic!("failed to stage {}: {e}", source.display()));
}

fn is_usable(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(meta) if meta.len() > 8 => std::fs::read_to_string(path)
            .map(|body| body.contains("\"methods\""))
            .unwrap_or(false),
        _ => false,
    }
}
