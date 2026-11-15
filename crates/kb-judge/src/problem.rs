//! Loading what the judge needs from a problem package: the signature, the checker,
//! the limits and the test cases. kb-pack validates the rest of the package.

use std::path::{Path, PathBuf};

pub use kb_core::ProblemMode;
use kb_core::{TestCase, TestKind};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::JudgeError;
use crate::checker::Checker;
use crate::signature::{RawSignature, Signature};

/// The parameter a program problem's test input holds: the whole stdin text.
pub const STDIN_PARAM: &str = "stdin";

/// The signature every program problem has inside the judge, `main(stdin: string) ->
/// string`: a test's input is `{"stdin": text}` and its expected value the stdout text,
/// so tests, custom tests and failures need no second shape (ADR 0028).
pub fn program_signature() -> Signature {
    Signature::from_json(&serde_json::json!({
        "name": "main",
        "params": [{"name": STDIN_PARAM, "type": "string"}],
        "returns": "string",
    }))
    .expect("the program signature is valid")
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: String,
    /// The package folder for [`Problem::load`]; empty for [`Problem::from_parts`]
    /// (content.db). Nothing in the judge reads it.
    pub dir: PathBuf,
    pub mode: ProblemMode,
    /// For a program problem, always [`program_signature`].
    pub signature: Signature,
    pub checker: Checker,
    /// Base CPU time for the user's code across the whole batch of tests (a function
    /// problem) or for each test's run (a program problem, ADR 0028), set for Rust; each
    /// language scales it and adds its startup allowance ([`crate::limits`], ADR 0017).
    pub time_limit_ms: u32,
    pub memory_mb: u32,
    /// Public tests first, then hidden, each in file-name order (numeric when possible).
    pub tests: Vec<TestCase>,
    /// An SQL problem's schema (ADR 0031); `None` for the other modes.
    pub sql: Option<crate::sql::SqlSchema>,
}

#[derive(Debug, Deserialize)]
struct ProblemToml {
    id: String,
    #[serde(default)]
    mode: Option<String>,
    signature: Option<RawSignature>,
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

/// A program problem's test file: the stdin text and the expected stdout text.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProgramTestFile {
    stdin: String,
    stdout: String,
}

impl Problem {
    /// Load `dir/problem.toml` and `dir/tests/{public,hidden}/*.json`.
    pub fn load(dir: &Path) -> Result<Problem, JudgeError> {
        let bad = |msg: String| JudgeError::Problem(format!("{}: {msg}", dir.display()));
        let src = std::fs::read_to_string(dir.join("problem.toml"))
            .map_err(|e| bad(format!("problem.toml: {e}")))?;
        let meta: ProblemToml =
            toml::from_str(&src).map_err(|e| bad(format!("problem.toml: {e}")))?;
        let mode: ProblemMode = meta
            .mode
            .as_deref()
            .unwrap_or("function")
            .parse()
            .map_err(bad)?;
        let signature: Signature = match (mode, meta.signature) {
            (ProblemMode::Function, Some(raw)) => raw.try_into().map_err(bad)?,
            (ProblemMode::Function, None) => {
                return Err(bad("problem.toml: missing [signature]".into()));
            }
            (ProblemMode::Program | ProblemMode::Sql, None) => program_signature(),
            (ProblemMode::Program, Some(_)) => {
                return Err(bad(
                    "problem.toml: a program problem has no [signature] (its program reads stdin)"
                        .into(),
                ));
            }
            (ProblemMode::Sql, Some(_)) => {
                return Err(bad(
                    "problem.toml: an SQL problem has no [signature] (its query reads tables)"
                        .into(),
                ));
            }
        };
        let sql = if mode == ProblemMode::Sql {
            let text = std::fs::read_to_string(dir.join("schema.sql"))
                .map_err(|e| bad(format!("schema.sql: {e}")))?;
            Some(crate::sql::parse_schema(&text).map_err(|e| bad(format!("schema.sql: {e}")))?)
        } else {
            None
        };
        let mut tests = load_tests(
            &dir.join("tests/public"),
            TestKind::Public,
            mode,
            &signature,
            sql.as_ref(),
        )?;
        tests.extend(load_tests(
            &dir.join("tests/hidden"),
            TestKind::Hidden,
            mode,
            &signature,
            sql.as_ref(),
        )?);
        let mut problem = Problem::new(
            meta.id,
            dir.to_path_buf(),
            mode,
            signature,
            &meta.judge.checker,
            meta.judge.time_limit_ms,
            meta.judge.memory_mb,
            tests,
        )
        .map_err(bad)?;
        problem.sql = sql;
        Ok(problem)
    }

