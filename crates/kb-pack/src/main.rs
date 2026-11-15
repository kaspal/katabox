//! KataBox problem packs.
//!
//! `kb-pack validate [DIR] [--run [--problems ID,ID | --from FILE]] [--learn [--topics ID,ID] [--subject ID]] [--root ROOT]` —
//! checks every problem folder under DIR and the Learn topics in content/learn (ADR 0029;
//! `--learn`: only the topics; `--topics` / `--subject` limit the sandbox runs to those
//! topics, and an unknown id is an error; `--problems` / `--from` (one id per line) limit
//! `--run` to those problems and skip the Learn samples)
//! (default `content/problems`) against the package format in the plan: structure,
//! metadata, test files, and that each reference solution matches its generated stub.
//! `--run` adds the sandbox checks (ADR 0016): `validate.py` accepts every test; every
//! reference, in every language, is Accepted within half the time limit; with `gen.py`
//! and `solutions/brute.py`, the references agree with brute on 500 seeded inputs; and
//! `solutions/naive.py` gets Time Limit Exceeded. ROOT is the sandbox root with the
//! toolchains (default `target/katabox`, created by `scripts/fetch-toolchains.ps1`).
//!
//! `kb-pack generate PROBLEM_DIR [--size max] --seeds 1,2 [--from N] [--root ROOT]` —
//! writes `tests/hidden/<N..>.json` from `gen.py` (outputs from `solutions/ref.py`), all
//! run in the sandbox. Generated tests are committed: the build never runs scripts.
//!
//! `kb-pack profile PROBLEM_DIR [--root ROOT]` — for a problem with `profile = true`,
//! writes the complexity profiler's reference (`profile/reference.json`: the hashes of
//! the inputs gen.py makes at each of `profile_sizes`, and the reference solutions'
//! times; committed) and the input cache (`profile/inputs/`, gitignored), all run in the
//! sandbox (ADR 0024).
//!
//! `kb-pack profile --inputs [DIR] [--root ROOT]` — regenerates the input cache of one
//! problem, or of every profiled problem under DIR (default `content/problems`), and
//! verifies it against the committed hashes. `kb-pack build` packs a profile only when
//! its cache is complete.
//!
//! `kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]` — validates as
//! above (without `--run`), then compiles every problem into content.db (default
//! `target/content.db`, pack `core`, this crate's version). Refuses to build if any
//! problem fails validation.
//!
//! `kb-pack new ID [--mode function|program|sql] [DIR]` — scaffolds `DIR/ID` (default
//! `content/problems`) with every file a package needs, each marked `TODO(kb-pack)` where
//! something must be written; validation refuses a file that still has the marker.
//!
//! `kb-pack new-topic SUBJECT ID [LEARN_DIR]` — the same for a Learn topic (default
//! `content/learn`); the tracer in `apps/desktop/src/learn/tracers` comes next.
//!
//! `kb-pack preview PROBLEM_DIR [--out DIR]` — renders the statements (EN and ES), hints
//! and editorial into `DIR/<id>.html` (default `target/preview`) to read them as a page.

use kb_pack::{BuildOptions, build, run_references, validate_all, validate_learn_only};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage:\n  kb-pack validate [DIR] [--run [--problems ID,ID | --from FILE]] [--learn [--topics ID,ID] [--subject ID]] [--root ROOT]\n  \
                     kb-pack generate PROBLEM_DIR [--size max|small] --seeds 1,2 [--from N] [--root ROOT]\n  \
                     kb-pack profile PROBLEM_DIR [--root ROOT]\n  \
                     kb-pack profile --inputs [DIR] [--root ROOT]
                       kb-pack build [DIR] [--out FILE] [--pack-id ID] [--pack-version V]
                       kb-pack new ID [--mode function|program|sql] [DIR]
                       kb-pack new-topic SUBJECT ID [LEARN_DIR]
                       kb-pack preview PROBLEM_DIR [--out DIR]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("validate") => validate_cmd(args),
        Some("build") => build_cmd(args),
        Some("generate") => generate_cmd(args),
        Some("profile") => profile_cmd(args),
        Some("new") => new_cmd(args),
        Some("new-topic") => new_topic_cmd(args),
        Some("preview") => preview_cmd(args),
        _ => usage(""),
    }
}

