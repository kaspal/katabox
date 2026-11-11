//! Per-language runtime definitions and templates from `languages/<id>/`, embedded at
//! build time so the judge never reads templates from disk at run time.

use std::path::{Path, PathBuf};
use std::time::Duration;

use kb_core::Language;
use serde::Deserialize;

use crate::JudgeError;

/// One supported language: its `language.toml` plus its templates.
#[derive(Debug, Clone)]
pub struct LanguageDef {
    pub language: Language,
    /// The `compile` command line (empty for interpreted languages).
    pub compile: String,
    /// The `run` command line. Placeholders: `{main}`, `{binary}`, `{memory_mb}`,
    /// `{toolchain}`.
    pub run: String,
    /// Limits for the compile step (compiled languages only).
    pub compile_limits: Option<CompileLimits>,
    /// The language's time limit factor and startup allowance (ADR 0017).
    pub time: TimeDef,
    pub stub_template: &'static str,
    pub harness_template: &'static str,
}

/// Sandbox limits for one compile, from `[compile_limits]` in language.toml. Larger than
/// the run limits: compilers are heavier than solutions (ADR 0009).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompileLimits {
    pub cpu_time_ms: u32,
    pub wall_time_ms: u32,
    pub memory_mb: u32,
    pub max_processes: u32,
}

impl CompileLimits {
    pub fn wall_time(&self) -> Duration {
        Duration::from_millis(u64::from(self.wall_time_ms))
    }
}

/// `[time]` in language.toml: how this language's time limit derives from a problem's
/// base limit (ADR 0017). See [`crate::limits`].
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeDef {
    /// Multiplier on the problem's base limit (Rust is 1).
    pub factor: f64,
    /// CPU allowed on top of the limit for starting the runtime and the harness.
    pub startup_ms: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LanguageToml {
    id: String,
    extension: String,
    compile: String,
    run: String,
    #[serde(default)]
    compile_limits: Option<CompileLimits>,
    time: TimeDef,
    #[allow(dead_code)] // used by kb-bridge later
    language_server: String,
    #[allow(dead_code)]
    debug_adapter: String,
}

macro_rules! lang_file {
    ($lang:literal, $file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../languages/",
            $lang,
            "/",
            $file
        ))
    };
}

/// Languages the judge can run today.
pub const SUPPORTED: [Language; 4] = [
    Language::Python,
    Language::TypeScript,
    Language::Php,
    Language::Rust,
];

pub fn definition(lang: Language) -> Result<LanguageDef, JudgeError> {
    let (toml_src, stub_template, harness_template) = match lang {
        Language::Python => (
            lang_file!("python", "language.toml"),
            lang_file!("python", "stub.py.j2"),
            lang_file!("python", "harness.py.j2"),
        ),
        Language::TypeScript => (
            lang_file!("typescript", "language.toml"),
            lang_file!("typescript", "stub.ts.j2"),
            lang_file!("typescript", "harness.ts.j2"),
        ),
        Language::Php => (
            lang_file!("php", "language.toml"),
            lang_file!("php", "stub.php.j2"),
            lang_file!("php", "harness.php.j2"),
        ),
        Language::Rust => (
            lang_file!("rust", "language.toml"),
            lang_file!("rust", "stub.rs.j2"),
            lang_file!("rust", "harness.rs.j2"),
        ),
        Language::JavaScript => return Err(JudgeError::Unsupported(lang)),
    };
    let bad = |msg: String| JudgeError::Internal(format!("languages/{lang}/language.toml: {msg}"));
    let t: LanguageToml = toml::from_str(toml_src).map_err(|e| bad(e.to_string()))?;
    if t.id != lang.id() || t.extension != lang.extension() {
        return Err(bad(format!("does not match the {lang} definition")));
    }
    if lang.is_compiled() != !t.compile.is_empty()
        || t.compile_limits.is_some() != !t.compile.is_empty()
    {
        return Err(bad(
            "compiled languages need `compile` and `[compile_limits]`; others neither".into(),
        ));
    }
    if !(t.time.factor.is_finite() && t.time.factor >= 1.0) {
        return Err(bad("[time] factor must be a number ≥ 1".into()));
    }
    Ok(LanguageDef {
        language: lang,
        compile: t.compile,
        run: t.run,
        compile_limits: t.compile_limits,
        time: t.time,
        stub_template,
        harness_template,
    })
}

impl LanguageDef {
    /// The harness entry file (`main.py`, `main.rs`), relative to the work folder.
    pub fn main_file(&self) -> String {
        format!("main.{}", self.language.extension())
    }

    /// The user's solution file (`solution.py`, `solution.rs`), relative to the work folder.
    pub fn solution_file(&self) -> String {
        format!("solution.{}", self.language.extension())
    }

    /// The compiler's output file, relative to the work folder.
    pub fn binary_file(&self) -> String {
        if cfg!(windows) {
            "solution.exe".into()
        } else {
            "solution".into()
        }
    }

    /// Compiler (resolved inside `toolchain`) and arguments.
    pub fn compile_command(&self, toolchain: &Path) -> Result<(PathBuf, Vec<String>), JudgeError> {
        self.expand(&self.compile, toolchain, 0, None)
    }

    /// Program and arguments for one batch run. `binary` is the compiled program for
    /// compiled languages (`{binary}` in the run line).
    pub fn command(
        &self,
        toolchain: &Path,
        memory_mb: u32,
        binary: Option<&Path>,
    ) -> Result<(PathBuf, Vec<String>), JudgeError> {
        self.expand(&self.run, toolchain, memory_mb, binary)
    }