    /// Build a problem from stored parts (kb-store's `JudgeData`), with the same checks
    /// as [`Problem::load`]. `signature` is the JSON form of `[signature]`; a program
    /// problem's is [`program_signature`] whatever is passed.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: &str,
        mode: ProblemMode,
        signature: &Value,
        checker: &str,
        time_limit_ms: u32,
        memory_mb: u32,
        tests: Vec<TestCase>,
    ) -> Result<Problem, JudgeError> {
        let bad = |msg: String| JudgeError::Problem(format!("{id}: {msg}"));
        let signature = match mode {
            ProblemMode::Function => Signature::from_json(signature).map_err(bad)?,
            ProblemMode::Program => program_signature(),
            ProblemMode::Sql => {
                return Err(bad(
                    "SQL problems are judged from their package until the app supports them (ADR 0031)"
                        .into(),
                ));
            }
        };
        for t in &tests {
            check_input(&t.input, &signature)
                .map_err(|e| bad(format!("{:?} test {}: {e}", t.kind, t.index)))?;
            if mode == ProblemMode::Program && !t.expected.is_string() {
                return Err(bad(format!(
                    "{:?} test {}: expected output is not text",
                    t.kind, t.index
                )));
            }
        }
        Problem::new(
            id.to_string(),
            PathBuf::new(),
            mode,
            signature,
            checker,
            time_limit_ms,
            memory_mb,
            tests,
        )
        .map_err(bad)
    }

    /// Build an SQL problem (ADR 0031) from stored parts: its `schema.sql` text, checker,
    /// limits and tests, with the same checks as [`Problem::load`].
    pub fn from_sql_parts(
        id: &str,
        schema_sql: &str,
        checker: &str,
        time_limit_ms: u32,
        memory_mb: u32,
        tests: Vec<TestCase>,
    ) -> Result<Problem, JudgeError> {
        let bad = |msg: String| JudgeError::Problem(format!("{id}: {msg}"));
        let schema =
            crate::sql::parse_schema(schema_sql).map_err(|e| bad(format!("schema.sql: {e}")))?;
        for t in &tests {
            crate::sql::check_tables(&schema, &t.input)
                .map_err(|e| bad(format!("{:?} test {}: {e}", t.kind, t.index)))?;
            crate::sql::check_result_shape(&t.expected)
                .map_err(|e| bad(format!("{:?} test {}: output: {e}", t.kind, t.index)))?;
        }
        let mut problem = Problem::new(
            id.to_string(),
            PathBuf::new(),
            ProblemMode::Sql,
            program_signature(),
            checker,
            time_limit_ms,
            memory_mb,
            tests,
        )
        .map_err(bad)?;
        problem.sql = Some(schema);
        Ok(problem)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        id: String,
        dir: PathBuf,
        mode: ProblemMode,
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
        // Text checkers compare stdout text; rows checkers SQL results; value checkers
        // compare returned JSON.
        match (mode, checker.is_text(), checker.is_rows()) {
            (ProblemMode::Program, false, _) => {
                return Err(
                    "a program problem needs a text checker (tokens | tokens-float:<eps> | lines)"
                        .into(),
                );
            }
            (ProblemMode::Function, true, _) => {
                return Err("text checkers are for program problems (mode = \"program\")".into());
            }
            (ProblemMode::Sql, _, false) => {
                return Err(
                    "an SQL problem needs a rows checker (rows-ordered | rows-unordered, \
                     optionally :<eps>)"
                        .into(),
                );
            }
            (ProblemMode::Function | ProblemMode::Program, _, true) => {
                return Err("rows checkers are for SQL problems (mode = \"sql\")".into());
            }
            _ => {}
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
            mode,
            signature,
            checker,
            time_limit_ms,
            memory_mb,
            tests,
            sql: None,
        })
    }

    pub fn public_tests(&self) -> impl Iterator<Item = &TestCase> {
        self.tests.iter().filter(|t| t.kind == TestKind::Public)
    }
}

fn load_tests(
    dir: &Path,
    kind: TestKind,
    mode: ProblemMode,
    sig: &Signature,
    sql: Option<&crate::sql::SqlSchema>,
) -> Result<Vec<TestCase>, JudgeError> {
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
        let (input, expected) = match mode {
            ProblemMode::Sql => {
                let t: TestFile = serde_json::from_str(&src).map_err(|e| bad(e.to_string()))?;
                let schema = sql.expect("an SQL problem's schema is loaded first");
                crate::sql::check_tables(schema, &t.input).map_err(bad)?;
                crate::sql::check_result_shape(&t.output)
                    .map_err(|e| bad(format!("output: {e}")))?;
                tests.push(TestCase {
                    kind,
                    index: u32::try_from(i + 1).unwrap_or(u32::MAX),
                    input: t.input,
                    expected: t.output,
                });
                continue;
            }
            ProblemMode::Function => {
                let t: TestFile = serde_json::from_str(&src).map_err(|e| bad(e.to_string()))?;
                (t.input, t.output)
            }
            ProblemMode::Program => {
                let t: ProgramTestFile =
                    serde_json::from_str(&src).map_err(|e| bad(e.to_string()))?;
                let mut input = Map::new();
                input.insert(STDIN_PARAM.into(), Value::String(t.stdin));
                (input, Value::String(t.stdout))
            }
        };
        check_input(&input, sig).map_err(bad)?;
        tests.push(TestCase {
            kind,
            index: u32::try_from(i + 1).unwrap_or(u32::MAX),
            input,
            expected,
        });
    }
    Ok(tests)
}

