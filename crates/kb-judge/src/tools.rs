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
    let def = definition(Language::Python)?;
    let toolchain = env.toolchain(Language::Python)?;
    let (program, args) = def.command(toolchain, limits.memory_mb, None)?;
    let work = WorkDir::create(&env.runs_dir)?;
    std::fs::write(work.path.join(def.main_file()), main)?;
    for (name, contents) in files {
        if name.contains(['/', '\\']) || *name == def.main_file() {
            return Err(JudgeError::Internal(format!(
                "invalid script file name {name}"
            )));
        }
        std::fs::write(work.path.join(name), contents)?;
    }
    let mut run_limits = Limits::default_for(Language::Python);
    run_limits.cpu_time_ms = limits.cpu_time_ms;
    run_limits.memory_mb = limits.memory_mb;
    let mut spec = SandboxSpec::new(&env.sandbox_root, program, &work.path, &run_limits);
    spec.args = args;
    spec.read_only_dirs = vec![toolchain.to_path_buf()];
    spec.stdin = stdin.to_vec();
    spec.max_output_bytes = limits.max_output_bytes;
    spec.wall_time = Duration::from_millis(u64::from(limits.cpu_time_ms) * 3);
    let outcome = kb_runner::run(&spec)?;
    Ok(ScriptOutput {
        stdout: outcome.stdout,
        stderr: String::from_utf8_lossy(&outcome.stderr).into_owned(),
        exit_code: outcome.exit_code,
        termination: outcome.termination,
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
"#;

const GEN_DRIVER: &str = r#"gen = load("gen")
out = []
for seed in request["seeds"]:
    inp = gen.generate(seed, request["size"])
    if not isinstance(inp, dict):
        raise TypeError(f"generate({seed}, ...) returned {type(inp).__name__}, not a dict")
    out.append(inp)
sys.stdout.write(json.dumps(out, allow_nan=False))
sys.stdout.flush()
"#;

const VALIDATE_DRIVER: &str = r#"validate = load("validate")
out = []
for inp in request["inputs"]:
    msg = validate.validate(inp)
    if msg is not None and not isinstance(msg, str):
        raise TypeError(f"validate(...) returned {type(msg).__name__}, not str or None")
    out.append(msg)
sys.stdout.write(json.dumps(out))
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
