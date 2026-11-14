//! Program problems (ADR 0028) in the Windows sandbox, in every language: Accepted, Wrong
//! Answer with the mismatch, Runtime Error with stderr, the time and output limits, a
//! syntax or compile error, UTF-8 through stdin and stdout, and the starters. Also
//! measures each runtime's startup per test run. Needs the dev sandbox root:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-judge --release --test program_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::time::Instant;

use kb_core::{Language, MessageCode, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, ProblemMode, judge, program_starter};
use kb_runner::ROOT_MARKER;
use serde_json::{Map, Value};

const LANGS: [Language; 4] = [
    Language::Python,
    Language::TypeScript,
    Language::Php,
    Language::Rust,
];

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    let root = std::path::absolute(root).unwrap();
    assert!(
        root.join(ROOT_MARKER).is_file(),
        "sandbox root missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    JudgeEnv::dev(root)
}

fn test(index: u32, stdin: &str, stdout: &str) -> TestCase {
    let mut input = Map::new();
    input.insert("stdin".into(), Value::String(stdin.into()));
    TestCase {
        kind: TestKind::Public,
        index,
        input,
        expected: Value::String(stdout.into()),
    }
}

fn problem(checker: &str, tests: Vec<TestCase>) -> Problem {
    Problem::from_parts(
        "program-e2e",
        ProblemMode::Program,
        &Value::Null,
        checker,
        300,
        256,
        tests,
    )
    .unwrap()
}

/// Sum every integer on stdin.
fn sum_program(lang: Language, plus: i64) -> String {
    match lang {
        Language::Python => format!(
            "import sys\nprint(sum(int(t) for t in sys.stdin.buffer.read().split()) + {plus})\n"
        ),
        Language::TypeScript => format!(
            "import {{ readFileSync }} from \"node:fs\";\nconst t = readFileSync(0, \"utf8\").split(/\\s+/).filter((x) => x.length > 0).map(Number);\nconsole.log(t.reduce((a, b) => a + b, 0) + {plus});\n"
        ),
        Language::Php => format!(
            "<?php\n$t = preg_split('/\\s+/', trim(stream_get_contents(STDIN)));\n$s = 0;\nforeach ($t as $x) {{ if ($x !== '') $s += (int) $x; }}\necho $s + {plus}, \"\\n\";\n"
        ),
        _ => format!(
            "use std::io::{{self, Read}};\nfn main() {{\n    let mut s = String::new();\n    io::stdin().read_to_string(&mut s).unwrap();\n    let t: i64 = s.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).sum();\n    println!(\"{{}}\", t + {plus});\n}}\n"
        ),
    }
}

fn run(lang: Language, src: &str, p: &Problem) -> kb_core::Report {
    judge(p, &p.tests, lang, src, &env()).unwrap()
}

#[test]
fn accepted_and_wrong_answer_with_the_mismatch() {
    let p = problem(
        "tokens",
        vec![test(1, "3\n1 2 3\n", "9\n"), test(2, "1\n-5", "-4")],
    );
    for lang in LANGS {
        let r = run(lang, &sum_program(lang, 0), &p);
        assert_eq!(r.verdict, Verdict::Accepted, "{lang}: {r:#?}");
        assert_eq!(r.tests.len(), 2);
        assert_eq!(
            r.tests[0].actual,
            Some(Value::String("9\n".into())),
            "{lang}"
        );
        assert!(r.tests[0].result.time_us > 0);
        let r = run(lang, &sum_program(lang, 1), &p);
        assert_eq!(r.verdict, Verdict::WrongAnswer, "{lang}");
        assert_eq!(
            r.tests.len(),
            2,
            "{lang}: a wrong answer does not stop the batch"
        );
        let f = r.first_failure.unwrap();
        let m = f.message.unwrap();
        assert_eq!(m.code, MessageCode::OutputMismatch);
        assert_eq!(m.text, "line 1, token 1: expected `9`, got `10`", "{lang}");
        assert_eq!(f.input["stdin"], "3\n1 2 3\n");
        assert_eq!(f.actual, Some(Value::String("10\n".into())));
    }
}

