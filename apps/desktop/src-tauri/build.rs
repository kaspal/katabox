use std::path::PathBuf;

fn main() {
    // content.db is a bundled resource: build it from content/problems first, so that
    // `pnpm dev`, `tauri build` and a plain `cargo check` all have it (ADR 0011).
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let problems = manifest.join("../../../content/problems");
    let out = manifest.join("resources/content.db");
    println!("cargo:rerun-if-changed={}", problems.display());
    let version = std::env::var("CARGO_PKG_VERSION").expect("package version");
    if let Err(e) = kb_pack::build_pack(&problems, &out, "core", &version) {
        panic!("building {}: {e}", out.display());
    }
    tauri_build::build()
}