fn usage(problem: &str) -> ExitCode {
    if !problem.is_empty() {
        eprintln!("{problem}");
    }
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn new_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    use kb_pack::scaffold::{NewMode, new_problem};
    let mut id: Option<String> = None;
    let mut dir = PathBuf::from("content/problems");
    let mut mode = NewMode::Function;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--mode" => match args.next().as_deref() {
                Some("function") => mode = NewMode::Function,
                Some("program") => mode = NewMode::Program,
                Some("sql") => mode = NewMode::Sql,
                _ => return usage("--mode is function, program or sql"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ if id.is_none() => id = Some(a),
            _ => dir = a.into(),
        }
    }
    let Some(id) = id else {
        return usage("new needs an id");
    };
    match new_problem(&dir, &id, mode) {
        Ok(files) => {
            println!("wrote {} files in {}", files.len(), dir.join(&id).display());
            println!(
                "next: fill every TODO(kb-pack), then `kb-pack generate {0} --seeds 1,2,…`, `kb-pack validate --run --problems {id}` and `kb-pack preview {0}`",
                dir.join(&id).display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => usage(&format!("kb-pack new: {e}")),
    }
}

fn new_topic_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let (Some(subject), Some(id)) = (args.next(), args.next()) else {
        return usage("new-topic needs a subject and an id");
    };
    let learn = args
        .next()
        .map_or_else(|| PathBuf::from("content/learn"), PathBuf::from);
    match kb_pack::scaffold::new_topic(&learn, &subject, &id) {
        Ok(files) => {
            println!(
                "wrote {} files in {}",
                files.len(),
                learn.join(&subject).join(&id).display()
            );
            println!(
                "next: fill every TODO(kb-pack); add the tracer apps/desktop/src/learn/tracers/{id}.ts and its narration keys (en.json, es.json); `pnpm learn:cases`; `kb-pack validate --learn --run --topics {id}`"
            );
            ExitCode::SUCCESS
        }
        Err(e) => usage(&format!("kb-pack new-topic: {e}")),
    }
}

fn preview_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut problem: Option<PathBuf> = None;
    let mut out = PathBuf::from("target/preview");
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => match args.next() {
                Some(v) => out = v.into(),
                None => return usage("--out needs a folder"),
            },
            _ => problem = Some(a.into()),
        }
    }
    let Some(problem) = problem else {
        return usage("preview needs a problem folder");
    };
    match kb_pack::scaffold::preview(&problem, &out) {
        Ok(path) => {
            println!("wrote {}", path.display());
            ExitCode::SUCCESS
        }
        Err(e) => usage(&format!("kb-pack preview: {e}")),
    }
}

fn validate_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir = PathBuf::from("content/problems");
    let mut run_root: Option<PathBuf> = None;
    let mut run = false;
    let mut learn_only = false;
    let mut filter = kb_pack::TopicFilter::default();
    let mut only: Vec<String> = Vec::new();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--run" => run = true,
            "--problems" => match args.next() {
                Some(v) => only.extend(
                    v.split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                ),
                None => return usage("--problems needs a value"),
            },
            "--from" => match args.next().map(std::fs::read_to_string) {
                Some(Ok(text)) => only.extend(
                    text.lines()
                        .map(|l| l.trim().to_string())
                        .filter(|l| !l.is_empty() && !l.starts_with('#')),
                ),
                Some(Err(e)) => return usage(&format!("--from: {e}")),
                None => return usage("--from needs a file"),
            },
            "--learn" => learn_only = true,
            "--topics" => match args.next() {
                Some(v) => {
                    filter.topics = v
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                }
                None => return usage("--topics needs a value"),
            },
            "--subject" => match args.next() {
                Some(v) => filter.subject = Some(v),
                None => return usage("--subject needs a value"),
            },
            "--root" => match args.next() {
                Some(r) => run_root = Some(r.into()),
                None => return usage("--root needs a value"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ => dir = a.into(),
        }
    }
    let root = run_root.unwrap_or_else(|| "target/katabox".into());
    if (!filter.topics.is_empty() || filter.subject.is_some()) && !learn_only {
        return usage("--topics and --subject need --learn");
    }
    if !only.is_empty() && (learn_only || !run) {
        return usage("--problems and --from need --run (and not --learn)");
    }
    if learn_only {
        return validate_learn_only(&dir, run.then_some(root.as_path()), &filter);
    }
    let code = validate_all(&dir);
    if code != ExitCode::SUCCESS || !run {
        return code;
    }
    run_references(&dir, &root, &only)
}