/// A test input must hold exactly the signature's params, by name, each a value of its
/// type (lists and trees in their canonical JSON; ADR 0025).
pub fn check_input(input: &Map<String, Value>, sig: &Signature) -> Result<(), String> {
    for p in &sig.params {
        match input.get(&p.name) {
            None => return Err(format!("input lacks param {:?}", p.name)),
            Some(v) => {
                p.ty.check_value(v)
                    .map_err(|e| format!("{}: {e}", p.name))?
            }
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
        assert_eq!(p.time_limit_ms, 300);
        assert_eq!(p.public_tests().count(), 3);
        // Public first, then hidden: 3 public, and at least 15 hidden (edge, random, max).
        assert!(p.tests.len() >= 18, "{}", p.tests.len());
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
        let p = Problem::from_parts(
            "two-sum",
            ProblemMode::Function,
            &sig,
            "unordered",
            2000,
            256,
            disk.tests.clone(),
        )
        .unwrap();
        assert_eq!(p.signature, disk.signature);
        assert_eq!(p.checker, disk.checker);
        assert_eq!(p.tests, disk.tests);
        assert_eq!((p.time_limit_ms, p.memory_mb), (2000, 256));
        assert_eq!(p.dir, PathBuf::new());

        let err = |r: Result<Problem, JudgeError>| r.unwrap_err().to_string();
        let t = disk.tests.clone();
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
                &sig,
                "custom",
                1,
                1,
                t.clone()
            ))
            .contains("custom")
        );
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
                &sig,
                "exact",
                0,
                1,
                t.clone()
            ))
            .contains("positive")
        );
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
                &sig,
                "exact",
                1,
                1,
                vec![]
            ))
            .contains("no tests")
        );
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
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
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
                &sig,
                "exact",
                1,
                1,
                vec![bad]
            ))
            .contains("target")
        );
        // Text checkers belong to program problems.
        assert!(
            err(Problem::from_parts(
                "x",
                ProblemMode::Function,
                &sig,
                "tokens",
                1,
                1,
                t.clone()
            ))
            .contains("program")
        );
    }

    /// A program package in a temporary folder.
    fn program_dir(name: &str, toml_extra: &str, tests: &[&str]) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("kb-judge-program-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("tests/public")).unwrap();
        std::fs::write(
            dir.join("problem.toml"),
            format!(
                "id = \"{name}\"\nmode = \"program\"\n{toml_extra}\n[judge]\nchecker = \"tokens\"\ntime_limit_ms = 300\nmemory_mb = 256\n"
            ),
        )
        .unwrap();
        for (i, test) in tests.iter().enumerate() {
            std::fs::write(dir.join(format!("tests/public/{}.json", i + 1)), test).unwrap();
        }
        dir
    }

    #[test]
    fn loads_a_program_problem_with_its_synthetic_signature() {
        let dir = program_dir("sum", "", &[r#"{"stdin": "2\n3 4\n", "stdout": "7\n"}"#]);
        let p = Problem::load(&dir).unwrap();
        assert_eq!(p.mode, ProblemMode::Program);
        assert_eq!(p.signature, program_signature());
        assert_eq!(p.checker, Checker::Tokens);
        assert_eq!(p.tests[0].input["stdin"], "2\n3 4\n");
        assert_eq!(p.tests[0].expected, "7\n");
        // The same from stored parts, whatever signature JSON is passed.
        let q = Problem::from_parts(
            "sum",
            ProblemMode::Program,
            &Value::Null,
            "tokens",
            300,
            256,
            p.tests.clone(),
        )
        .unwrap();
        assert_eq!(q.signature, program_signature());
        assert!(
            Problem::from_parts(
                "sum",
                ProblemMode::Program,
                &Value::Null,
                "exact",
                300,
                256,
                p.tests.clone()
            )
            .unwrap_err()
            .to_string()
            .contains("text checker")
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn program_problems_refuse_a_signature_and_other_test_shapes() {
        let err = |dir: &Path| Problem::load(dir).unwrap_err().to_string();
        let dir = program_dir(
            "sig",
            "[signature]\nname = \"f\"\nparams = []\nreturns = \"int\"\n",
            &[r#"{"stdin": "", "stdout": ""}"#],
        );
        assert!(err(&dir).contains("no [signature]"), "{}", err(&dir));
        std::fs::remove_dir_all(&dir).unwrap();
        let dir = program_dir("shape", "", &[r#"{"input": {"stdin": ""}, "output": ""}"#]);
        assert!(err(&dir).contains("unknown field"), "{}", err(&dir));
        std::fs::remove_dir_all(&dir).unwrap();
        let dir = program_dir("extra", "", &[r#"{"stdin": "", "stdout": "", "note": 1}"#]);
        assert!(err(&dir).contains("unknown field"), "{}", err(&dir));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