#[test]
fn runtime_error_keeps_stderr_apart() {
    let p = problem("tokens", vec![test(1, "1", "1")]);
    for lang in LANGS {
        let src = match lang {
            Language::Python => {
                "import sys\nprint('partial')\nprint('debug line', file=sys.stderr)\nraise ValueError('boom')\n"
            }
            Language::TypeScript => {
                "console.log(\"partial\");\nconsole.error(\"debug line\");\nthrow new Error(\"boom\");\n"
            }
            Language::Php => {
                "<?php\necho \"partial\\n\";\nfwrite(STDERR, \"debug line\\n\");\nthrow new Exception(\"boom\");\n"
            }
            _ => {
                "fn main() {\n    println!(\"partial\");\n    eprintln!(\"debug line\");\n    panic!(\"boom\");\n}\n"
            }
        };
        let r = run(lang, src, &p);
        assert_eq!(r.verdict, Verdict::RuntimeError, "{lang}: {r:#?}");
        let t = &r.tests[0];
        let m = t.result.message.as_ref().unwrap();
        assert_eq!(m.code, MessageCode::ProcessExited);
        assert!(m.text.contains("boom"), "{lang}: {}", m.text);
        assert!(
            !m.text.contains("judge-"),
            "{lang}: work folder stripped: {}",
            m.text
        );
        assert_eq!(t.stdout.trim(), "partial", "{lang}");
        assert!(
            t.stderr.as_deref().unwrap().contains("debug line"),
            "{lang}"
        );
        assert!(
            r.first_failure
                .unwrap()
                .stderr
                .unwrap()
                .contains("debug line")
        );
    }
}

#[test]
fn time_limit_stops_the_batch() {
    let p = problem("tokens", vec![test(1, "1", "1"), test(2, "2", "2")]);
    for lang in LANGS {
        let src = match lang {
            Language::Python => "while True:\n    pass\n",
            Language::TypeScript => "let x = 0;\nfor (;;) { x++; }\n",
            Language::Php => "<?php\n$x = 0;\nwhile (true) { $x++; }\n",
            _ => {
                "fn main() {\n    let mut x = 0u64;\n    loop { x = std::hint::black_box(x.wrapping_add(1)); }\n}\n"
            }
        };
        let r = run(lang, src, &p);
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded, "{lang}");
        assert_eq!(r.tests.len(), 1, "{lang}: the batch stops at a time limit");
        assert_eq!(
            r.tests[0].result.message.as_ref().unwrap().code,
            MessageCode::TimeLimit
        );
    }
}

#[test]
fn output_limit() {
    let p = problem("tokens", vec![test(1, "1", "1")]);
    for lang in LANGS {
        let src = match lang {
            Language::Python => "import sys\nsys.stdout.write('x' * (9 * 1024 * 1024))\n",
            Language::TypeScript => "process.stdout.write(\"x\".repeat(9 * 1024 * 1024));\n",
            Language::Php => "<?php\necho str_repeat(\"x\", 9 * 1024 * 1024);\n",
            _ => {
                "use std::io::Write;\nfn main() {\n    let s = vec![b'x'; 9 * 1024 * 1024];\n    std::io::stdout().write_all(&s).unwrap();\n}\n"
            }
        };
        let r = run(lang, src, &p);
        assert_eq!(
            r.verdict,
            Verdict::OutputLimitExceeded,
            "{lang}: {:?}",
            r.tests[0].result
        );
        assert_eq!(
            r.tests[0].result.message.as_ref().unwrap().code,
            MessageCode::OutputLimit
        );
    }
}

#[test]
fn syntax_and_compile_errors_are_compilation_errors() {
    let p = problem("tokens", vec![test(1, "1", "1")]);
    for lang in LANGS {
        let src = match lang {
            Language::Python => "def broken(:\n    pass\n",
            Language::TypeScript => "const = ;\n",
            Language::Php => "<?php\n$x = ;\n",
            _ => "fn main( {\n}\n",
        };
        let r = run(lang, src, &p);
        assert_eq!(r.verdict, Verdict::CompilationError, "{lang}: {r:#?}");
        let m = r.compile_output.unwrap();
        assert!(!m.text.is_empty(), "{lang}");
        assert!(
            !m.text.contains("judge-"),
            "{lang}: work folder stripped: {}",
            m.text
        );
        assert!(r.tests.is_empty());
    }
}

