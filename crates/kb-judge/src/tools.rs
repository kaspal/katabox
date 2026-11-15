//! Problem tooling run in the sandbox like user code (ADR 0016): an author's `gen.py`
//! (random inputs) and `validate.py` (constraint checks) never run on the host, so
//! community-contributed problems can be checked safely.
//!
//! The author's script is imported as a module by a fixed driver (`main.py`) that reads a
//! JSON request on stdin and prints one JSON reply. `brute.py` and `naive.py` are ordinary
//! solutions and go through [`crate::judge`].

use std::time::Duration;

use kb_core::{Language, Limits};
use kb_runner::{SandboxSpec, Termination};
use serde_json::{Map, Value, json};

use crate::JudgeError;
use crate::judge::{JudgeEnv, WorkDir};
use crate::languages::definition;

/// Limits for one tooling run.
#[derive(Debug, Clone, Copy)]
pub struct ScriptLimits {
    pub cpu_time_ms: u32,
    pub memory_mb: u32,
    /// stdout + stderr; generated max-size inputs are large.
    pub max_output_bytes: usize,
}

impl Default for ScriptLimits {
    fn default() -> Self {
        ScriptLimits {
            cpu_time_ms: 30_000,
            memory_mb: 1024,
            max_output_bytes: 64 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptOutput {
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub exit_code: u32,
    pub termination: Termination,
    /// CPU time of the run (user + kernel), in microseconds.
    pub cpu_us: u64,
}

impl ScriptOutput {
    pub fn succeeded(&self) -> bool {
        self.termination == Termination::Exited && self.exit_code == 0
    }
}

/// Run `main` (the driver) with `files` beside it, in a fresh work folder of the sandbox
/// root, with only the Python toolchain readable. Blocks until it finishes.
pub fn run_python(
    env: &JudgeEnv,
    main: &str,
    files: &[(&str, &str)],
    stdin: &[u8],
    limits: ScriptLimits,
) -> Result<ScriptOutput, JudgeError> {
    run_script(env, Language::Python, main, files, stdin, limits)
}

/// [`run_python`] for any interpreted language the judge runs: Python, or TypeScript
/// (Node, which also runs plain JavaScript written as TypeScript). Only that language's
/// toolchain is readable.
pub fn run_script(
    env: &JudgeEnv,
    language: Language,
    main: &str,
    files: &[(&str, &str)],
    stdin: &[u8],
    limits: ScriptLimits,
) -> Result<ScriptOutput, JudgeError> {
    let def = definition(language)?;
    if def.compile_limits.is_some() {
        return Err(JudgeError::Unsupported(language));
    }
    let toolchain = env.toolchain(language)?;
    let (program, args) = def.command(toolchain, limits.memory_mb, None)?;
    let work = WorkDir::create(&env.runs_dir)?;
    std::fs::write(work.path.join(def.main_file()), main)?;
    if language == Language::TypeScript {
        // As for judged code (codegen): `.ts` files are ES modules, and Node finds this
        // package.json instead of looking in the folders above.
        std::fs::write(work.path.join("package.json"), "{\"type\": \"module\"}\n")?;
    }
    for (name, contents) in files {
        if name.contains(['/', '\\']) || *name == def.main_file() {
            return Err(JudgeError::Internal(format!(
                "invalid script file name {name}"
            )));
        }
        std::fs::write(work.path.join(name), contents)?;
    }
    let mut run_limits = Limits::default_for(language);
    run_limits.cpu_time_ms = limits.cpu_time_ms;
    run_limits.memory_mb = limits.memory_mb;
    let mut spec = SandboxSpec::new(&env.sandbox_root, program, &work.path, &run_limits);
    spec.args = args;
    spec.read_only_dirs = vec![toolchain.to_path_buf()];
    spec.stdin = stdin.to_vec();
    spec.max_output_bytes = limits.max_output_bytes;
    spec.wall_time =
        Duration::from_millis(u64::from(limits.cpu_time_ms) * u64::from(env.wall_factor.max(1)));
    let outcome = crate::sandbox::run(&spec)?;
    Ok(ScriptOutput {
        stdout: outcome.stdout,
        stderr: String::from_utf8_lossy(&outcome.stderr).into_owned(),
        exit_code: outcome.exit_code,
        termination: outcome.termination,
        cpu_us: u64::try_from((outcome.user_time + outcome.kernel_time).as_micros())
            .unwrap_or(u64::MAX),
    })
}

/// Imports the author's script from this folder (Python runs with `-I`, so the script's
/// folder is not on `sys.path`) and answers one JSON request.
const DRIVER_PRELUDE: &str = r#"import importlib.util, json, os, sys

def load(name):
    here = os.path.dirname(os.path.abspath(__file__))
    spec = importlib.util.spec_from_file_location(name, os.path.join(here, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

request = json.loads(sys.stdin.read())

def as_input(inp, call):
    # A program problem's generator returns the stdin text (ADR 0028).
    if isinstance(inp, str):
        return {"stdin": inp}
    if not isinstance(inp, dict):
        raise TypeError(f"{call} returned {type(inp).__name__}, not a dict (or the stdin text)")
    return inp
"#;

const GEN_DRIVER: &str = r#"gen = load("gen")
out = []
for seed in request["seeds"]:
    out.append(as_input(gen.generate(seed, request["size"]), f"generate({seed}, ...)"))
sys.stdout.write(json.dumps(out, allow_nan=False))
sys.stdout.flush()
"#;

const SIZED_DRIVER: &str = r#"gen = load("gen")
out = []
for n in request["sizes"]:
    out.append(as_input(gen.generate(request["seed"], n), f"generate(seed, {n})"))
sys.stdout.write(json.dumps(out, allow_nan=False))
sys.stdout.flush()
"#;

const VALIDATE_DRIVER: &str = r#"validate = load("validate")
out = []
for inp in request["inputs"]:
    # A program problem's validator gets the stdin text (ADR 0028).
    if list(inp) == ["stdin"] and isinstance(inp["stdin"], str):
        inp = inp["stdin"]
    msg = validate.validate(inp)
    if msg is not None and not isinstance(msg, str):
        raise TypeError(f"validate(...) returned {type(msg).__name__}, not str or None")
    out.append(msg)
sys.stdout.write(json.dumps(out))
sys.stdout.flush()
"#;

// SQL problems (ADR 0031): `brute.py`'s `solve(tables)` returns the expected result
// computed in plain Python, `{"columns": [...], "rows": [[...]]}`.
const BRUTE_ROWS_DRIVER: &str = r#"brute = load("brute")
out = []
for inp in request["inputs"]:
    r = brute.solve(inp["tables"])
    if not isinstance(r, dict) or set(r) != {"columns", "rows"}:
        raise TypeError("solve(tables) must return {\"columns\": [...], \"rows\": [...]}")
    out.append({"columns": list(r["columns"]), "rows": [list(row) for row in r["rows"]]})
sys.stdout.write(json.dumps(out, allow_nan=False))
sys.stdout.flush()
"#;

// An author's query (`ref.sql`, `wrong.sql`) on many inputs in one run, with plain
// sqlite3 (still in the sandbox, read-only). User queries never come here: they go
// through the locked-down harness (`crate::sql::judge_sql`).
const SQL_ROWS_DRIVER: &str = r#"import sqlite3
query = request["query"].strip()
while query.endswith(";"):
    query = query[:-1].rstrip()
out = []
for inp in request["inputs"]:
    conn = sqlite3.connect(":memory:", isolation_level=None)
    conn.executescript(request["schema"])
    for name, rows in inp["tables"].items():
        if rows:
            marks = ", ".join("?" * len(rows[0]))
            conn.executemany(f'INSERT INTO "{name}" VALUES ({marks})', rows)
    conn.execute("PRAGMA query_only = ON")
    try:
        cursor = conn.execute(query)
        columns = [d[0] for d in cursor.description or []]
        out.append({"columns": columns, "rows": [list(row) for row in cursor.fetchall()]})
    except sqlite3.Error as e:
        out.append({"error": str(e)})
    conn.close()
sys.stdout.write(json.dumps(out, allow_nan=False))
sys.stdout.flush()
"#;

fn driver(body: &str) -> String {
    format!("{DRIVER_PRELUDE}{body}")
}

/// Why a script failed: its own error (last lines of the traceback) or the sandbox's.
fn failure(what: &str, out: &ScriptOutput) -> JudgeError {
    let tail: Vec<&str> = out.stderr.lines().rev().take(6).collect();
    let tail: Vec<&str> = tail.into_iter().rev().collect();
    let why = match out.termination {
        Termination::CpuTimeLimit | Termination::WallTimeLimit => "ran out of time".to_string(),
        Termination::OutputLimit => "printed too much".to_string(),
        _ => format!("exited with code {}", out.exit_code),
    };
    JudgeError::Problem(format!("{what} {why}\n{}", tail.join("\n")))
}

/// Run `gen.py`'s `generate(seed, size)` for each seed, in one sandboxed run.
pub fn generate_inputs(
    env: &JudgeEnv,
    gen_src: &str,
    seeds: &[u64],
    size: &str,
    limits: ScriptLimits,
) -> Result<Vec<Map<String, Value>>, JudgeError> {
    let request = json!({ "seeds": seeds, "size": size }).to_string();
    let out = run_python(
        env,
        &driver(GEN_DRIVER),
        &[("gen.py", gen_src)],
        request.as_bytes(),
        limits,
    )?;
    if !out.succeeded() {
        return Err(failure("gen.py", &out));
    }
    let values: Vec<Value> = serde_json::from_slice(&out.stdout)
        .map_err(|e| JudgeError::Problem(format!("gen.py: unreadable output: {e}")))?;
    if values.len() != seeds.len() {
        return Err(JudgeError::Problem("gen.py: wrong number of inputs".into()));
    }
    values
        .into_iter()
        .map(|v| match v {
            Value::Object(m) => Ok(m),
            _ => Err(JudgeError::Problem(
                "gen.py: an input is not an object".into(),
            )),
        })
        .collect()
}

/// Run `gen.py`'s `generate(seed, n)` for each profile size n (ADR 0024: an int size asks
/// for an input whose driving dimension is n), in one sandboxed run.
pub fn generate_sized(
    env: &JudgeEnv,
    gen_src: &str,
    seed: u64,
    sizes: &[u32],
    limits: ScriptLimits,
) -> Result<Vec<Map<String, Value>>, JudgeError> {
    let request = json!({ "seed": seed, "sizes": sizes }).to_string();
    let out = run_python(
        env,
        &driver(SIZED_DRIVER),
        &[("gen.py", gen_src)],
        request.as_bytes(),
        limits,
    )?;
    if !out.succeeded() {
        return Err(failure("gen.py", &out));
    }
    let values: Vec<Value> = serde_json::from_slice(&out.stdout)
        .map_err(|e| JudgeError::Problem(format!("gen.py: unreadable output: {e}")))?;
    if values.len() != sizes.len() {
        return Err(JudgeError::Problem("gen.py: wrong number of inputs".into()));
    }
    values
        .into_iter()
        .map(|v| match v {
            Value::Object(m) => Ok(m),
            _ => Err(JudgeError::Problem(
                "gen.py: an input is not an object".into(),
            )),
        })
        .collect()
}

/// Run `validate.py`'s `validate(input)` on each input, in one sandboxed run: `None` for
/// a valid input, else the broken constraint.
pub fn validate_inputs(
    env: &JudgeEnv,
    validate_src: &str,
    inputs: &[&Map<String, Value>],
    limits: ScriptLimits,
) -> Result<Vec<Option<String>>, JudgeError> {
    let request = json!({ "inputs": inputs }).to_string();
    let out = run_python(
        env,
        &driver(VALIDATE_DRIVER),
        &[("validate.py", validate_src)],
        request.as_bytes(),
        limits,
    )?;
    if !out.succeeded() {
        return Err(failure("validate.py", &out));
    }
    let values: Vec<Option<String>> = serde_json::from_slice(&out.stdout)
        .map_err(|e| JudgeError::Problem(format!("validate.py: unreadable output: {e}")))?;
    if values.len() != inputs.len() {
        return Err(JudgeError::Problem(
            "validate.py: wrong number of answers".into(),
        ));
    }
    Ok(values)
}

/// Run `brute.py`'s `solve(tables)` on each SQL input, in one sandboxed run: the expected
/// results, each `{"columns": [...], "rows": [[...]]}` (ADR 0031).
pub fn brute_rows(
    env: &JudgeEnv,
    brute_src: &str,
    inputs: &[&Map<String, Value>],
    limits: ScriptLimits,
) -> Result<Vec<Value>, JudgeError> {
    let request = json!({ "inputs": inputs }).to_string();
    let out = run_python(
        env,
        &driver(BRUTE_ROWS_DRIVER),
        &[("brute.py", brute_src)],
        request.as_bytes(),
        limits,
    )?;
    if !out.succeeded() {
        return Err(failure("brute.py", &out));
    }
    let values: Vec<Value> = serde_json::from_slice(&out.stdout)
        .map_err(|e| JudgeError::Problem(format!("brute.py: unreadable output: {e}")))?;
    if values.len() != inputs.len() {
        return Err(JudgeError::Problem(
            "brute.py: wrong number of results".into(),
        ));
    }
    for (i, v) in values.iter().enumerate() {
        crate::sql::check_result_shape(v)
            .map_err(|e| JudgeError::Problem(format!("brute.py, input {}: {e}", i + 1)))?;
    }
    Ok(values)
}

/// An author's query (`ref.sql`) on each SQL input, in one sandboxed run of plain sqlite3
/// (read-only): each input's result, or the query's error on it. For package tooling
/// only; user queries are judged by [`crate::sql::judge_sql`].
pub fn sql_rows(
    env: &JudgeEnv,
    schema_sql: &str,
    query: &str,
    inputs: &[&Map<String, Value>],
    limits: ScriptLimits,
) -> Result<Vec<Result<Value, String>>, JudgeError> {
    let request = json!({ "schema": schema_sql, "query": query, "inputs": inputs }).to_string();
    let out = run_python(
        env,
        &driver(SQL_ROWS_DRIVER),
        &[],
        request.as_bytes(),
        limits,
    )?;
    if !out.succeeded() {
        return Err(failure("the query", &out));
    }
    let values: Vec<Value> = serde_json::from_slice(&out.stdout)
        .map_err(|e| JudgeError::Problem(format!("the query: unreadable output: {e}")))?;
    if values.len() != inputs.len() {
        return Err(JudgeError::Problem(
            "the query: wrong number of results".into(),
        ));
    }
    Ok(values
        .into_iter()
        .map(|v| match v.get("error").and_then(Value::as_str) {
            Some(e) => Err(e.to_string()),
            None => Ok(v),
        })
        .collect())
}
