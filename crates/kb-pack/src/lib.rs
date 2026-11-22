//! KataBox problem packs: validation and the content.db build, shared by the `kb-pack`
//! CLI (see `main.rs`) and the desktop app's build script.

mod build;
pub mod checks;
pub mod kbpack;
pub mod learn;
pub mod pack;
pub mod profile;
pub mod reading;
pub mod scaffold;
mod validate;

pub use build::{BuildOptions, build};
pub use checks::{Check, Finding, check_problem, env_for, generate_tests};
pub use learn::TopicFilter;
pub use validate::{
    TEST_BUDGET_BYTES, budget_error, run_references, select_problems, test_size, validate_all,
    validate_learn_only,
};

/// Structural checks of one problem folder, as for built-in content (packs and the CLI's
/// `pack` command use them; expansion M11).
pub fn validate_problem_dir(dir: &std::path::Path) -> (Option<kb_core::Difficulty>, Vec<String>) {
    validate::validate_problem_meta(dir)
}

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
        pack_date: None,
    })
}

/// [`build_pack`] with the pack's id, version and date from `content/pack.toml` next to
/// `dir` (see [`pack`]); without that file, `core` and `fallback_version`, no date.
pub fn build_content(
    dir: &std::path::Path,
    out: &std::path::Path,
    fallback_version: &str,
) -> Result<usize, String> {
    let info = pack::read_pack_info(dir)?;
    build(&BuildOptions {
        dir: dir.to_path_buf(),
        out: out.to_path_buf(),
        pack_id: info.as_ref().map_or("core".into(), |i| i.id.clone()),
        pack_version: info
            .as_ref()
            .map_or(fallback_version.into(), |i| i.version.clone()),
        pack_date: info.map(|i| i.date),
    })
}