/// The TypeScript syntax check treats the file as the run does: types stripped, an ES
/// module (the editor's JavaScript mode saves plain JS into the same `.ts` file).
#[test]
fn typescript_check_matches_the_run() {
    let p = problem("tokens", vec![test(1, "", "6")]);
    let ts = Language::TypeScript;
    let accepted = [
        // Annotations and no leading import: once a false compilation error.
        "function total(a: number[]): number {\n  return a.reduce((s: number, x: number) => s + x, 0);\n}\nconst xs: number[] = [1, 2, 3];\nconsole.log(total(xs));\n",
        "interface P { x: number }\ntype Q = P | null;\nconst p: Q = { x: 6 };\nconsole.log(p!.x as number);\n",
        // Plain JavaScript (the JS editor mode): no imports, then an import and top-level await.
        "function total(a) {\n  return a.reduce((s, x) => s + x, 0);\n}\nconsole.log(total([1, 2, 3]));\n",
        "import { readFileSync } from \"node:fs\";\nconst n = await Promise.resolve(6);\nconsole.log(readFileSync(0, \"utf8\") + n);\n",
    ];
    for src in accepted {
        let r = run(ts, src, &p);
        assert_eq!(r.verdict, Verdict::Accepted, "{src}\n{r:#?}");
    }
    // CommonJS-looking code passes the check and fails at run time, as the run does
    // (an ES module has no `require`).
    let r = run(
        ts,
        "const fs = require(\"node:fs\");\nconst n: number = 6;\nconsole.log(n);\n",
        &p,
    );
    assert_eq!(r.verdict, Verdict::RuntimeError, "{r:#?}");
    // Real syntax errors stay compilation errors, with the file and line.
    let errors = [
        (
            "const a = 1;\nfunction f(a: number[] {\n}\n",
            "solution.ts:2",
        ),
        (
            "const x = 1;\nconst y = 2;\nconst x = 3;\n",
            "solution.ts:3",
        ),
        ("enum Color { Red }\nconsole.log(Color.Red);\n", "enum"),
        ("const n = 1;\nreturn n;\n", "solution.ts:2"),
    ];
    for (src, needle) in errors {
        let r = run(ts, src, &p);
        assert_eq!(r.verdict, Verdict::CompilationError, "{src}\n{r:#?}");
        let m = r.compile_output.unwrap();
        assert_eq!(m.code, MessageCode::SyntaxError, "{src}");
        assert!(m.text.contains(needle), "{src}: {}", m.text);
        assert!(
            !m.text.contains("judge-"),
            "work folder stripped: {}",
            m.text
        );
    }
}

#[test]
fn utf8_round_trips_through_stdin_and_stdout() {
    let text = "Café con ñandú\nAÑO 2026 — «hola»\n";
    let p = problem("lines", vec![test(1, text, text)]);
    for lang in LANGS {
        let src = match lang {
            Language::Python => "import sys\nsys.stdout.write(sys.stdin.read())\n",
            Language::TypeScript => {
                "import { readFileSync } from \"node:fs\";\nprocess.stdout.write(readFileSync(0, \"utf8\"));\n"
            }
            Language::Php => "<?php\necho stream_get_contents(STDIN);\n",
            _ => {
                "use std::io::{self, Read};\nfn main() {\n    let mut s = String::new();\n    io::stdin().read_to_string(&mut s).unwrap();\n    print!(\"{s}\");\n}\n"
            }
        };
        let r = run(lang, src, &p);
        assert_eq!(r.verdict, Verdict::Accepted, "{lang}: {:?}", r.tests[0]);
        assert_eq!(
            r.tests[0].actual,
            Some(Value::String(text.into())),
            "{lang}"
        );
    }
}

#[test]
fn starters_read_stdin_and_run_cleanly() {
    // The starters print one empty line: right for an empty expected output.
    let p = problem("tokens", vec![test(1, "5\n1 2 3 4 5\n", "")]);
    for lang in LANGS {
        let src = program_starter(lang).unwrap();
        let r = run(lang, &src, &p);
        assert_eq!(r.verdict, Verdict::Accepted, "{lang}: {r:#?}");
    }
}

/// Each runtime's cost per test run, which program problems pay once per test: the
/// median CPU (what the limit measures) and wall time of 20 runs of a trivial program.
#[test]
fn startup_cost_per_run() {
    let tests: Vec<TestCase> = (1..=20).map(|i| test(i, "1 2", "3")).collect();
    let p = problem("tokens", tests);
    for lang in LANGS {
        let src = sum_program(lang, 0);
        let _ = run(lang, &src, &p); // warm the binary cache and the file system
        let start = Instant::now();
        let r = run(lang, &src, &p);
        let wall_ms = start.elapsed().as_secs_f64() * 1000.0 / 20.0;
        assert_eq!(r.verdict, Verdict::Accepted, "{lang}");
        let mut cpu: Vec<u64> = r.tests.iter().map(|t| t.result.time_us).collect();
        cpu.sort_unstable();
        let def = kb_judge::languages::definition(lang).unwrap();
        println!(
            "{lang}: startup CPU median {:.1} ms (min {:.1}, max {:.1}); wall per test run {wall_ms:.0} ms; allowance {} ms",
            cpu[10] as f64 / 1000.0,
            cpu[0] as f64 / 1000.0,
            cpu[19] as f64 / 1000.0,
            def.time.startup_ms
        );
    }
}