    fn expand(
        &self,
        line: &str,
        toolchain: &Path,
        memory_mb: u32,
        binary: Option<&Path>,
    ) -> Result<(PathBuf, Vec<String>), JudgeError> {
        let main = self.main_file();
        let out = self.binary_file();
        let tc = toolchain.to_string_lossy();
        // Split first, then substitute, so paths with spaces stay one argument.
        let mut words = line.split_whitespace().map(|w| {
            w.replace("{main}", &main)
                .replace("{binary}", &out)
                .replace("{memory_mb}", &memory_mb.to_string())
                .replace("{toolchain}", &tc)
        });
        let first = line.split_whitespace().next().unwrap_or_default();
        let name = words
            .next()
            .ok_or_else(|| JudgeError::Internal(format!("empty command for {}", self.language)))?;
        let program = if first == "{binary}" {
            binary
                .map(Path::to_path_buf)
                .ok_or_else(|| JudgeError::Internal("run needs the compiled binary".into()))?
        } else {
            resolve_program(toolchain, &name)?
        };
        Ok((program, words.collect()))
    }
}

/// `python` -> `<toolchain>/python.exe` (or `bin/python.exe`, as for rustc) on Windows,
/// `<toolchain>/bin/python` (or `bin/python3`) on Linux.
fn resolve_program(toolchain: &Path, name: &str) -> Result<PathBuf, JudgeError> {
    let candidates: Vec<PathBuf> = if cfg!(windows) {
        vec![
            toolchain.join(format!("{name}.exe")),
            toolchain.join("bin").join(format!("{name}.exe")),
        ]
    } else {
        vec![
            toolchain.join("bin").join(name),
            toolchain.join("bin").join(format!("{name}3")),
        ]
    };
    candidates
        .iter()
        .find(|p| p.is_file())
        .cloned()
        .ok_or_else(|| {
            JudgeError::Toolchain(format!(
                "{name} not found in {} (looked for {})",
                toolchain.display(),
                candidates
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_languages_load() {
        for lang in SUPPORTED {
            let d = definition(lang).unwrap();
            assert!(d.run.contains("{main}") || d.run == "{binary}", "{lang}");
            assert_eq!(d.compile_limits.is_some(), lang.is_compiled(), "{lang}");
            assert!(d.time.factor >= 1.0 && d.time.startup_ms > 0, "{lang}");
        }
        // Rust is the base language: every other factor is relative to it.
        assert_eq!(definition(Language::Rust).unwrap().time.factor, 1.0);
        assert!(matches!(
            definition(Language::JavaScript),
            Err(JudgeError::Unsupported(Language::JavaScript))
        ));
    }

    /// A fake toolchain folder with empty program files, removed on drop.
    struct FakeToolchain(PathBuf);
    impl FakeToolchain {
        fn new(name: &str, programs: &[&str]) -> Self {
            let dir = std::env::temp_dir().join(format!("kbj-{name}-{}", std::process::id()));
            for p in programs {
                let exe = if cfg!(windows) {
                    dir.join(format!("{p}.exe"))
                } else {
                    dir.join("bin").join(p)
                };
                std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
                std::fs::write(&exe, "").unwrap();
            }
            FakeToolchain(dir)
        }
    }
    impl Drop for FakeToolchain {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn command_substitutes_placeholders() {
        let tc = FakeToolchain::new("node", &["node"]);
        let (program, args) = definition(Language::TypeScript)
            .unwrap()
            .command(&tc.0, 256, None)
            .unwrap();
        assert!(program.starts_with(&tc.0));
        assert!(args.contains(&"--max-old-space-size=256".to_string()));
        assert_eq!(args.last().unwrap(), "main.ts");
        let missing = definition(Language::Python)
            .unwrap()
            .command(&tc.0, 256, None);
        assert!(matches!(missing, Err(JudgeError::Toolchain(_))));
    }

    #[test]
    fn php_gets_its_extension_dir() {
        let tc = FakeToolchain::new("php dir", &["php"]);
        let (_, args) = definition(Language::Php)
            .unwrap()
            .command(&tc.0, 128, None)
            .unwrap();
        let ext = format!("extension_dir={}", tc.0.join("ext").display()).replace('\\', "/");
        assert!(
            args.iter().any(|a| a.replace('\\', "/") == ext),
            "{args:?} lacks {ext}"
        );
        assert!(args.contains(&"memory_limit=128M".to_string()));
    }

    #[test]
    fn rust_compiles_then_runs_the_binary() {
        let def = definition(Language::Rust).unwrap();
        let tc = FakeToolchain::new("rust", &["rustc"]);
        let (rustc, args) = def.compile_command(&tc.0).unwrap();
        assert!(rustc.ends_with(if cfg!(windows) { "rustc.exe" } else { "rustc" }));
        assert!(args.contains(&"-O".to_string()));
        assert!(args.contains(&def.binary_file()));
        assert_eq!(args.last().unwrap(), "main.rs");

        let bin = tc.0.join("cached.exe");
        let (program, run_args) = def.command(&tc.0, 256, Some(&bin)).unwrap();
        assert_eq!(program, bin);
        assert!(run_args.is_empty());
        assert!(def.command(&tc.0, 256, None).is_err());
        assert!(def.compile_limits.unwrap().memory_mb > 256);
    }
}
