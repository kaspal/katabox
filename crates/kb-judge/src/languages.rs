//! Per-language runtime definitions and templates from `languages/<id>/`, embedded at
//! build time so the judge never reads templates from disk at run time.

use std::path::{Path, PathBuf};

use kb_core::Language;
use serde::Deserialize;

use crate::JudgeError;

/// One supported language: its `language.toml` plus its templates.
#[derive(Debug, Clone)]
pub struct LanguageDef {
    pub language: Language,
    /// The `run` command line, with `{main}` and `{memory_mb}` placeholders.
    pub run: String,
    pub stub_template: &'static str,
    pub harness_template: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LanguageToml {
    id: String,
    extension: String,
    compile: String,
    run: String,
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
pub const SUPPORTED: [Language; 2] = [Language::Python, Language::TypeScript];

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
        _ => return Err(JudgeError::Unsupported(lang)),
    };
    let t: LanguageToml = toml::from_str(toml_src)
        .map_err(|e| JudgeError::Internal(format!("languages/{lang}/language.toml: {e}")))?;
    if t.id != lang.id() || t.extension != lang.extension() || !t.compile.is_empty() {
        return Err(JudgeError::Internal(format!(
            "languages/{lang}/language.toml does not match the {lang} definition"
        )));
    }
    Ok(LanguageDef {
        language: lang,
        run: t.run,
        stub_template,
        harness_template,
    })
}

impl LanguageDef {
    /// The harness entry file (`main.py`, `main.ts`), relative to the work folder.
    pub fn main_file(&self) -> String {
        format!("main.{}", self.language.extension())
    }

    /// The user's solution file (`solution.py`, `solution.ts`), relative to the work folder.
    pub fn solution_file(&self) -> String {
        format!("solution.{}", self.language.extension())
    }

    /// Program (resolved inside `toolchain`) and arguments for one batch run.
    pub fn command(
        &self,
        toolchain: &Path,
        memory_mb: u32,
    ) -> Result<(PathBuf, Vec<String>), JudgeError> {
        let main = self.main_file();
        let mut words = self.run.split_whitespace().map(|w| {
            w.replace("{main}", &main)
                .replace("{memory_mb}", &memory_mb.to_string())
        });
        let name = words.next().ok_or_else(|| {
            JudgeError::Internal(format!("empty run command for {}", self.language))
        })?;
        Ok((resolve_program(toolchain, &name)?, words.collect()))
    }
}

/// `python` -> `<toolchain>/python.exe` on Windows, `<toolchain>/bin/python` (or
/// `bin/python3`) on Linux.
fn resolve_program(toolchain: &Path, name: &str) -> Result<PathBuf, JudgeError> {
    let candidates: Vec<PathBuf> = if cfg!(windows) {
        vec![toolchain.join(format!("{name}.exe"))]
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
            assert!(d.run.contains("{main}"), "{lang}");
        }
        assert!(matches!(
            definition(Language::Rust),
            Err(JudgeError::Unsupported(Language::Rust))
        ));
    }

    #[test]
    fn command_substitutes_placeholders() {
        let dir = std::env::temp_dir().join(format!("kbj-cmd-{}", std::process::id()));
        let exe = if cfg!(windows) {
            dir.join("node.exe")
        } else {
            dir.join("bin/node")
        };
        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::write(&exe, "").unwrap();
        let (program, args) = definition(Language::TypeScript)
            .unwrap()
            .command(&dir, 256)
            .unwrap();
        assert_eq!(program, exe);
        assert!(args.contains(&"--max-old-space-size=256".to_string()));
        assert_eq!(args.last().unwrap(), "main.ts");
        let missing = definition(Language::Python).unwrap().command(&dir, 256);
        assert!(matches!(missing, Err(JudgeError::Toolchain(_))));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
