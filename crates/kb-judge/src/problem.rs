//! Loading what the judge needs from a problem package: the signature, the checker,
//! the limits and the test cases. kb-pack validates the rest of the package.

use std::path::{Path, PathBuf};

use kb_core::{TestCase, TestKind};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::JudgeError;
use crate::checker::Checker;
use crate::signature::{RawSignature, Signature};

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: String,
    pub dir: PathBuf,
    pub signature: Signature,
    pub checker: Checker,
    /// CPU time for the user's code across the whole batch of tests.
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    /// Public tests first, then hidden, each in file-name order (numeric when possible).
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
struct ProblemToml {
    id: String,
    signature: RawSignature,
    judge: JudgeToml,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JudgeToml {
    checker: String,
    time_limit_ms: u32,
    memory_mb: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TestFile {
    input: Map<String, Value>,
    output: Value,
}

impl Problem {
    /// Load `dir/problem.toml` and `dir/tests/{public,hidden}/*.json`.
    pub fn load(dir: &Path) -> Result<Problem, JudgeError> {
        let bad = |msg: String| JudgeError::Problem(format!("{}: {msg}", dir.display()));
        let src = std::fs::read_to_string(dir.join("problem.toml"))
            .map_err(|e| bad(format!("problem.toml: {e}")))?;
        let meta: ProblemToml =
            toml::from_str(&src).map_err(|e| bad(format!("problem.toml: {e}")))?;
        let signature: Signature = meta.signature.try_into().map_err(bad)?;
        let checker: Checker = meta.judge.checker.parse().map_err(bad)?;
        if checker == Checker::Custom {
            return Err(bad(
                "custom checkers are not supported by the judge yet".into()
            ));
        }
        if meta.judge.time_limit_ms == 0 || meta.judge.memory_mb == 0 {
            return Err(bad("judge limits must be positive".into()));
        }

        let mut tests = load_tests(&dir.join("tests/public"), TestKind::Public, &signature)?;
        tests.extend(load_tests(
            &dir.join("tests/hidden"),
            TestKind::Hidden,
            &signature,
        )?);
        if tests.is_empty() {
            return Err(bad("no tests".into()));
        }
        Ok(Problem {
            id: meta.id,
            dir: dir.to_path_buf(),
            signature,
            checker,
            time_limit_ms: meta.judge.time_limit_ms,
            memory_mb: meta.judge.memory_mb,
            tests,
        })
    }

    pub fn public_tests(&self) -> impl Iterator<Item = &TestCase> {
        self.tests.iter().filter(|t| t.kind == TestKind::Public)
    }
}

fn load_tests(dir: &Path, kind: TestKind, sig: &Signature) -> Result<Vec<TestCase>, JudgeError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(JudgeError::Problem(format!("{}: {e}", dir.display()))),
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort_by_key(|p| {
        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        (stem.parse::<u64>().unwrap_or(u64::MAX), stem.to_string())
    });

    let mut tests = Vec::with_capacity(files.len());
    for (i, path) in files.iter().enumerate() {
        let bad = |msg: String| JudgeError::Problem(format!("{}: {msg}", path.display()));
        let src = std::fs::read_to_string(path).map_err(|e| bad(e.to_string()))?;
        let t: TestFile = serde_json::from_str(&src).map_err(|e| bad(e.to_string()))?;
        for p in &sig.params {
            if !t.input.contains_key(&p.name) {
                return Err(bad(format!("input lacks param {:?}", p.name)));
            }
        }
        if let Some(extra) = t
            .input
            .keys()
            .find(|k| sig.params.iter().all(|p| &p.name != *k))
        {
            return Err(bad(format!("input has unknown key {extra:?}")));
        }
        tests.push(TestCase {
            kind,
            index: u32::try_from(i + 1).unwrap_or(u32::MAX),
            input: t.input,
            expected: t.output,
        });
    }
    Ok(tests)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_sum_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum")
    }

    #[test]
    fn loads_two_sum() {
        let p = Problem::load(&two_sum_dir()).unwrap();
        assert_eq!(p.id, "two-sum");
        assert_eq!(p.signature.name, "twoSum");
        assert_eq!(p.checker, Checker::Unordered);
        assert_eq!(p.time_limit_ms, 2000);
        assert_eq!(p.public_tests().count(), 3);
        assert_eq!(p.tests.len(), 7);
        assert_eq!(p.tests[0].kind, TestKind::Public);
        assert_eq!(p.tests[0].index, 1);
        assert_eq!(p.tests[3].kind, TestKind::Hidden);
        assert_eq!(p.tests[3].index, 1);
        assert!(p.tests[0].input.contains_key("nums"));
    }
}
