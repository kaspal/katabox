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
    /// The package folder for [`Problem::load`]; empty for [`Problem::from_parts`]
    /// (content.db). Nothing in the judge reads it.
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
        let mut tests = load_tests(&dir.join("tests/public"), TestKind::Public, &signature)?;
        tests.extend(load_tests(
            &dir.join("tests/hidden"),
            TestKind::Hidden,
            &signature,
        )?);
        Problem::new(
            meta.id,
            dir.to_path_buf(),
            signature,
            &meta.judge.checker,
            meta.judge.time_limit_ms,
            meta.judge.memory_mb,
            tests,
        )
        .map_err(bad)
    }

    /// Build a problem from stored parts (kb-store's `JudgeData`), with the same checks
    /// as [`Problem::load`]. `signature` is the JSON form of `[signature]`.
    pub fn from_parts(
        id: &str,
        signature: &Value,
        checker: &str,
        time_limit_ms: u32,
        memory_mb: u32,
        tests: Vec<TestCase>,
    ) -> Result<Problem, JudgeError> {
        let bad = |msg: String| JudgeError::Problem(format!("{id}: {msg}"));
        let signature = Signature::from_json(signature).map_err(bad)?;
        for t in &tests {
            check_input(&t.input, &signature)
                .map_err(|e| bad(format!("{:?} test {}: {e}", t.kind, t.index)))?;
        }
        Problem::new(
            id.to_string(),
            PathBuf::new(),
            signature,
            checker,
            time_limit_ms,
            memory_mb,
            tests,
        )
        .map_err(bad)
    }

    fn new(
        id: String,
        dir: PathBuf,
        signature: Signature,
        checker: &str,
        time_limit_ms: u32,
        memory_mb: u32,
        tests: Vec<TestCase>,
    ) -> Result<Problem, String> {
        let checker: Checker = checker.parse()?;
        if checker == Checker::Custom {
            return Err("custom checkers are not supported by the judge yet".into());
        }
        if time_limit_ms == 0 || memory_mb == 0 {
            return Err("judge limits must be positive".into());
        }
        if tests.is_empty() {
            return Err("no tests".into());
        }
        Ok(Problem {
            id,
            dir,
            signature,
            checker,
            time_limit_ms,
            memory_mb,
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
        check_input(&t.input, sig).map_err(bad)?;
        tests.push(TestCase {
            kind,
            index: u32::try_from(i + 1).unwrap_or(u32::MAX),
            input: t.input,
            expected: t.output,
        });
    }
    Ok(tests)
}

/// A test input must hold exactly the signature's params, by name.
pub fn check_input(input: &Map<String, Value>, sig: &Signature) -> Result<(), String> {
    for p in &sig.params {
        if !input.contains_key(&p.name) {
            return Err(format!("input lacks param {:?}", p.name));
        }
    }
    if let Some(extra) = input
        .keys()
        .find(|k| sig.params.iter().all(|p| &p.name != *k))
    {
        return Err(format!("input has unknown key {extra:?}"));
    }
    Ok(())
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

    #[test]
    fn from_parts_matches_load() {
        let disk = Problem::load(&two_sum_dir()).unwrap();
        let sig = serde_json::json!({
            "name": "twoSum",
            "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
            "returns": "int[]"
        });
        let p = Problem::from_parts("two-sum", &sig, "unordered", 2000, 256, disk.tests.clone())
            .unwrap();
        assert_eq!(p.signature, disk.signature);
        assert_eq!(p.checker, disk.checker);
        assert_eq!(p.tests, disk.tests);
        assert_eq!((p.time_limit_ms, p.memory_mb), (2000, 256));
        assert_eq!(p.dir, PathBuf::new());

        let err = |r: Result<Problem, JudgeError>| r.unwrap_err().to_string();
        let t = disk.tests.clone();
        assert!(err(Problem::from_parts("x", &sig, "custom", 1, 1, t.clone())).contains("custom"));
        assert!(err(Problem::from_parts("x", &sig, "exact", 0, 1, t.clone())).contains("positive"));
        assert!(err(Problem::from_parts("x", &sig, "exact", 1, 1, vec![])).contains("no tests"));
        assert!(
            err(Problem::from_parts(
                "x",
                &serde_json::json!({"name": 1}),
                "exact",
                1,
                1,
                t.clone()
            ))
            .contains("signature")
        );
        let mut bad = t[0].clone();
        bad.input.remove("target");
        assert!(err(Problem::from_parts("x", &sig, "exact", 1, 1, vec![bad])).contains("target"));
    }
}
