//! The Learn correctness guarantee (ADR 0029): every topic's sample.py and sample.ts run
//! in the sandbox, through the program-mode judge, on every case in cases.json (each
//! preset plus 20 seeded random inputs, or every n for a count input). Each output must
//! equal what the tracer recorded, and for sorting topics that must equal a plain sort.
//! `KB_LEARN_TOPICS=id,id` limits the run to those topics (an unknown id fails). Needs the
//! dev sandbox root:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1
//! cargo test -p kb-pack --test learn_samples_windows -- --nocapture
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};

use kb_core::Language;
use kb_pack::env_for;
use kb_pack::learn::{SAMPLE_LANGUAGES, TopicFilter, is_sort_of, read_learn, run_sample};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn env() -> kb_judge::JudgeEnv {
    let root = std::env::var_os("KB_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo().join("target/katabox"));
    assert!(
        root.join("toolchains/python-3.14/python.exe").is_file(),
        "dev sandbox root missing at {}. Run: pwsh scripts/fetch-toolchains.ps1",
        root.display()
    );
    env_for(&root).unwrap()
}

fn topics() -> Vec<kb_pack::learn::TopicDir> {
    read_learn(
        &repo().join("content/learn"),
        &repo().join("content/problems"),
    )
    .unwrap_or_else(|e| panic!("{e:#?}"))
    .topics
}

#[test]
fn every_sample_matches_its_tracer_and_a_plain_sort() {
    let env = env();
    let all = topics();
    let filter = TopicFilter {
        topics: std::env::var("KB_LEARN_TOPICS")
            .map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default(),
        subject: None,
        sets: vec![],
    };
    let topics = filter
        .select(&all)
        .unwrap_or_else(|e| panic!("KB_LEARN_TOPICS: {e}"));
    if filter.topics.is_empty() {
        assert!(topics.len() >= 3, "at least the Phase 0 topics");
    } else {
        assert_eq!(topics.len(), filter.topics.len());
    }
    for t in topics {
        let lim = t.topic.input;
        if lim.max_len == 1 {
            assert_eq!(
                t.cases.len() as i64,
                (lim.max - lim.min + 1) as i64,
                "{}: every n",
                t.topic.id
            );
        } else {
            // At least one preset plus the 20 random inputs.
            assert!(t.cases.len() >= 21, "{}: presets + 20 random", t.topic.id);
        }
        if t.sorts {
            for c in &t.cases {
                assert!(is_sort_of(&c.output, &c.input), "{} {}", t.topic.id, c.name);
            }
        }
        for language in SAMPLE_LANGUAGES {
            let started = std::time::Instant::now();
            run_sample(t, language, &env)
                .unwrap_or_else(|e| panic!("{} sample.{}: {e}", t.topic.id, language.extension()));
            println!(
                "{} sample.{}: {} cases ok in {:.1} s",
                t.topic.id,
                language.extension(),
                t.cases.len(),
                started.elapsed().as_secs_f64()
            );
        }
    }
}

#[test]
fn a_sample_that_disagrees_with_the_tracer_is_caught() {
    let env = env();
    let mut t = topics()
        .into_iter()
        .find(|t| t.topic.id == "quick-sort")
        .unwrap();
    // Sort descending instead: still a valid program, wrong order.
    for (language, source) in &mut t.topic.samples {
        let (from, to) = match language {
            Language::Python => ("if a[j] < pivot:", "if a[j] > pivot:"),
            _ => ("if (a[j] < pivot) {", "if (a[j] > pivot) {"),
        };
        assert!(source.contains(from));
        *source = source.replace(from, to);
    }
    for language in SAMPLE_LANGUAGES {
        let err = run_sample(&t, language, &env).unwrap_err();
        assert!(err.starts_with("WrongAnswer"), "{}: {err}", language.id());
    }
}