fn generate_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir: Option<PathBuf> = None;
    let mut size = "max".to_string();
    let mut seeds: Vec<u64> = Vec::new();
    let mut first: Option<u32> = None;
    let mut root = PathBuf::from("target/katabox");
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        let set = match a.as_str() {
            "--size" => value("--size").map(|v| size = v),
            "--root" => value("--root").map(|v| root = v.into()),
            "--from" => value("--from").and_then(|v| {
                v.parse()
                    .map(|n| first = Some(n))
                    .map_err(|_| "--from needs a number".to_string())
            }),
            "--seeds" => value("--seeds").and_then(|v| {
                v.split(',')
                    .map(|s| s.trim().parse::<u64>().map_err(|_| format!("bad seed {s}")))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|s| seeds = s)
            }),
            s if s.starts_with("--") => Err(format!("unknown option {s}")),
            _ => {
                dir = Some(a.into());
                Ok(())
            }
        };
        if let Err(e) = set {
            return usage(&e);
        }
    }
    let Some(dir) = dir else {
        return usage("generate needs a problem folder");
    };
    if seeds.is_empty() {
        return usage("generate needs --seeds");
    }
    // Default: after the last numbered hidden test.
    let first = first.unwrap_or_else(|| {
        std::fs::read_dir(dir.join("tests/hidden"))
            .map(|rd| {
                rd.flatten()
                    .filter_map(|e| {
                        e.path()
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .and_then(|s| s.parse::<u32>().ok())
                    })
                    .max()
                    .unwrap_or(0)
                    + 1
            })
            .unwrap_or(1)
    });
    let env = match kb_pack::env_for(&root) {
        Ok(e) => e,
        Err(e) => return usage(&e),
    };
    match kb_pack::generate_tests(&dir, &env, &size, &seeds, first) {
        Ok(files) => {
            for f in files {
                let bytes = std::fs::metadata(&f).map(|m| m.len()).unwrap_or(0);
                println!("wrote {} ({bytes} bytes)", f.display());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack generate: {e}");
            ExitCode::FAILURE
        }
    }
}

fn profile_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut dir: Option<PathBuf> = None;
    let mut root = PathBuf::from("target/katabox");
    let mut inputs_only = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--inputs" => inputs_only = true,
            "--root" => match args.next() {
                Some(v) => root = v.into(),
                None => return usage("--root needs a value"),
            },
            s if s.starts_with("--") => return usage(&format!("unknown option {s}")),
            _ => dir = Some(a.into()),
        }
    }
    if inputs_only {
        let dir = dir.unwrap_or_else(|| "content/problems".into());
        let env = match kb_pack::env_for(&root) {
            Ok(e) => e,
            Err(e) => return usage(&e),
        };
        return match kb_pack::profile::fill_caches(&dir, &env) {
            Ok(lines) => {
                for l in lines {
                    println!("{l}");
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("kb-pack profile --inputs: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let Some(dir) = dir else {
        return usage("profile needs a problem folder");
    };
    let env = match kb_pack::env_for(&root) {
        Ok(e) => e,
        Err(e) => return usage(&e),
    };
    match kb_pack::profile::profile_problem(&dir, &env) {
        Ok(lines) => {
            for l in lines {
                println!("{l}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack profile: {e}");
            ExitCode::FAILURE
        }
    }
}

fn build_cmd(mut args: impl Iterator<Item = String>) -> ExitCode {
    let mut opts = BuildOptions {
        dir: "content/problems".into(),
        out: "target/content.db".into(),
        pack_id: "core".into(),
        pack_version: env!("CARGO_PKG_VERSION").into(),
    };
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        let set = match a.as_str() {
            "--out" => value("--out").map(|v| opts.out = v.into()),
            "--pack-id" => value("--pack-id").map(|v| opts.pack_id = v),
            "--pack-version" => value("--pack-version").map(|v| opts.pack_version = v),
            s if s.starts_with("--") => Err(format!("unknown option {s}")),
            _ => {
                opts.dir = a.into();
                Ok(())
            }
        };
        if let Err(e) = set {
            return usage(&e);
        }
    }
    match build(&opts) {
        Ok(n) => {
            println!("built {} ({n} problem(s))", opts.out.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kb-pack build: {e}");
            ExitCode::FAILURE
        }
    }
}
