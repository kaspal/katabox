//! KataBox problem packs: validation and the content.db build, shared by the `kb-pack`
//! CLI (see `main.rs`) and the desktop app's build script.

mod build;
mod validate;

pub use build::{BuildOptions, build};
pub use validate::{run_references, validate_all};

/// Validate every problem under `dir` and compile them into content.db at `out` (see
/// [`build`]). Returns the problem count.
pub fn build_pack(
    dir: &std::path::Path,
    out: &std::path::Path,
    pack_id: &str,
    pack_version: &str,
) -> Result<usize, String> {
    build(&BuildOptions {
        dir: dir.to_path_buf(),
        out: out.to_path_buf(),
        pack_id: pack_id.to_string(),
        pack_version: pack_version.to_string(),
    })
}
